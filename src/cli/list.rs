use crate::config;
use crate::output::{colors, masked, table};
use crate::project::resolver;
use crate::session;
use crate::vault;

pub fn run(all: bool) -> Result<(), Box<dyn std::error::Error>> {
    let session = session::check_session().map_err(|e| {
        format!(
            "{}. Run {} to authenticate.",
            e,
            colors::bold("envsec auth")
        )
    })?;

    let vault_path = config::vault_path();
    let vault = vault::Vault::load(&vault_path, session.passphrase_hash.as_bytes())?;

    if all {
        list_all(&vault)?;
    } else {
        list_current(&vault)?;
    }

    Ok(())
}

fn list_current(vault: &vault::Vault) -> Result<(), Box<dyn std::error::Error>> {
    let project_config = resolver::load_current_project().map_err(|e| {
        format!(
            "{}. Run {} to set a project.",
            e,
            colors::bold("envsec use <project> <environment>")
        )
    })?;

    let environment = vault
        .data()
        .get_project(&project_config.project)
        .and_then(|p| p.environments.get(&project_config.environment));

    match environment {
        Some(env) => {
            if env.secrets.is_empty() {
                println!(
                    "{}",
                    colors::dim(&format!(
                        "No secrets in {} / {}",
                        project_config.project, project_config.environment
                    ))
                );
            } else {
                let mut rows: Vec<table::KeyValueRow> = env
                    .secrets
                    .iter()
                    .map(|(k, s)| table::KeyValueRow {
                        key: k.clone(),
                        value: masked::mask_value(&s.value),
                        updated: s.updated.format("%Y-%m-%d %H:%M").to_string(),
                    })
                    .collect();
                rows.sort_by(|a, b| a.key.cmp(&b.key));

                table::print_table(
                    &rows,
                    Some(&format!(
                        "{} / {}",
                        project_config.project, project_config.environment
                    )),
                );
            }
        }
        None => {
            println!(
                "{}",
                colors::dim(&format!(
                    "No environment '{}' in project '{}'",
                    project_config.environment, project_config.project
                ))
            );
        }
    }

    println!();
    Ok(())
}

fn list_all(vault: &vault::Vault) -> Result<(), Box<dyn std::error::Error>> {
    let data = vault.data();

    if data.projects.is_empty() {
        println!("{}", colors::dim("No projects found."));
        return Ok(());
    }

    let mut rows: Vec<table::ProjectRow> = Vec::new();

    for (project_name, project) in &data.projects {
        let env_names: Vec<&str> = project.environments.keys().map(|s| s.as_str()).collect();
        let total_keys: usize = project
            .environments
            .values()
            .map(|e| e.secrets.len())
            .sum();

        rows.push(table::ProjectRow {
            project: project_name.clone(),
            environments: env_names.join(", "),
            key_count: total_keys.to_string(),
        });
    }

    rows.sort_by(|a, b| a.project.cmp(&b.project));
    table::print_table(&rows, Some("All projects"));

    // Show details for each project
    for (project_name, project) in &data.projects {
        for (env_name, environment) in &project.environments {
            if environment.secrets.is_empty() {
                continue;
            }
            let mut kv_rows: Vec<table::KeyValueRow> = environment
                .secrets
                .iter()
                .map(|(k, s)| table::KeyValueRow {
                    key: k.clone(),
                    value: masked::mask_value(&s.value),
                    updated: s.updated.format("%Y-%m-%d %H:%M").to_string(),
                })
                .collect();
            kv_rows.sort_by(|a, b| a.key.cmp(&b.key));

            table::print_table(
                &kv_rows,
                Some(&format!("  {} / {}", project_name, env_name)),
            );
        }
    }

    println!();
    Ok(())
}
