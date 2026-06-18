use dialoguer::Password;

use crate::config;
use crate::output::colors;
use crate::session;

pub fn run(duration: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let mut cfg = config::load_config();

    if let Some(dur) = duration {
        config::set_session_duration(&mut cfg, dur)?;
        config::save_config(&cfg)?;
    }

    let passphrase = Password::new()
        .with_prompt("Master passphrase")
        .interact()?;

    let session = session::create_session(passphrase.as_bytes(), cfg.session.duration_secs)?;

    let hours = session.duration_secs / 3600;
    let mins = (session.duration_secs % 3600) / 60;
    let duration_str = if hours > 0 {
        format!("{}h{}m", hours, mins)
    } else {
        format!("{}m", mins)
    };

    println!();
    println!(
        "{}  {} (expires in {})",
        colors::success("Authenticated."),
        colors::dim(&format!(
            "Session valid until {}",
            session.expires_at.format("%Y-%m-%d %H:%M:%S UTC")
        )),
        duration_str
    );

    Ok(())
}
