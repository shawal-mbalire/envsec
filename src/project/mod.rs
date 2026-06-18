pub mod resolver;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub project: String,
    pub environment: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ProjectError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("toml error: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("no .envsec file found in current directory or parents")]
    NotFound,
}

impl ProjectConfig {
    pub fn load(path: &Path) -> Result<Self, ProjectError> {
        let contents = fs::read_to_string(path)?;
        let config: ProjectConfig = toml::from_str(&contents)?;
        Ok(config)
    }

    pub fn save(&self, path: &Path) -> Result<(), ProjectError> {
        let contents = toml::to_string_pretty(self).unwrap_or_default();
        fs::write(path, contents)?;
        Ok(())
    }
}
