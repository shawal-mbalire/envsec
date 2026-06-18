use crate::clipboard;
use crate::config;
use crate::output::{colors, masked};
use crate::project::resolver;
use crate::session;
use crate::vault;

pub fn run(key: &str, show: bool) -> Result<(), Box<dyn std::error::Error>> {
    let session = session::check_session().map_err(|e| {
        format!(
            "{}. Run {} to authenticate.",
            e,
            colors::bold("envsec auth")
        )
    })?;

    let project_config = resolver::load_current_project().map_err(|e| {
        format!(
            "{}. Run {} to set a project.",
            e,
            colors::bold("envsec use <project> <environment>")
        )
    })?;

    let vault_path = config::vault_path();
    let vault = vault::Vault::load(&vault_path, session.passphrase_hash.as_bytes())?;

    let secret = vault
        .data()
        .get_secret(&project_config.project, &project_config.environment, key)
        .ok_or_else(|| format!("Secret '{}' not found", key))?;

    if show {
        println!(
            "{} = {}",
            colors::key_name(key),
            colors::value_masked(&masked::mask_value(&secret.value))
        );
    } else {
        let cfg = config::load_config();
        clipboard::copy_to_clipboard(&secret.value)?;
        clipboard::fork_clear_after(cfg.clipboard.clear_after_secs)?;

        println!(
            "{} Copied '{}' to clipboard. Will clear in {} seconds.",
            colors::success("OK"),
            colors::key_name(key),
            cfg.clipboard.clear_after_secs
        );
    }

    Ok(())
}
