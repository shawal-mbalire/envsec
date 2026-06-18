use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const DEFAULT_SESSION_DURATION_SECS: u64 = 7200; // 2 hours
const DEFAULT_CLIPBOARD_CLEAR_SECS: u64 = 120; // 2 minutes

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub session: SessionConfig,
    pub clipboard: ClipboardConfig,
    pub output: OutputConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    pub duration_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardConfig {
    pub clear_after_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub color: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            session: SessionConfig {
                duration_secs: DEFAULT_SESSION_DURATION_SECS,
            },
            clipboard: ClipboardConfig {
                clear_after_secs: DEFAULT_CLIPBOARD_CLEAR_SECS,
            },
            output: OutputConfig { color: true },
        }
    }
}

pub fn config_dir() -> PathBuf {
    dirs::home_dir()
        .expect("could not determine home directory")
        .join(".envsec")
}

pub fn config_path() -> PathBuf {
    config_dir().join("config.toml")
}

pub fn vault_path() -> PathBuf {
    config_dir().join("vault.enc")
}

pub fn session_path() -> PathBuf {
    config_dir().join(".session")
}

pub fn load_config() -> Config {
    let path = config_path();
    if !path.exists() {
        return Config::default();
    }
    match fs::read_to_string(&path) {
        Ok(contents) => toml::from_str(&contents).unwrap_or_default(),
        Err(_) => Config::default(),
    }
}

pub fn save_config(config: &Config) -> Result<(), std::io::Error> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let contents = toml::to_string_pretty(config).unwrap_or_default();
    fs::write(&path, contents)
}

pub fn init_config_dir() -> Result<(), std::io::Error> {
    let dir = config_dir();
    fs::create_dir_all(&dir)?;

    // Set permissions to 700 on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&dir, fs::Permissions::from_mode(0o700))?;
    }

    Ok(())
}

pub fn set_session_duration(config: &mut Config, duration_str: &str) -> Result<(), String> {
    let secs = parse_duration(duration_str)?;
    config.session.duration_secs = secs;
    Ok(())
}

fn parse_duration(s: &str) -> Result<u64, String> {
    let s = s.trim();
    if s.is_empty() {
        return Err("empty duration".to_string());
    }

    let (num_str, multiplier) = if let Some(stripped) = s.strip_suffix('h') {
        (stripped, 3600)
    } else if let Some(stripped) = s.strip_suffix('m') {
        (stripped, 60)
    } else if let Some(stripped) = s.strip_suffix('s') {
        (stripped, 1)
    } else {
        (s, 1)
    };

    let num: u64 = num_str
        .parse()
        .map_err(|_| format!("invalid duration: '{}'", s))?;

    Ok(num * multiplier)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("2h").unwrap(), 7200);
        assert_eq!(parse_duration("30m").unwrap(), 1800);
        assert_eq!(parse_duration("120s").unwrap(), 120);
        assert_eq!(parse_duration("120").unwrap(), 120);
        assert!(parse_duration("abc").is_err());
    }
}
