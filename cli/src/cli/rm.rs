use crate::config;
use crate::output::colors;
use crate::project::resolver;
use crate::session;
use crate::vault;

pub fn run_rm(key: &str) -> Result<(), Box<dyn std::error::Error>> {
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
    let mut vault = vault::Vault::load(&vault_path, session.passphrase_hash.as_bytes())?;

    let removed = vault.data_mut().remove_secret(
        &project_config.project,
        &project_config.environment,
        key,
    );

    if removed {
        vault.save()?;
        println!("{} '{}'", colors::success("Removed"), colors::key_name(key));
    } else {
        return Err(format!("Secret '{}' not found", key).into());
    }

    Ok(())
}

pub fn run_rename(old_key: &str, new_key: &str) -> Result<(), Box<dyn std::error::Error>> {
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
    let mut vault = vault::Vault::load(&vault_path, session.passphrase_hash.as_bytes())?;

    let renamed = vault.data_mut().rename_secret(
        &project_config.project,
        &project_config.environment,
        old_key,
        new_key,
    );

    if renamed {
        vault.save()?;
        println!(
            "{} '{}' -> '{}'",
            colors::success("Renamed"),
            colors::key_name(old_key),
            colors::key_name(new_key)
        );
    } else {
        return Err(format!("Secret '{}' not found", old_key).into());
    }

    Ok(())
}
