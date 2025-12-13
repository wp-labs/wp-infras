use crate::formatter::StaticDataFormatter;
use serde_json::{json, Value as JsonValue};
use wp_model_core::model::{types::value::ObjectValue, DataField, DataRecord, DataType, Value};

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
            json_obj.insert(k.clone(), to_json_value(v.get_value()));
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
        Value::Chars(v) => JsonValue::String(v.clone()),
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
                map.insert(k.clone(), to_json_value(field.get_value()));
            }
            JsonValue::Object(map)
        }
        Value::Array(v) => {
            JsonValue::Array(v.iter().map(|f| to_json_value(f.get_value())).collect())
        }
        _ => JsonValue::Null,
    }
}
