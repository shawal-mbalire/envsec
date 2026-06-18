pub mod password;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use crate::config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub passphrase_hash: String,
    pub authenticated_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub duration_secs: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("session file corrupted: {0}")]
    Corrupted(String),
    #[error("password error: {0}")]
    Password(#[from] password::PasswordError),
    #[error("session expired")]
    Expired,
    #[error("no active session")]
    NoSession,
}

impl Session {
    pub fn is_valid(&self) -> bool {
        Utc::now() < self.expires_at
    }

    pub fn remaining_secs(&self) -> u64 {
        let now = Utc::now();
        if now >= self.expires_at {
            0
        } else {
            (self.expires_at - now).num_seconds() as u64
        }
    }
}

pub fn load_session() -> Result<Option<Session>, SessionError> {
    let path = config::session_path();
    if !path.exists() {
        return Ok(None);
    }
    let contents = fs::read_to_string(&path)?;
    let session: Session =
        serde_json::from_str(&contents).map_err(|e| SessionError::Corrupted(e.to_string()))?;
    Ok(Some(session))
}

pub fn create_session(
    passphrase: &[u8],
    duration_secs: u64,
) -> Result<Session, SessionError> {
    let hash = password::hash_passphrase(passphrase)?;
    let now = Utc::now();
    let session = Session {
        passphrase_hash: hash,
        authenticated_at: now,
        expires_at: now + chrono::Duration::seconds(duration_secs as i64),
        duration_secs,
    };
    save_session(&session)?;
    Ok(session)
}

pub fn validate_session(passphrase: &[u8]) -> Result<Session, SessionError> {
    let session = load_session()?.ok_or(SessionError::NoSession)?;
    if !session.is_valid() {
        return Err(SessionError::Expired);
    }
    if !password::verify_passphrase(passphrase, &session.passphrase_hash)? {
        return Err(SessionError::NoSession);
    }
    Ok(session)
}

pub fn check_session() -> Result<Session, SessionError> {
    let session = load_session()?.ok_or(SessionError::NoSession)?;
    if !session.is_valid() {
        return Err(SessionError::Expired);
    }
    Ok(session)
}

pub fn save_session(session: &Session) -> Result<(), SessionError> {
    let path = config::session_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let contents = serde_json::to_string(session).unwrap_or_default();
    fs::write(&path, contents)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600))?;
    }

    Ok(())
}

pub fn clear_session() -> Result<(), SessionError> {
    let path = config::session_path();
    if path.exists() {
        fs::remove_file(&path)?;
    }
    Ok(())
}
