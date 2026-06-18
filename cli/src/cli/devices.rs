use crate::config;
use crate::output::{colors, table};
use crate::project::resolver;
use crate::sync::SyncClient;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = config::load_config();

    let room = match resolver::load_current_project() {
        Ok(p) => format!("{}-{}", p.project, p.environment),
        Err(_) => "default".to_string(),
    };

    let client = SyncClient::new(
        &cfg.sync.server_url,
        &room,
        &cfg.sync.device_id,
        &cfg.sync.device_name,
    );

    println!(
        "{} Fetching online devices for room '{}'...",
        colors::dim("*"),
        colors::key_name(&room)
    );

    let devices = client.list_devices().await?;

    if devices.is_empty() {
        println!("{}", colors::dim("No devices online."));
        return Ok(());
    }

    let rows: Vec<table::StatusRow> = devices
        .iter()
        .map(|d| {
            let age = {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64;
                let secs = (now.saturating_sub(d.joined_at)) / 1000;
                if secs < 60 {
                    format!("{}s ago", secs)
                } else if secs < 3600 {
                    format!("{}m ago", secs / 60)
                } else {
                    format!("{}h ago", secs / 3600)
                }
            };

            table::StatusRow {
                field: if d.id == cfg.sync.device_id {
                    format!("{} (this device)", d.name)
                } else {
                    d.name.clone()
                },
                value: format!("online ({})", age),
            }
        })
        .collect();

    table::print_table(&rows, Some(&format!("Online devices ({})", room)));
    println!();

    Ok(())
}
