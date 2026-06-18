use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum DotenvError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("parse error at line {line}: {message}")]
    Parse { line: usize, message: String },
}

pub fn parse_env_file(path: &Path) -> Result<HashMap<String, String>, DotenvError> {
    let contents = fs::read_to_string(path)?;
    parse_env_contents(&contents)
}

pub fn parse_env_contents(contents: &str) -> Result<HashMap<String, String>, DotenvError> {
    let mut map = HashMap::new();

    for (line_num, line) in contents.lines().enumerate() {
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Skip 'export ' prefix
        let line = line.strip_prefix("export ").unwrap_or(line);

        let Some(eq_pos) = line.find('=') else {
            return Err(DotenvError::Parse {
                line: line_num + 1,
                message: "missing '='".to_string(),
            });
        };

        let key = line[..eq_pos].trim().to_string();
        let value_raw = line[eq_pos + 1..].trim();

        if key.is_empty() {
            return Err(DotenvError::Parse {
                line: line_num + 1,
                message: "empty key".to_string(),
            });
        }

        let value = unquote_value(value_raw);
        map.insert(key, value);
    }

    Ok(map)
}

fn unquote_value(s: &str) -> String {
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

pub fn generate_env_contents(secrets: &HashMap<String, String>) -> String {
    let mut lines: Vec<String> = secrets
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect();
    lines.sort();
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let contents = r#"
# Comment
KEY1=value1
KEY2=value2
export KEY3=value3
"#;
        let map = parse_env_contents(contents).unwrap();
        assert_eq!(map.get("KEY1").unwrap(), "value1");
        assert_eq!(map.get("KEY2").unwrap(), "value2");
        assert_eq!(map.get("KEY3").unwrap(), "value3");
    }

    #[test]
    fn test_parse_quoted() {
        let contents = r#"
KEY1="quoted value"
KEY2='single quoted'
"#;
        let map = parse_env_contents(contents).unwrap();
        assert_eq!(map.get("KEY1").unwrap(), "quoted value");
        assert_eq!(map.get("KEY2").unwrap(), "single quoted");
    }

    #[test]
    fn test_generate_env() {
        let mut secrets = HashMap::new();
        secrets.insert("B".to_string(), "2".to_string());
        secrets.insert("A".to_string(), "1".to_string());
        let output = generate_env_contents(&secrets);
        assert_eq!(output, "A=1\nB=2");
    }
}
