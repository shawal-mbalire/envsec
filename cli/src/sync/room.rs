use sha2::{Digest, Sha256};

/// Derive a room ID from passphrase hash, project, and environment.
/// The room ID includes a short hash so only users who know the
/// passphrase can compute the correct room name.
///
/// We use the Argon2id hash (from the session) as input rather than
/// the raw passphrase. This is deterministic - same passphrase always
/// produces the same hash, so same room ID.
///
/// Format: "{project}-{environment}-{hash[:8]}"
pub fn derive_room_id(passphrase_hash: &str, project: &str, environment: &str) -> String {
    let hash = compute_hash(passphrase_hash, project, environment);
    format!("{}-{}-{}", project, environment, &hash[..8])
}

/// Verify that a room ID matches the expected hash.
pub fn verify_room_id(
    room_id: &str,
    passphrase_hash: &str,
    project: &str,
    environment: &str,
) -> bool {
    let expected = derive_room_id(passphrase_hash, project, environment);
    room_id == expected
}

/// Compute SHA256 hash of passphrase_hash + project + environment.
fn compute_hash(passphrase_hash: &str, project: &str, environment: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(passphrase_hash.as_bytes());
    hasher.update(project.as_bytes());
    hasher.update(environment.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

/// Parse a room ID into (project, environment, hash) parts.
pub fn parse_room_id(room_id: &str) -> Option<(&str, &str, &str)> {
    // Find last hyphen to split hash off
    let hash_start = room_id.rfind('-')?;
    let hash = &room_id[hash_start + 1..];
    let prefix = &room_id[..hash_start];

    // Hash is always 8 hex chars
    if hash.len() != 8 {
        return None;
    }

    // The prefix is "project-environment", split on first hyphen
    let env_start = prefix.find('-')?;
    let project = &prefix[..env_start];
    let environment = &prefix[env_start + 1..];

    Some((project, environment, hash))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_room_id() {
        let room = derive_room_id("argon2hash123", "myapp", "dev");
        assert!(room.starts_with("myapp-dev-"));
        assert_eq!(room.len(), "myapp-dev-".len() + 8);
    }

    #[test]
    fn test_derive_room_id_deterministic() {
        let r1 = derive_room_id("argon2hash123", "myapp", "dev");
        let r2 = derive_room_id("argon2hash123", "myapp", "dev");
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_derive_room_id_different_hash() {
        let r1 = derive_room_id("hash1", "myapp", "dev");
        let r2 = derive_room_id("hash2", "myapp", "dev");
        assert_ne!(r1, r2);
    }

    #[test]
    fn test_derive_room_id_different_project() {
        let r1 = derive_room_id("hash", "app1", "dev");
        let r2 = derive_room_id("hash", "app2", "dev");
        assert_ne!(r1, r2);
    }

    #[test]
    fn test_verify_room_id() {
        let room = derive_room_id("hash", "myapp", "dev");
        assert!(verify_room_id(&room, "hash", "myapp", "dev"));
        assert!(!verify_room_id(&room, "wrong", "myapp", "dev"));
        assert!(!verify_room_id(&room, "hash", "other", "dev"));
    }

    #[test]
    fn test_parse_room_id() {
        let room = derive_room_id("hash", "myapp", "dev");
        let (proj, env, hash) = parse_room_id(&room).unwrap();
        assert_eq!(proj, "myapp");
        assert_eq!(env, "dev");
        assert_eq!(hash.len(), 8);
    }

    #[test]
    fn test_full_roundtrip() {
        let hash = "argon2id-hash-of-my-passphrase";
        let project = "myapp";
        let environment = "production";

        let room_id = derive_room_id(hash, project, environment);
        assert!(verify_room_id(&room_id, hash, project, environment));

        let (p, e, _) = parse_room_id(&room_id).unwrap();
        assert_eq!(p, project);
        assert_eq!(e, environment);
    }
}

