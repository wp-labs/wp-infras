use crate::formatter::DataFormat;
use wp_model_core::model::{types::value::ObjectValue, DataField, DataRecord, DataType, Value};

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
