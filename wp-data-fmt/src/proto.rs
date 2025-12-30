use crate::formatter::DataFormat;
use wp_model_core::model::{DataField, DataRecord, DataType, Value, types::value::ObjectValue};

#[derive(Default)]
pub struct ProtoTxt;

impl ProtoTxt {
    pub fn new() -> Self {
        Self
    }
}

impl DataFormat for ProtoTxt {
    type Output = String;
    fn format_null(&self) -> String {
        String::new()
    }
    fn format_bool(&self, v: &bool) -> String {
        v.to_string()
    }
    fn format_string(&self, v: &str) -> String {
        format!("\"{}\"", v.replace('"', "\\\""))
    }
    fn format_i64(&self, v: &i64) -> String {
        v.to_string()
    }
    fn format_f64(&self, v: &f64) -> String {
        v.to_string()
    }
    fn format_ip(&self, v: &std::net::IpAddr) -> String {
        self.format_string(&v.to_string())
    }
    fn format_datetime(&self, v: &chrono::NaiveDateTime) -> String {
        self.format_string(&v.to_string())
    }
    fn format_object(&self, value: &ObjectValue) -> String {
        let mut out = String::new();
        for (k, v) in value.iter() {
            out.push_str(&format!("{}: {}\n", k, self.fmt_value(v.get_value())));
        }
        out
    }
    fn format_array(&self, value: &[DataField]) -> String {
        let items: Vec<String> = value.iter().map(|f| self.format_field(f)).collect();
        format!("[{}]", items.join(", "))
    }
    fn format_field(&self, field: &DataField) -> String {
        if *field.get_meta() == DataType::Ignore {
            String::new()
        } else {
            match field.get_value() {
                Value::Obj(_) | Value::Array(_) => format!(
                    "{}: {}",
                    field.get_name(),
                    self.fmt_value(field.get_value())
                ),
                _ => format!(
                    "{}: {}",
                    field.get_name(),
                    self.fmt_value(field.get_value())
                ),
            }
        }
    }
    fn format_record(&self, record: &DataRecord) -> String {
        let items = record
            .items
            .iter()
            .filter(|f| *f.get_meta() != DataType::Ignore)
            .map(|f| self.format_field(f))
            .collect::<Vec<_>>();
        // 生成标准的 proto-text 格式：消息用花括号包围
        format!("{{ {} }}", items.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::IpAddr;
    use std::str::FromStr;

    #[test]
    fn test_proto_new() {
        let proto = ProtoTxt::new();
        assert_eq!(proto.format_null(), "");
    }

    #[test]
    fn test_proto_default() {
        let proto = ProtoTxt;
        assert_eq!(proto.format_null(), "");
    }

    #[test]
    fn test_format_null() {
        let proto = ProtoTxt;
        assert_eq!(proto.format_null(), "");
    }

    #[test]
    fn test_format_bool() {
        let proto = ProtoTxt;
        assert_eq!(proto.format_bool(&true), "true");
        assert_eq!(proto.format_bool(&false), "false");
    }

    #[test]
    fn test_format_string() {
        let proto = ProtoTxt;
        assert_eq!(proto.format_string("hello"), "\"hello\"");
        assert_eq!(proto.format_string(""), "\"\"");
    }

    #[test]
    fn test_format_string_escape_quotes() {
        let proto = ProtoTxt;
        assert_eq!(proto.format_string("say \"hi\""), "\"say \\\"hi\\\"\"");
    }

    #[test]
    fn test_format_i64() {
        let proto = ProtoTxt;
        assert_eq!(proto.format_i64(&0), "0");
        assert_eq!(proto.format_i64(&42), "42");
        assert_eq!(proto.format_i64(&-100), "-100");
    }

    #[test]
    fn test_format_f64() {
        let proto = ProtoTxt;
        assert_eq!(proto.format_f64(&3.24), "3.24");
        assert_eq!(proto.format_f64(&0.0), "0");
    }

    #[test]
    fn test_format_ip() {
        let proto = ProtoTxt;
        let ip = IpAddr::from_str("192.168.1.1").unwrap();
        assert_eq!(proto.format_ip(&ip), "\"192.168.1.1\"");
    }

    #[test]
    fn test_format_datetime() {
        let proto = ProtoTxt;
        let dt = chrono::NaiveDateTime::parse_from_str("2024-01-15 10:30:45", "%Y-%m-%d %H:%M:%S")
            .unwrap();
        let result = proto.format_datetime(&dt);
        assert!(result.starts_with('"'));
        assert!(result.ends_with('"'));
        assert!(result.contains("2024"));
    }

    #[test]
    fn test_format_field() {
        let proto = ProtoTxt;
        let field = DataField::from_chars("name", "Alice");
        let result = proto.format_field(&field);
        assert_eq!(result, "name: \"Alice\"");
    }

    #[test]
    fn test_format_field_digit() {
        let proto = ProtoTxt;
        let field = DataField::from_digit("age", 30);
        let result = proto.format_field(&field);
        assert_eq!(result, "age: 30");
    }

    #[test]
    fn test_format_record() {
        let proto = ProtoTxt;
        let record = DataRecord {
            items: vec![
                DataField::from_chars("name", "Alice"),
                DataField::from_digit("age", 30),
            ],
        };
        let result = proto.format_record(&record);
        assert!(result.starts_with("{ "));
        assert!(result.ends_with(" }"));
        assert!(result.contains("name: \"Alice\""));
        assert!(result.contains("age: 30"));
    }

    #[test]
    fn test_format_array() {
        let proto = ProtoTxt;
        let arr = vec![DataField::from_digit("x", 1), DataField::from_digit("y", 2)];
        let result = proto.format_array(&arr);
        assert!(result.starts_with('['));
        assert!(result.ends_with(']'));
    }
}
