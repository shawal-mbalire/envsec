use dialoguer::Password;

use crate::config;
use crate::output::colors;
use crate::session;
use crate::vault;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let dir = config::config_dir();
    if dir.exists() && config::vault_path().exists() {
        println!(
            "{}",
            colors::warning("Vault already initialized. Delete ~/.envsec/ to reinitialize.")
        );
        return Ok(());
    }

    config::init_config_dir()?;

    println!("{}", colors::header("Initializing envsec vault"));

    let passphrase = Password::new()
        .with_prompt("Create master passphrase")
        .with_confirmation("Confirm passphrase", "Passphrases do not match")
        .interact()?;

    if passphrase.len() < 8 {
        return Err("Passphrase must be at least 8 characters".into());
    }

    // Create empty vault
    let vault_data = vault::types::VaultData::default();
    let vault_path = config::vault_path();
    vault::store::save_vault(&vault_path, passphrase.as_bytes(), &vault_data)?;

    // Create initial session
    let config = config::load_config();
    session::create_session(passphrase.as_bytes(), config.session.duration_secs)?;

    println!();
    println!("{}", colors::success("Vault initialized successfully"));
    println!(
        "  {} {}",
        colors::dim("Vault:"),
        vault_path.display()
    );
    println!(
        "  {} {}",
        colors::dim("Session:"),
        colors::success("active")
    );
    println!();
    println!(
        "{}",
        colors::dim("Use 'envsec use <project> <environment>' to set your active project.")
    );
    println!(
        "{}",
        colors::dim("Use 'envsec set KEY VALUE' to add secrets.")
    );

    Ok(())
}
