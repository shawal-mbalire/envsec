use crate::config;
use crate::dotenv;
use crate::output::colors;
use crate::project::resolver;
use crate::session;
use crate::vault;

pub fn run_import(
    file: &str,
    project: Option<&str>,
    environment: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let session = session::check_session().map_err(|e| {
        format!(
            "{}. Run {} to authenticate.",
            e,
            colors::bold("envsec auth")
        )
    })?;

    let (proj, env) = match (project, environment) {
        (Some(p), Some(e)) => (p.to_string(), e.to_string()),
        _ => {
            let pc = resolver::load_current_project().map_err(|e| {
                format!(
                    "{}. Run {} to set a project or use --project/--env flags.",
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

    let path = std::path::Path::new(file);
    let secrets = dotenv::parse_env_file(path)?;

    if secrets.is_empty() {
        return Err("No secrets found in .env file".into());
    }

    let vault_path = config::vault_path();
    let mut vault = vault::Vault::load(&vault_path, session.passphrase_hash.as_bytes())?;

    let count = secrets.len();
    for (key, value) in &secrets {
        vault.data_mut().set_secret(&proj, &env, key, value);
    }
    vault.save()?;

    println!(
        "{} Imported {} secrets into {} / {}",
        colors::success("OK"),
        colors::bold(&count.to_string()),
        colors::key_name(&proj),
        colors::key_name(&env)
    );

    Ok(())
}
