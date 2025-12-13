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
