use std::collections::HashMap;

use crate::config;
use crate::output::colors;
use crate::project::resolver;
use crate::session;
use crate::sync::SyncClient;
use crate::vault;

pub async fn run(
    project: Option<&str>,
    environment: Option<&str>,
    push: bool,
    pull: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let session = session::check_session().map_err(|e| {
        format!(
            "{}. Run {} to authenticate.",
            e,
            colors::bold("envsec auth")
        )
    })?;

    let cfg = config::load_config();

    let (proj, env) = match (project, environment) {
        (Some(p), Some(e)) => (p.to_string(), e.to_string()),
        _ => {
            let pc = resolver::load_current_project().map_err(|e| {
                format!(
                    "{}. Run {} to set a project.",
                    e,
                    colors::bold("envsec use <project> <environment>")
                )
            })?;
            (
                project.unwrap_or(&pc.project).to_string(),
                environment.unwrap_or(&pc.environment).to_string(),
            )
        }
    };

    let room = format!("{}-{}", proj, env);

    let client = SyncClient::new(
        &cfg.sync.server_url,
        &room,
        &cfg.sync.device_id,
        &cfg.sync.device_name,
    );

    if push {
        return push_secrets(&session, &client, &cfg, &proj, &env).await;
    }

    if pull {
        return pull_secrets(&session, &client, &cfg, &proj, &env).await;
    }

    // Default: show sync status and list devices
    println!(
        "{} Sync status for {} / {}",
        colors::header("envsec sync"),
        colors::key_name(&proj),
        colors::key_name(&env)
    );
    println!();

    let devices = client.list_devices().await?;
    println!(
        "{} {} device(s) online",
        colors::dim("Devices:"),
        colors::bold(&devices.len().to_string())
    );

    for d in &devices {
        let marker = if d.id == cfg.sync.device_id {
            " (this device)"
        } else {
            ""
        };
        println!(
            "  {} {}",
            colors::success("*"),
            format!("{}{}", d.name, marker)
        );
    }

    println!();
    println!(
        "{}",
        colors::dim("Use --push to sync local secrets to peers")
    );
    println!(
        "{}",
        colors::dim("Use --pull to request secrets from peers")
    );

    Ok(())
}

async fn push_secrets(
    session: &crate::session::Session,
    client: &SyncClient,
    cfg: &config::Config,
    project: &str,
    environment: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let vault_path = config::vault_path();
    let vault = vault::Vault::load(&vault_path, session.passphrase_hash.as_bytes())?;

    let secrets: HashMap<String, String> = vault
        .data()
        .get_project(project)
        .and_then(|p| p.environments.get(environment))
        .map(|env| {
            env.secrets
                .iter()
                .map(|(k, s)| (k.clone(), s.value.clone()))
                .collect()
        })
        .unwrap_or_default();

    if secrets.is_empty() {
        println!("{}", colors::dim("No secrets to push."));
        return Ok(());
    }

    println!(
        "{} Pushing {} secrets to peers in room '{}'...",
        colors::dim("*"),
        colors::bold(&secrets.len().to_string()),
        colors::key_name(&format!("{}-{}", project, environment))
    );

    client.sync_secrets(project, environment, &secrets).await?;

    println!(
        "{} Secrets pushed. Peers will be notified.",
        colors::success("OK")
    );

    Ok(())
}

async fn pull_secrets(
    session: &crate::session::Session,
    client: &SyncClient,
    cfg: &config::Config,
    project: &str,
    environment: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let vault_path = config::vault_path();
    let mut vault = vault::Vault::load(&vault_path, session.passphrase_hash.as_bytes())?;

    println!(
        "{} Requesting secrets from peers in room '{}'...",
        colors::dim("*"),
        colors::key_name(&format!("{}-{}", project, environment))
    );

    let (tx, mut rx) = client.connect().await?;

    // Send sync request
    use crate::sync::SignalMessage;
    let msg = SignalMessage::SyncRequest {
        from: cfg.sync.device_id.clone(),
        payload: crate::sync::SyncRequestPayload {
            requested_by: cfg.sync.device_id.clone(),
        },
    };
    tx.send(msg).await?;

    // Wait for responses
    let timeout = tokio::time::Duration::from_secs(5);
    let start = tokio::time::Instant::now();
    let mut received = 0;

    while start.elapsed() < timeout {
        if let Ok(msg) = tokio::time::timeout(tokio::time::Duration::from_millis(500), rx.recv()).await {
            match msg {
                Some(SignalMessage::SyncResponse { payload, .. }) => {
                    if payload.project == project && payload.environment == environment {
                        for (key, value) in &payload.secrets {
                            vault.data_mut().set_secret(project, environment, key, value);
                            received += 1;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    if received > 0 {
        vault.save()?;
        println!(
            "{} Received and saved {} secrets from peers.",
            colors::success("OK"),
            colors::bold(&received.to_string())
        );
    } else {
        println!(
            "{} No secrets received from peers (they may not be online).",
            colors::warning("!")
        );
    }

    Ok(())
}
