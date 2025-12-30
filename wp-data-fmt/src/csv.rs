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

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::IpAddr;
    use std::str::FromStr;

    #[test]
    fn test_csv_default() {
        let csv = Csv::default();
        assert_eq!(csv.delimiter, ',');
        assert_eq!(csv.quote_char, '"');
        assert_eq!(csv.escape_char, '"');
    }

    #[test]
    fn test_csv_new() {
        let csv = Csv::new();
        assert_eq!(csv.delimiter, ',');
    }

    #[test]
    fn test_csv_builder_pattern() {
        let csv = Csv::new()
            .with_delimiter(';')
            .with_quote_char('\'')
            .with_escape_char('\\');
        assert_eq!(csv.delimiter, ';');
        assert_eq!(csv.quote_char, '\'');
        assert_eq!(csv.escape_char, '\\');
    }

    #[test]
    fn test_format_null() {
        let csv = Csv::default();
        assert_eq!(csv.format_null(), "");
    }

    #[test]
    fn test_format_bool() {
        let csv = Csv::default();
        assert_eq!(csv.format_bool(&true), "true");
        assert_eq!(csv.format_bool(&false), "false");
    }

    #[test]
    fn test_format_string_simple() {
        let csv = Csv::default();
        assert_eq!(csv.format_string("hello"), "hello");
        assert_eq!(csv.format_string("world"), "world");
    }

    #[test]
    fn test_format_string_with_delimiter() {
        let csv = Csv::default();
        // String containing delimiter should be quoted
        assert_eq!(csv.format_string("hello,world"), "\"hello,world\"");
    }

    #[test]
    fn test_format_string_with_newline() {
        let csv = Csv::default();
        assert_eq!(csv.format_string("hello\nworld"), "\"hello\nworld\"");
        assert_eq!(csv.format_string("hello\rworld"), "\"hello\rworld\"");
    }

    #[test]
    fn test_format_string_with_quote() {
        let csv = Csv::default();
        // Quote char should be escaped by doubling
        assert_eq!(csv.format_string("say \"hello\""), "\"say \"\"hello\"\"\"");
    }

    #[test]
    fn test_format_i64() {
        let csv = Csv::default();
        assert_eq!(csv.format_i64(&0), "0");
        assert_eq!(csv.format_i64(&42), "42");
        assert_eq!(csv.format_i64(&-100), "-100");
        assert_eq!(csv.format_i64(&i64::MAX), i64::MAX.to_string());
    }

    #[test]
    fn test_format_f64() {
        let csv = Csv::default();
        assert_eq!(csv.format_f64(&0.0), "0");
        assert_eq!(csv.format_f64(&3.24), "3.24");
        assert_eq!(csv.format_f64(&-2.5), "-2.5");
    }

    #[test]
    fn test_format_ip() {
        let csv = Csv::default();
        let ipv4 = IpAddr::from_str("192.168.1.1").unwrap();
        assert_eq!(csv.format_ip(&ipv4), "192.168.1.1");

        let ipv6 = IpAddr::from_str("::1").unwrap();
        assert_eq!(csv.format_ip(&ipv6), "::1");
    }

    #[test]
    fn test_format_datetime() {
        let csv = Csv::default();
        let dt = chrono::NaiveDateTime::parse_from_str("2024-01-15 10:30:45", "%Y-%m-%d %H:%M:%S")
            .unwrap();
        let result = csv.format_datetime(&dt);
        assert!(result.contains("2024"));
        assert!(result.contains("01"));
        assert!(result.contains("15"));
    }

    #[test]
    fn test_format_record() {
        let csv = Csv::default();
        let record = DataRecord {
            items: vec![
                DataField::from_chars("name", "Alice"),
                DataField::from_digit("age", 30),
            ],
        };
        let result = csv.format_record(&record);
        assert_eq!(result, "Alice,30");
    }

    #[test]
    fn test_format_record_with_custom_delimiter() {
        let csv = Csv::new().with_delimiter(';');
        let record = DataRecord {
            items: vec![
                DataField::from_chars("a", "x"),
                DataField::from_chars("b", "y"),
            ],
        };
        let result = csv.format_record(&record);
        assert_eq!(result, "x;y");
    }

    #[test]
    fn test_format_record_with_special_chars() {
        let csv = Csv::default();
        let record = DataRecord {
            items: vec![
                DataField::from_chars("msg", "hello,world"),
                DataField::from_digit("count", 5),
            ],
        };
        let result = csv.format_record(&record);
        assert!(result.contains("\"hello,world\""));
    }
}
