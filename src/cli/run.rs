use std::process::Command;

use crate::config;
use crate::output::colors;
use crate::project::resolver;
use crate::session;
use crate::vault;

pub fn run(command: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if command.is_empty() {
        return Err("No command specified".into());
    }

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

    let environment = vault
        .data()
        .get_project(&project_config.project)
        .and_then(|p| p.environments.get(&project_config.environment));

    let secrets: Vec<(String, String)> = match environment {
        Some(env) => env
            .secrets
            .iter()
            .map(|(k, s)| (k.clone(), s.value.clone()))
            .collect(),
        None => Vec::new(),
    };

    let cmd_name = &command[0];
    let args = &command[1..];

    let mut cmd = Command::new(cmd_name);
    cmd.args(args);

    // Inject secrets as environment variables
    for (key, value) in &secrets {
        cmd.env(key, value);
    }

    // Inherit parent stdin/stdout/stderr
    cmd.stdin(std::process::Stdio::inherit());
    cmd.stdout(std::process::Stdio::inherit());
    cmd.stderr(std::process::Stdio::inherit());

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to run '{}': {}", cmd_name, e))?;

    std::process::exit(status.code().unwrap_or(1));
}
