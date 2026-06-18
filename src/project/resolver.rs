use std::env;
use std::path::PathBuf;

use super::{ProjectConfig, ProjectError};

const ENVSEC_FILE: &str = ".envsec";

pub fn find_envsec_file() -> Option<PathBuf> {
    let Ok(cwd) = env::current_dir() else {
        return None;
    };
    let mut dir = cwd.as_path();
    loop {
        let candidate = dir.join(ENVSEC_FILE);
        if candidate.exists() {
            return Some(candidate);
        }
        dir = dir.parent()?;
    }
}

pub fn load_current_project() -> Result<ProjectConfig, ProjectError> {
    let path = find_envsec_file().ok_or(ProjectError::NotFound)?;
    ProjectConfig::load(&path)
}

pub fn write_project_config(project: &str, environment: &str) -> Result<PathBuf, ProjectError> {
    let cwd = env::current_dir().map_err(ProjectError::Io)?;
    let path = cwd.join(ENVSEC_FILE);
    let config = ProjectConfig {
        project: project.to_string(),
        environment: environment.to_string(),
    };
    config.save(&path)?;
    Ok(path)
}
