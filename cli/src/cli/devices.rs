use crate::config;
use crate::output::{colors, table};
use crate::project::resolver;
use crate::session;
use crate::sync::{derive_room_id, SyncClient};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let session = session::check_session().map_err(|e| {
        format!(
            "{}. Run {} to authenticate.",
            e,
            colors::bold("envsec auth")
        )
    })?;

    let cfg = config::load_config();

    let (proj, env) = match resolver::load_current_project() {
        Ok(p) => (p.project, p.environment),
        Err(_) => {
            println!(
                "{}",
                colors::dim("No project set. Use 'envsec use <project> <environment>' first.")
            );
            return Ok(());
        }
    };

    // Derive room ID from passphrase hash + project + environment
    let room = derive_room_id(&session.passphrase_hash, &proj, &env);

    let client = SyncClient::new(
        &cfg.sync.server_url,
        &room,
        &cfg.sync.device_id,
        &cfg.sync.device_name,
    );

    println!(
        "{} Fetching online devices for {} / {}...",
        colors::dim("*"),
        colors::key_name(&proj),
        colors::key_name(&env)
    );
    println!(
        "{} {}",
        colors::dim("Room:"),
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

    table::print_table(&rows, Some(&format!("Online devices")));
    println!(
        "{}",
        colors::dim("Only devices with the same passphrase can see each other")
    );
    println!();

    Ok(())
}
