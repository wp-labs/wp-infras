use crate::formatter::StaticDataFormatter;
use serde_json::{Value as JsonValue, json};
use wp_model_core::model::{DataField, DataRecord, DataType, Value, types::value::ObjectValue};

#[derive(Debug, Default)]
pub struct Json;
impl StaticDataFormatter for Json {
    type Output = String;
    fn stdfmt_null() -> String {
        "null".to_string()
    }
    fn stdfmt_bool(value: &bool) -> String {
        json!(value).to_string()
    }
    fn stdfmt_string(value: &str) -> String {
        serde_json::to_string(value).unwrap_or_else(|_| "\"\"".to_string())
    }
    fn stdfmt_i64(value: &i64) -> String {
        json!(value).to_string()
    }
    fn stdfmt_f64(value: &f64) -> String {
        if value.is_nan() {
            "null".to_string()
        } else if value.is_infinite() {
            if value.is_sign_positive() {
                "\"Infinity\"".to_string()
            } else {
                "\"-Infinity\"".to_string()
            }
        } else {
            json!(value).to_string()
        }
    }
    fn stdfmt_ip_addr(value: &std::net::IpAddr) -> String {
        json!(value.to_string()).to_string()
    }
    fn stdfmt_datetime(value: &chrono::NaiveDateTime) -> String {
        json!(value.to_string()).to_string()
    }
    fn stdfmt_object(value: &ObjectValue) -> String {
        let mut json_obj = serde_json::Map::new();
        for (k, v) in value.iter() {
            json_obj.insert(k.to_string(), to_json_value(v.get_value()));
        }
        json!(json_obj).to_string()
    }
    fn stdfmt_array(value: &[DataField]) -> String {
        let items: Vec<String> = value
            .iter()
            .map(|field| match field.get_value() {
                Value::Chars(s) => Self::stdfmt_string(s),
                _ => Self::stdfmt_value(field.get_value()),
            })
            .collect();
        format!("[{}]", items.join(","))
    }
    fn stdfmt_field(field: &DataField) -> String {
        if field.get_name().is_empty() {
            Self::stdfmt_value(field.get_value())
        } else {
            format!(
                "\"{}\":{}",
                field.get_name(),
                Self::stdfmt_value(field.get_value())
            )
        }
    }
    fn stdfmt_record(record: &DataRecord) -> String {
        let mut items = Vec::new();
        for field in record
            .items
            .iter()
            .filter(|f| *f.get_meta() != DataType::Ignore)
        {
            items.push(Self::stdfmt_field(field));
        }
        format!("{{{}}}", items.join(","))
    }
}

impl crate::formatter::DataFormat for Json {
    type Output = String;
    fn format_null(&self) -> String {
        Self::stdfmt_null()
    }
    fn format_bool(&self, v: &bool) -> String {
        Self::stdfmt_bool(v)
    }
    fn format_string(&self, v: &str) -> String {
        Self::stdfmt_string(v)
    }
    fn format_i64(&self, v: &i64) -> String {
        Self::stdfmt_i64(v)
    }
    fn format_f64(&self, v: &f64) -> String {
        Self::stdfmt_f64(v)
    }
    fn format_ip(&self, v: &std::net::IpAddr) -> String {
        Self::stdfmt_ip_addr(v)
    }
    fn format_datetime(&self, v: &chrono::NaiveDateTime) -> String {
        Self::stdfmt_datetime(v)
    }
    fn format_object(&self, v: &ObjectValue) -> String {
        Self::stdfmt_object(v)
    }
    fn format_array(&self, v: &[DataField]) -> String {
        Self::stdfmt_array(v)
    }
    fn format_field(&self, f: &DataField) -> String {
        Self::stdfmt_field(f)
    }
    fn format_record(&self, r: &DataRecord) -> String {
        Self::stdfmt_record(r)
    }
}

fn to_json_value(value: &Value) -> JsonValue {
    match value {
        Value::Bool(v) => JsonValue::Bool(*v),
        Value::Chars(v) => JsonValue::String(v.to_string()),
        Value::Digit(v) => JsonValue::Number((*v).into()),
        Value::Float(v) => {
            if v.is_nan() {
                JsonValue::Null
            } else if v.is_infinite() {
                if v.is_sign_positive() {
                    JsonValue::String("Infinity".to_string())
                } else {
                    JsonValue::String("-Infinity".to_string())
                }
            } else {
                JsonValue::Number(serde_json::Number::from_f64(*v).unwrap_or(0.into()))
            }
        }
        Value::IpAddr(v) => JsonValue::String(v.to_string()),
        Value::Time(v) => JsonValue::String(v.to_string()),
        Value::Obj(v) => {
            let mut map = serde_json::Map::new();
            for (k, field) in v.iter() {
                map.insert(k.to_string(), to_json_value(field.get_value()));
            }
            JsonValue::Object(map)
        }
        Value::Array(v) => {
            JsonValue::Array(v.iter().map(|f| to_json_value(f.get_value())).collect())
        }
        _ => JsonValue::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formatter::DataFormat;
    use std::net::IpAddr;
    use std::str::FromStr;

    #[test]
    fn test_json_stdfmt_null() {
        assert_eq!(Json::stdfmt_null(), "null");
    }

    #[test]
    fn test_json_stdfmt_bool() {
        assert_eq!(Json::stdfmt_bool(&true), "true");
        assert_eq!(Json::stdfmt_bool(&false), "false");
    }

    #[test]
    fn test_json_stdfmt_string() {
        assert_eq!(Json::stdfmt_string("hello"), "\"hello\"");
        assert_eq!(Json::stdfmt_string(""), "\"\"");
        // Special chars should be escaped
        assert_eq!(Json::stdfmt_string("say \"hi\""), "\"say \\\"hi\\\"\"");
    }

    #[test]
    fn test_json_stdfmt_i64() {
        assert_eq!(Json::stdfmt_i64(&0), "0");
        assert_eq!(Json::stdfmt_i64(&42), "42");
        assert_eq!(Json::stdfmt_i64(&-100), "-100");
    }

    #[test]
    fn test_json_stdfmt_f64_normal() {
        assert_eq!(Json::stdfmt_f64(&3.24), "3.24");
        assert_eq!(Json::stdfmt_f64(&0.0), "0.0");
        assert_eq!(Json::stdfmt_f64(&-2.5), "-2.5");
    }

    #[test]
    fn test_json_stdfmt_f64_special() {
        assert_eq!(Json::stdfmt_f64(&f64::NAN), "null");
        assert_eq!(Json::stdfmt_f64(&f64::INFINITY), "\"Infinity\"");
        assert_eq!(Json::stdfmt_f64(&f64::NEG_INFINITY), "\"-Infinity\"");
    }

    #[test]
    fn test_json_stdfmt_ip_addr() {
        let ipv4 = IpAddr::from_str("192.168.1.1").unwrap();
        assert_eq!(Json::stdfmt_ip_addr(&ipv4), "\"192.168.1.1\"");

        let ipv6 = IpAddr::from_str("::1").unwrap();
        assert_eq!(Json::stdfmt_ip_addr(&ipv6), "\"::1\"");
    }

    #[test]
    fn test_json_stdfmt_datetime() {
        let dt = chrono::NaiveDateTime::parse_from_str("2024-01-15 10:30:45", "%Y-%m-%d %H:%M:%S")
            .unwrap();
        let result = Json::stdfmt_datetime(&dt);
        assert!(result.starts_with('"'));
        assert!(result.ends_with('"'));
        assert!(result.contains("2024"));
    }

    #[test]
    fn test_json_stdfmt_field_with_name() {
        let field = DataField::from_chars("name", "Alice");
        let result = Json::stdfmt_field(&field);
        assert_eq!(result, "\"name\":\"Alice\"");
    }

    #[test]
    fn test_json_stdfmt_field_without_name() {
        let field = DataField::from_chars("", "value");
        let result = Json::stdfmt_field(&field);
        assert_eq!(result, "\"value\"");
    }

    #[test]
    fn test_json_stdfmt_record() {
        let record = DataRecord {
            items: vec![
                DataField::from_chars("name", "Alice"),
                DataField::from_digit("age", 30),
            ],
        };
        let result = Json::stdfmt_record(&record);
        assert!(result.starts_with('{'));
        assert!(result.ends_with('}'));
        assert!(result.contains("\"name\":\"Alice\""));
        assert!(result.contains("\"age\":30"));
    }

    #[test]
    fn test_json_dataformat_impl() {
        let json = Json;
        assert_eq!(json.format_null(), "null");
        assert_eq!(json.format_bool(&true), "true");
        assert_eq!(json.format_string("test"), "\"test\"");
        assert_eq!(json.format_i64(&42), "42");
        assert_eq!(json.format_f64(&3.24), "3.24");
    }

    #[test]
    fn test_to_json_value_basic() {
        assert_eq!(to_json_value(&Value::Bool(true)), JsonValue::Bool(true));
        assert_eq!(
            to_json_value(&Value::Chars("hi".into())),
            JsonValue::String("hi".into())
        );
        assert_eq!(
            to_json_value(&Value::Digit(42)),
            JsonValue::Number(42.into())
        );
    }

    #[test]
    fn test_to_json_value_float_special() {
        assert_eq!(to_json_value(&Value::Float(f64::NAN)), JsonValue::Null);
        assert_eq!(
            to_json_value(&Value::Float(f64::INFINITY)),
            JsonValue::String("Infinity".into())
        );
    }

    #[test]
    fn test_json_stdfmt_array() {
        let arr = vec![
            DataField::from_digit("", 1),
            DataField::from_digit("", 2),
            DataField::from_digit("", 3),
        ];
        let result = Json::stdfmt_array(&arr);
        assert_eq!(result, "[1,2,3]");
    }
}
