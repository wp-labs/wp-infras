use crate::formatter::DataFormat;
use std::fmt::Write;
use wp_model_core::model::{DataField, DataRecord, DataType, types::value::ObjectValue};

pub struct KeyValue {
    pair_separator: String,
    key_value_separator: String,
    quote_strings: bool,
}

impl Default for KeyValue {
    fn default() -> Self {
        Self {
            pair_separator: ", ".to_string(),
            key_value_separator: ": ".to_string(),
            quote_strings: true,
        }
    }
}

impl KeyValue {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_pair_separator(mut self, s: impl Into<String>) -> Self {
        self.pair_separator = s.into();
        self
    }
    pub fn with_key_value_separator(mut self, s: impl Into<String>) -> Self {
        self.key_value_separator = s.into();
        self
    }
    pub fn with_quote_strings(mut self, quote: bool) -> Self {
        self.quote_strings = quote;
        self
    }

    fn format_string_value(&self, value: &str) -> String {
        if self.quote_strings {
            format!("\"{}\"", value.replace('\"', "\\\""))
        } else {
            value.to_string()
        }
    }
}

impl DataFormat for KeyValue {
    type Output = String;

    fn format_null(&self) -> String {
        String::new()
    }
    fn format_bool(&self, v: &bool) -> String {
        if *v { "true".into() } else { "false".into() }
    }
    fn format_string(&self, v: &str) -> String {
        self.format_string_value(v)
    }
    fn format_i64(&self, v: &i64) -> String {
        v.to_string()
    }
    fn format_f64(&self, v: &f64) -> String {
        v.to_string()
    }
    fn format_ip(&self, v: &std::net::IpAddr) -> String {
        v.to_string()
    }
    fn format_datetime(&self, v: &chrono::NaiveDateTime) -> String {
        v.to_string()
    }

    fn format_object(&self, value: &ObjectValue) -> String {
        let mut output = String::new();
        output.push('{');
        for (i, (k, v)) in value.iter().enumerate() {
            if i > 0 {
                output.push_str(&self.pair_separator);
            }
            write!(
                output,
                "{}{}{}",
                self.format_string(k),
                self.key_value_separator,
                self.fmt_value(v.get_value())
            )
            .unwrap();
        }
        output.push('}');
        output
    }

    fn format_array(&self, value: &[DataField]) -> String {
        let mut output = String::new();
        output.push('[');
        for (i, field) in value.iter().enumerate() {
            if i > 0 {
                output.push_str(&self.pair_separator);
            }
            output.push_str(&self.fmt_value(field.get_value()));
        }
        output.push(']');
        output
    }

    fn format_field(&self, field: &DataField) -> String {
        format!(
            "{}{}{}",
            field.get_name(),
            self.key_value_separator,
            self.fmt_value(field.get_value())
        )
    }

    fn format_record(&self, record: &DataRecord) -> String {
        record
            .items
            .iter()
            .filter(|f| *f.get_meta() != DataType::Ignore)
            .map(|field| self.format_field(field))
            .collect::<Vec<_>>()
            .join(&self.pair_separator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::IpAddr;
    use std::str::FromStr;

    #[test]
    fn test_kv_default() {
        let kv = KeyValue::default();
        assert_eq!(kv.pair_separator, ", ");
        assert_eq!(kv.key_value_separator, ": ");
        assert!(kv.quote_strings);
    }

    #[test]
    fn test_kv_new() {
        let kv = KeyValue::new();
        assert_eq!(kv.pair_separator, ", ");
    }

    #[test]
    fn test_kv_builder_pattern() {
        let kv = KeyValue::new()
            .with_pair_separator("; ")
            .with_key_value_separator("=")
            .with_quote_strings(false);
        assert_eq!(kv.pair_separator, "; ");
        assert_eq!(kv.key_value_separator, "=");
        assert!(!kv.quote_strings);
    }

    #[test]
    fn test_format_null() {
        let kv = KeyValue::default();
        assert_eq!(kv.format_null(), "");
    }

    #[test]
    fn test_format_bool() {
        let kv = KeyValue::default();
        assert_eq!(kv.format_bool(&true), "true");
        assert_eq!(kv.format_bool(&false), "false");
    }

    #[test]
    fn test_format_string_with_quotes() {
        let kv = KeyValue::default();
        assert_eq!(kv.format_string("hello"), "\"hello\"");
        assert_eq!(kv.format_string("world"), "\"world\"");
    }

    #[test]
    fn test_format_string_without_quotes() {
        let kv = KeyValue::new().with_quote_strings(false);
        assert_eq!(kv.format_string("hello"), "hello");
    }

    #[test]
    fn test_format_string_escape_quotes() {
        let kv = KeyValue::default();
        assert_eq!(kv.format_string("say \"hi\""), "\"say \\\"hi\\\"\"");
    }

    #[test]
    fn test_format_i64() {
        let kv = KeyValue::default();
        assert_eq!(kv.format_i64(&0), "0");
        assert_eq!(kv.format_i64(&42), "42");
        assert_eq!(kv.format_i64(&-100), "-100");
    }

    #[test]
    fn test_format_f64() {
        let kv = KeyValue::default();
        assert_eq!(kv.format_f64(&3.24), "3.24");
        assert_eq!(kv.format_f64(&0.0), "0");
    }

    #[test]
    fn test_format_ip() {
        let kv = KeyValue::default();
        let ip = IpAddr::from_str("192.168.1.1").unwrap();
        assert_eq!(kv.format_ip(&ip), "192.168.1.1");
    }

    #[test]
    fn test_format_datetime() {
        let kv = KeyValue::default();
        let dt = chrono::NaiveDateTime::parse_from_str("2024-01-15 10:30:45", "%Y-%m-%d %H:%M:%S")
            .unwrap();
        let result = kv.format_datetime(&dt);
        assert!(result.contains("2024"));
    }

    #[test]
    fn test_format_field() {
        let kv = KeyValue::default();
        let field = DataField::from_chars("name", "Alice");
        let result = kv.format_field(&field);
        assert_eq!(result, "name: \"Alice\"");
    }

    #[test]
    fn test_format_field_with_custom_separator() {
        let kv = KeyValue::new().with_key_value_separator("=");
        let field = DataField::from_digit("age", 30);
        let result = kv.format_field(&field);
        assert_eq!(result, "age=30");
    }

    #[test]
    fn test_format_record() {
        let kv = KeyValue::default();
        let record = DataRecord {
            items: vec![
                DataField::from_chars("name", "Alice"),
                DataField::from_digit("age", 30),
            ],
        };
        let result = kv.format_record(&record);
        assert!(result.contains("name: \"Alice\""));
        assert!(result.contains("age: 30"));
        assert!(result.contains(", "));
    }

    #[test]
    fn test_format_record_custom_separators() {
        let kv = KeyValue::new()
            .with_pair_separator(" | ")
            .with_key_value_separator("=")
            .with_quote_strings(false);
        let record = DataRecord {
            items: vec![
                DataField::from_chars("a", "x"),
                DataField::from_chars("b", "y"),
            ],
        };
        let result = kv.format_record(&record);
        assert_eq!(result, "a=x | b=y");
    }

    #[test]
    fn test_format_array() {
        let kv = KeyValue::default();
        let arr = vec![DataField::from_digit("", 1), DataField::from_digit("", 2)];
        let result = kv.format_array(&arr);
        assert!(result.starts_with('['));
        assert!(result.ends_with(']'));
        assert!(result.contains("1"));
        assert!(result.contains("2"));
    }
}
