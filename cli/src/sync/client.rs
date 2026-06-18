use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use super::messages::*;

#[derive(Debug, thiserror::Error)]
pub enum SyncError {
    #[error("WebSocket error: {0}")]
    WebSocket(String),
    #[error("connection closed")]
    ConnectionClosed,
    #[error("no response from server")]
    NoResponse,
    #[error("invalid message: {0}")]
    InvalidMessage(String),
}

pub struct SyncClient {
    server_url: String,
    room: String,
    device_id: String,
    device_name: String,
}

impl SyncClient {
    pub fn new(server_url: &str, room: &str, device_id: &str, device_name: &str) -> Self {
        Self {
            server_url: server_url.to_string(),
            room: room.to_string(),
            device_id: device_id.to_string(),
            device_name: device_name.to_string(),
        }
    }

    pub async fn list_devices(&self) -> Result<Vec<Device>, SyncError> {
        let http_url = self
            .server_url
            .replace("wss://", "https://")
            .replace("ws://", "http://");
        let url = format!("{}/api/rooms?room={}", http_url, self.room);

        let body = ureq::get(&url)
            .call()
            .map_err(|e| SyncError::WebSocket(e.to_string()))?
            .into_string()
            .map_err(|e| SyncError::WebSocket(e.to_string()))?;

        let response: serde_json::Value =
            serde_json::from_str(&body).map_err(|e| SyncError::InvalidMessage(e.to_string()))?;

        let devices = response["devices"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|d| serde_json::from_value(d.clone()).ok())
                    .collect()
            })
            .unwrap_or_default();

        Ok(devices)
    }

    pub async fn connect(
        &self,
    ) -> Result<(mpsc::Sender<SignalMessage>, mpsc::Receiver<SignalMessage>), SyncError> {
        let ws_url = format!(
            "{}?room={}&device={}&name={}",
            self.server_url, self.room, self.device_id, self.device_name
        );

        let (ws_stream, _) = connect_async(&ws_url)
            .await
            .map_err(|e| SyncError::WebSocket(e.to_string()))?;

        let (mut write, mut read) = ws_stream.split();
        let (tx_out, mut rx_out) = mpsc::channel::<SignalMessage>(32);
        let (tx_in, rx_in) = mpsc::channel::<SignalMessage>(32);

        // Spawn task to handle outgoing messages
        tokio::spawn(async move {
            while let Some(msg) = rx_out.recv().await {
                if let Ok(json) = serde_json::to_string(&msg) {
                    if write.send(Message::Text(json.into())).await.is_err() {
                        break;
                    }
                }
            }
        });

        // Spawn task to handle incoming messages
        tokio::spawn(async move {
            while let Some(result) = read.next().await {
                match result {
                    Ok(Message::Text(text)) => {
                        if let Ok(msg) = serde_json::from_str::<SignalMessage>(&text) {
                            if tx_in.send(msg).await.is_err() {
                                break;
                            }
                        }
                    }
                    Ok(Message::Close(_)) => break,
                    Err(_) => break,
                    _ => {}
                }
            }
        });

        Ok((tx_out, rx_in))
    }

    pub async fn sync_secrets(
        &self,
        project: &str,
        environment: &str,
        secrets: &HashMap<String, String>,
    ) -> Result<(), SyncError> {
        let (tx, _rx) = self.connect().await?;

        // Send sync response with our secrets
        let msg = SignalMessage::SyncResponse {
            from: self.device_id.clone(),
            to: String::new(), // broadcast
            payload: SyncResponsePayload {
                project: project.to_string(),
                environment: environment.to_string(),
                secrets: secrets.clone(),
            },
        };
        tx.send(msg)
            .await
            .map_err(|e| SyncError::WebSocket(e.to_string()))?;

        // Wait briefly for any responses
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        Ok(())
    }

    pub async fn notify_secret_updated(
        &self,
        project: &str,
        environment: &str,
        key: &str,
        action: SecretAction,
    ) -> Result<(), SyncError> {
        let (tx, _rx) = self.connect().await?;

        let msg = SignalMessage::SecretUpdated {
            from: self.device_id.clone(),
            payload: SecretUpdate {
                project: project.to_string(),
                environment: environment.to_string(),
                key: key.to_string(),
                action,
            },
        };
        tx.send(msg)
            .await
            .map_err(|e| SyncError::WebSocket(e.to_string()))?;

        Ok(())
    }
}
