use crate::formatter::DataFormat;
use wp_model_core::model::types::value::ObjectValue;
use wp_model_core::model::{DataField, DataRecord, DataType, Value};

#[derive(Debug, Default)]
pub struct Raw;

impl Raw {
    pub fn new() -> Self {
        Self
    }
}

impl DataFormat for Raw {
    type Output = String;
    fn format_null(&self) -> String {
        String::new()
    }
    fn format_bool(&self, v: &bool) -> String {
        v.to_string()
    }
    fn format_string(&self, v: &str) -> String {
        v.to_string()
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
        if value.is_empty() {
            return "{}".to_string();
        }
        let segments: Vec<String> = value
            .iter()
            .map(|(k, v)| format!("{}={}", k, self.fmt_value(v.get_value())))
            .collect();
        format!("{{{}}}", segments.join(", "))
    }
    fn format_array(&self, value: &[DataField]) -> String {
        if value.is_empty() {
            return "[]".to_string();
        }
        let content: Vec<String> = value
            .iter()
            .map(|field| self.fmt_value(field.get_value()))
            .collect();
        format!("[{}]", content.join(", "))
    }
    fn format_field(&self, field: &DataField) -> String {
        match field.get_value() {
            Value::Chars(s) => s.clone(),
            _ => self.fmt_value(field.get_value()),
        }
    }
    fn format_record(&self, record: &DataRecord) -> String {
        record
            .items
            .iter()
            .filter(|f| *f.get_meta() != DataType::Ignore)
            .map(|f| self.format_field(f))
            .collect::<Vec<_>>()
            .join(" ")
    }
}
