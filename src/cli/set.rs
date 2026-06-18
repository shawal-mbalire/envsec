use dialoguer::Password;

use crate::config;
use crate::output::colors;
use crate::project::resolver;
use crate::session;
use crate::vault;

pub fn run(key: &str, value: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
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

    let actual_value = match value {
        Some(v) => v.to_string(),
        None => Password::new()
            .with_prompt(format!("Value for {}", key))
            .allow_empty_password(true)
            .interact()?,
    };

    let vault_path = config::vault_path();
    let mut vault = vault::Vault::load(&vault_path, session.passphrase_hash.as_bytes())?;

    vault.data_mut().set_secret(
        &project_config.project,
        &project_config.environment,
        key,
        &actual_value,
    );
    vault.save()?;

    println!(
        "{} {} = {}",
        colors::success("Set"),
        colors::key_name(key),
        colors::value_masked("***")
    );

    Ok(())
}
