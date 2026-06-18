use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

use crate::config;
use crate::dotenv;
use crate::output::colors;
use crate::project::resolver;
use crate::session;
use crate::vault;

pub fn run_export(file: Option<&str>, raw: bool) -> Result<(), Box<dyn std::error::Error>> {
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

    let secrets: HashMap<String, String> = match environment {
        Some(env) => env
            .secrets
            .iter()
            .map(|(k, s)| (k.clone(), s.value.clone()))
            .collect(),
        None => HashMap::new(),
    };

    if secrets.is_empty() {
        println!("{}", colors::dim("No secrets to export."));
        return Ok(());
    }

    if raw {
        if file.is_none() {
            // Printing to stdout with real values - warn
            eprintln!(
                "{}",
                colors::warning("WARNING: Printing secrets to stdout. Use --file to write to a file instead.")
            );
        }
        let contents = dotenv::generate_env_contents(&secrets);

        match file {
            Some(path) => {
                fs::write(path, &contents)?;
                // Set permissions to 600 on Unix
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    fs::set_permissions(path, fs::Permissions::from_mode(0o600))?;
                }
                println!(
                    "{} Wrote {} secrets to {}",
                    colors::success("OK"),
                    colors::bold(&secrets.len().to_string()),
                    colors::key_name(path)
                );
            }
            None => {
                io::stdout().write_all(contents.as_bytes())?;
                println!();
            }
        }
    } else {
        // Masked output
        let masked: HashMap<String, String> = secrets
            .iter()
            .map(|(k, v)| {
                use crate::output::masked;
                (k.clone(), masked::mask_value(v))
            })
            .collect();
        let contents = dotenv::generate_env_contents(&masked);

        match file {
            Some(path) => {
                eprintln!(
                    "{}",
                    colors::dim("(Values are masked. Use --raw to export actual values.)")
                );
                fs::write(path, &contents)?;
                println!(
                    "{} Wrote {} masked secrets to {}",
                    colors::success("OK"),
                    colors::bold(&secrets.len().to_string()),
                    colors::key_name(path)
                );
            }
            None => {
                eprintln!(
                    "{}",
                    colors::dim("(Values are masked. Use --raw to see actual values.)")
                );
                io::stdout().write_all(contents.as_bytes())?;
                println!();
            }
        }
    }

    Ok(())
}
