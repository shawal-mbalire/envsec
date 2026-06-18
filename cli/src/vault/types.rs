use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Secret {
    pub value: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub secrets: HashMap<String, Secret>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub environments: HashMap<String, Environment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultData {
    pub version: u32,
    pub projects: HashMap<String, Project>,
}

impl Default for VaultData {
    fn default() -> Self {
        Self {
            version: 1,
            projects: HashMap::new(),
        }
    }
}

impl VaultData {
    pub fn project_names(&self) -> Vec<&str> {
        self.projects.keys().map(|s| s.as_str()).collect()
    }

    pub fn get_project(&self, name: &str) -> Option<&Project> {
        self.projects.get(name)
    }

    pub fn get_project_mut(&mut self, name: &str) -> Option<&mut Project> {
        self.projects.get_mut(name)
    }

    pub fn ensure_project(&mut self, name: &str) -> &mut Project {
        self.projects
            .entry(name.to_string())
            .or_insert_with(|| Project {
                environments: HashMap::new(),
            })
    }

    pub fn ensure_environment(&mut self, project: &str, env: &str) -> &mut Environment {
        let proj = self.ensure_project(project);
        proj.environments
            .entry(env.to_string())
            .or_insert_with(|| Environment {
                secrets: HashMap::new(),
            })
    }

    pub fn get_secret(&self, project: &str, env: &str, key: &str) -> Option<&Secret> {
        self.projects
            .get(project)?
            .environments
            .get(env)?
            .secrets
            .get(key)
    }

    pub fn set_secret(&mut self, project: &str, env: &str, key: &str, value: &str) {
        let environment = self.ensure_environment(project, env);
        let now = Utc::now();
        environment
            .secrets
            .entry(key.to_string())
            .and_modify(|s| {
                s.value = value.to_string();
                s.updated = now;
            })
            .or_insert_with(|| Secret {
                value: value.to_string(),
                created: now,
                updated: now,
            });
    }

    pub fn remove_secret(&mut self, project: &str, env: &str, key: &str) -> bool {
        if let Some(proj) = self.projects.get_mut(project) {
            if let Some(environment) = proj.environments.get_mut(env) {
                return environment.secrets.remove(key).is_some();
            }
        }
        false
    }

    pub fn rename_secret(&mut self, project: &str, env: &str, old_key: &str, new_key: &str) -> bool {
        if let Some(proj) = self.projects.get_mut(project) {
            if let Some(environment) = proj.environments.get_mut(env) {
                if let Some(secret) = environment.secrets.remove(old_key) {
                    environment.secrets.insert(new_key.to_string(), secret);
                    return true;
                }
            }
        }
        false
    }

    pub fn remove_project(&mut self, name: &str) -> bool {
        self.projects.remove(name).is_some()
    }
}
