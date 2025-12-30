use std::fmt::Write;
use wp_connector_api::Tags;

use crate::ConfParser;

impl ConfParser<[String]> for Tags {
    fn from_parse(items: &[String]) -> Self {
        parse_tags(items)
    }

    fn validate(items: &[String]) -> Result<(), String> {
        validate_tags(items)
    }
}
fn parse_tags(items: &[String]) -> Tags {
    let mut tags = Tags::default();
    for item in items {
        if let Some((k, v)) = item.split_once(':').or_else(|| item.split_once('=')) {
            tags.set(k.trim(), v.trim().to_string());
        } else {
            tags.set(item.trim(), "true".to_string());
        }
    }
    tags
}

/// 校验 tags 列表是否满足约束：
/// - 数量：最多 4 个
/// - key：1..=32，字符集 [A-Za-z0-9_.-]
/// - value：0..=64，字符集 [A-Za-z0-9_.:/=@+,-]
///   返回 Err(String) 以避免在该 crate 引入错误依赖；上层可将其映射为 anyhow。
fn validate_tags(items: &[String]) -> Result<(), String> {
    if items.len() > 4 {
        return Err(format!(
            "tags must have at most 4 items (got {})",
            items.len()
        ));
    }
    for (idx, item) in items.iter().enumerate() {
        let (k, v) = if let Some((k, v)) = item.split_once(':').or_else(|| item.split_once('=')) {
            (k.trim(), v.trim())
        } else {
            (item.trim(), "true")
        };
        if k.is_empty() || k.len() > 32 || !k.chars().all(is_valid_key_char) {
            let mut msg = String::new();
            let _ = write!(
                &mut msg,
                "invalid tag key at index {}: '{}' (allowed: [A-Za-z0-9_.-], len 1..=32)",
                idx, k
            );
            return Err(msg);
        }
        if v.len() > 64 || !v.chars().all(is_valid_val_char) {
            let mut msg = String::new();
            let _ = write!(
                &mut msg,
                "invalid tag value at index {}: '{}' (allowed: [A-Za-z0-9_.:/=@+,-], len 0..=64)",
                idx, v
            );
            return Err(msg);
        }
    }
    Ok(())
}

fn is_valid_key_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-')
}
fn is_valid_val_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | ':' | '/' | '=' | '@' | '+' | ',' | '-')
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== parse_tags 测试 ==========

    #[test]
    fn test_parse_tags_with_colon_separator() {
        let items = vec!["env:prod".to_string(), "region:us-east-1".to_string()];
        let tags = parse_tags(&items);
        assert_eq!(tags.get("env"), Some("prod"));
        assert_eq!(tags.get("region"), Some("us-east-1"));
    }

    #[test]
    fn test_parse_tags_with_equals_separator() {
        let items = vec!["env=staging".to_string(), "version=1.0.0".to_string()];
        let tags = parse_tags(&items);
        assert_eq!(tags.get("env"), Some("staging"));
        assert_eq!(tags.get("version"), Some("1.0.0"));
    }

    #[test]
    fn test_parse_tags_without_separator() {
        let items = vec!["debug".to_string(), "enabled".to_string()];
        let tags = parse_tags(&items);
        assert_eq!(tags.get("debug"), Some("true"));
        assert_eq!(tags.get("enabled"), Some("true"));
    }

    #[test]
    fn test_parse_tags_with_whitespace() {
        let items = vec![
            " env : prod ".to_string(),
            "  region = us-west  ".to_string(),
        ];
        let tags = parse_tags(&items);
        assert_eq!(tags.get("env"), Some("prod"));
        assert_eq!(tags.get("region"), Some("us-west"));
    }

    #[test]
    fn test_parse_tags_mixed_formats() {
        let items = vec![
            "env:prod".to_string(),
            "region=us-east-1".to_string(),
            "debug".to_string(),
        ];
        let tags = parse_tags(&items);
        assert_eq!(tags.get("env"), Some("prod"));
        assert_eq!(tags.get("region"), Some("us-east-1"));
        assert_eq!(tags.get("debug"), Some("true"));
    }

    #[test]
    fn test_parse_tags_empty_list() {
        let items: Vec<String> = vec![];
        let tags = parse_tags(&items);
        assert_eq!(tags.len(), 0);
    }

    #[test]
    fn test_parse_tags_empty_value() {
        let items = vec!["key:".to_string()];
        let tags = parse_tags(&items);
        assert_eq!(tags.get("key"), Some(""));
    }

    // ========== validate_tags 测试 ==========

    #[test]
    fn test_validate_tags_success() {
        let items = vec![
            "env:prod".to_string(),
            "region:us-east-1".to_string(),
            "version:1.0.0".to_string(),
        ];
        assert!(validate_tags(&items).is_ok());
    }

    #[test]
    fn test_validate_tags_empty_list() {
        let items: Vec<String> = vec![];
        assert!(validate_tags(&items).is_ok());
    }

    #[test]
    fn test_validate_tags_max_allowed() {
        let items = vec![
            "tag1:value1".to_string(),
            "tag2:value2".to_string(),
            "tag3:value3".to_string(),
            "tag4:value4".to_string(),
        ];
        assert!(validate_tags(&items).is_ok());
    }

    #[test]
    fn test_validate_tags_too_many() {
        let items = vec![
            "tag1:value1".to_string(),
            "tag2:value2".to_string(),
            "tag3:value3".to_string(),
            "tag4:value4".to_string(),
            "tag5:value5".to_string(),
        ];
        let result = validate_tags(&items);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("tags must have at most 4 items")
        );
    }

    #[test]
    fn test_validate_tags_empty_key() {
        let items = vec![":value".to_string()];
        let result = validate_tags(&items);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalid tag key"));
    }

    #[test]
    fn test_validate_tags_key_too_long() {
        let long_key = "a".repeat(33);
        let items = vec![format!("{}:value", long_key)];
        let result = validate_tags(&items);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalid tag key"));
    }

    #[test]
    fn test_validate_tags_key_max_length() {
        let max_key = "a".repeat(32);
        let items = vec![format!("{}:value", max_key)];
        assert!(validate_tags(&items).is_ok());
    }

    #[test]
    fn test_validate_tags_invalid_key_char() {
        let items = vec!["env@prod:value".to_string()];
        let result = validate_tags(&items);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalid tag key"));
    }

    #[test]
    fn test_validate_tags_valid_key_chars() {
        let items = vec![
            "env_name:value".to_string(),
            "app.name:value".to_string(),
            "app-version:value".to_string(),
        ];
        assert!(validate_tags(&items).is_ok());
    }

    #[test]
    fn test_validate_tags_value_too_long() {
        let long_value = "v".repeat(65);
        let items = vec![format!("key:{}", long_value)];
        let result = validate_tags(&items);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalid tag value"));
    }

    #[test]
    fn test_validate_tags_value_max_length() {
        let max_value = "v".repeat(64);
        let items = vec![format!("key:{}", max_value)];
        assert!(validate_tags(&items).is_ok());
    }

    #[test]
    fn test_validate_tags_empty_value() {
        let items = vec!["key:".to_string()];
        assert!(validate_tags(&items).is_ok());
    }

    #[test]
    fn test_validate_tags_invalid_value_char() {
        let items = vec!["key:value#123".to_string()];
        let result = validate_tags(&items);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalid tag value"));
    }

    #[test]
    fn test_validate_tags_valid_value_chars() {
        let items = vec![
            "key:path/to/file".to_string(),
            "key:user@domain".to_string(),
            "key:v1.0.0+build".to_string(),
            "key:a,b,c".to_string(),
        ];
        assert!(validate_tags(&items).is_ok());
    }

    #[test]
    fn test_validate_tags_flag_without_separator() {
        let items = vec!["debug".to_string()];
        assert!(validate_tags(&items).is_ok());
    }

    #[test]
    fn test_validate_tags_with_equals_separator() {
        let items = vec!["env=prod".to_string()];
        assert!(validate_tags(&items).is_ok());
    }

    #[test]
    fn test_validate_tags_with_whitespace() {
        let items = vec![" env : prod ".to_string()];
        assert!(validate_tags(&items).is_ok());
    }

    // ========== 辅助函数测试 ==========

    #[test]
    fn test_is_valid_key_char() {
        // 合法字符
        assert!(is_valid_key_char('a'));
        assert!(is_valid_key_char('Z'));
        assert!(is_valid_key_char('0'));
        assert!(is_valid_key_char('_'));
        assert!(is_valid_key_char('.'));
        assert!(is_valid_key_char('-'));

        // 非法字符
        assert!(!is_valid_key_char('@'));
        assert!(!is_valid_key_char(':'));
        assert!(!is_valid_key_char('/'));
        assert!(!is_valid_key_char(' '));
    }

    #[test]
    fn test_is_valid_val_char() {
        // 合法字符
        assert!(is_valid_val_char('a'));
        assert!(is_valid_val_char('Z'));
        assert!(is_valid_val_char('0'));
        assert!(is_valid_val_char('_'));
        assert!(is_valid_val_char('.'));
        assert!(is_valid_val_char(':'));
        assert!(is_valid_val_char('/'));
        assert!(is_valid_val_char('='));
        assert!(is_valid_val_char('@'));
        assert!(is_valid_val_char('+'));
        assert!(is_valid_val_char(','));
        assert!(is_valid_val_char('-'));

        // 非法字符
        assert!(!is_valid_val_char('#'));
        assert!(!is_valid_val_char('$'));
        assert!(!is_valid_val_char(' '));
    }
}
