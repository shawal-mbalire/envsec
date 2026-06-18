use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SignalMessage {
    #[serde(rename = "offer")]
    Offer {
        from: String,
        to: String,
        payload: serde_json::Value,
    },
    #[serde(rename = "answer")]
    Answer {
        from: String,
        to: String,
        payload: serde_json::Value,
    },
    #[serde(rename = "ice-candidate")]
    IceCandidate {
        from: String,
        to: String,
        payload: serde_json::Value,
    },
    #[serde(rename = "secret-updated")]
    SecretUpdated {
        from: String,
        payload: SecretUpdate,
    },
    #[serde(rename = "sync-request")]
    SyncRequest {
        from: String,
        payload: SyncRequestPayload,
    },
    #[serde(rename = "sync-response")]
    SyncResponse {
        from: String,
        to: String,
        payload: SyncResponsePayload,
    },
    #[serde(rename = "presence")]
    Presence {
        from: String,
        payload: PresencePayload,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretUpdate {
    pub project: String,
    pub environment: String,
    pub key: String,
    pub action: SecretAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SecretAction {
    Set,
    Remove,
    Rename { new_key: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRequestPayload {
    pub requested_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResponsePayload {
    pub project: String,
    pub environment: String,
    pub secrets: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresencePayload {
    pub action: String,
    pub devices: Vec<Device>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub joined_at: u64,
}
