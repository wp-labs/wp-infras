use crate::formatter::DataFormat;
use std::fmt::Write;
use wp_model_core::model::{DataField, DataRecord, DataType, types::value::ObjectValue};

pub struct Csv {
    delimiter: char,
    quote_char: char,
    escape_char: char,
}

impl Default for Csv {
    fn default() -> Self {
        Self {
            delimiter: ',',
            quote_char: '"',
            escape_char: '"',
        }
    }
}

impl Csv {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_delimiter(mut self, delimiter: char) -> Self {
        self.delimiter = delimiter;
        self
    }
    pub fn with_quote_char(mut self, quote_char: char) -> Self {
        self.quote_char = quote_char;
        self
    }
    pub fn with_escape_char(mut self, escape_char: char) -> Self {
        self.escape_char = escape_char;
        self
    }

    fn escape_string(&self, value: &str, output: &mut String) {
        let needs_quoting = value.contains(self.delimiter)
            || value.contains('\n')
            || value.contains('\r')
            || value.contains(self.quote_char);
        if needs_quoting {
            output.push(self.quote_char);
            for c in value.chars() {
                if c == self.quote_char {
                    output.push(self.escape_char);
                }
                output.push(c);
            }
            output.push(self.quote_char);
        } else {
            output.push_str(value);
        }
    }
}
impl DataFormat for Csv {
    type Output = String;
    fn format_null(&self) -> String {
        "".to_string()
    }
    fn format_bool(&self, value: &bool) -> String {
        if *value { "true" } else { "false" }.to_string()
    }
    fn format_string(&self, value: &str) -> String {
        let mut o = String::new();
        self.escape_string(value, &mut o);
        o
    }
    fn format_i64(&self, value: &i64) -> String {
        value.to_string()
    }
    fn format_f64(&self, value: &f64) -> String {
        value.to_string()
    }
    fn format_ip(&self, value: &std::net::IpAddr) -> String {
        self.format_string(&value.to_string())
    }
    fn format_datetime(&self, value: &chrono::NaiveDateTime) -> String {
        self.format_string(&value.to_string())
    }
    fn format_object(&self, value: &ObjectValue) -> String {
        let mut output = String::new();
        for (i, (k, v)) in value.iter().enumerate() {
            if i > 0 {
                output.push_str(", ");
            }
            write!(output, "{}:{}", k, self.fmt_value(v.get_value())).unwrap();
        }
        output
    }
    fn format_array(&self, value: &[DataField]) -> String {
        let mut output = String::new();
        self.escape_string(
            &value
                .iter()
                .map(|f| self.format_field(f))
                .collect::<Vec<_>>()
                .join(", "),
            &mut output,
        );
        output
    }
    fn format_field(&self, field: &DataField) -> String {
        self.fmt_value(field.get_value())
    }
    fn format_record(&self, record: &DataRecord) -> String {
        let mut output = String::new();
        let mut first = true;
        for field in record
            .items
            .iter()
            .filter(|f| *f.get_meta() != DataType::Ignore)
        {
            if !first {
                output.push(self.delimiter);
            }
            first = false;
            output.push_str(&self.format_field(field));
        }
        output
    }
}
