pub fn mask_value(value: &str) -> String {
    if value.len() <= 3 {
        "***".to_string()
    } else {
        format!("{}***", &value[..3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_short() {
        assert_eq!(mask_value("ab"), "***");
        assert_eq!(mask_value(""), "***");
    }

    #[test]
    fn test_mask_long() {
        assert_eq!(mask_value("sk-1234567890"), "sk-***");
        assert_eq!(mask_value("abcdefgh"), "abc***");
    }
}
