use crate::config;
use crate::output::{colors, table};
use crate::project::resolver;
use crate::session;
use crate::vault;

pub fn run_projects() -> Result<(), Box<dyn std::error::Error>> {
    let session = session::check_session().map_err(|e| {
        format!(
            "{}. Run {} to authenticate.",
            e,
            colors::bold("envsec auth")
        )
    })?;

    let vault_path = config::vault_path();
    let vault = vault::Vault::load(&vault_path, session.passphrase_hash.as_bytes())?;

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
    table::print_table(&rows, Some("Projects"));
    println!();

    Ok(())
}

pub fn run_use(
    project: &str,
    environment: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let env = environment.unwrap_or("default");
    let path = resolver::write_project_config(project, env)?;

    println!(
        "{} Active project: {} / {}",
        colors::success("OK"),
        colors::key_name(project),
        colors::key_name(env)
    );
    println!(
        "  {} {}",
        colors::dim("Written to:"),
        path.display()
    );

    Ok(())
}

pub fn run_rm_project(project: &str) -> Result<(), Box<dyn std::error::Error>> {
    let session = session::check_session().map_err(|e| {
        format!(
            "{}. Run {} to authenticate.",
            e,
            colors::bold("envsec auth")
        )
    })?;

    let vault_path = config::vault_path();
    let mut vault = vault::Vault::load(&vault_path, session.passphrase_hash.as_bytes())?;

    let removed = vault.data_mut().remove_project(project);

    if removed {
        vault.save()?;
        println!(
            "{} Project '{}' and all its secrets removed.",
            colors::success("Removed"),
            colors::key_name(project)
        );
    } else {
        return Err(format!("Project '{}' not found", project).into());
    }

    Ok(())
}
