use crate::formatter::DataFormat;
use wp_model_core::model::fmt_def::TextFmt;
use wp_model_core::model::{types::value::ObjectValue, DataField, DataRecord, DataType, Value};

pub struct SqlInsert {
    pub table_name: String,
    pub quote_identifiers: bool,
    pub obj_formatter: crate::SqlFormat,
}

impl Default for SqlInsert {
    fn default() -> Self {
        Self {
            table_name: String::new(),
            quote_identifiers: true,
            obj_formatter: crate::SqlFormat::from(&TextFmt::Json),
        }
    }
}

impl SqlInsert {
    pub fn new_with_json<T: Into<String>>(table: T) -> Self {
        Self {
            table_name: table.into(),
            quote_identifiers: true,
            obj_formatter: crate::SqlFormat::from(&TextFmt::Json),
        }
    }
    fn quote_identifier(&self, name: &str) -> String {
        if self.quote_identifiers {
            let escaped = name.replace('"', "\"\"");
            format!("\"{}\"", escaped)
        } else {
            name.to_string()
        }
    }
    fn escape_string(&self, value: &str) -> String {
        value.replace('\'', "''")
    }
}

impl DataFormat for SqlInsert {
    type Output = String;
    fn format_null(&self) -> String {
        "NULL".to_string()
    }
    fn format_bool(&self, value: &bool) -> String {
        if *value { "TRUE" } else { "FALSE" }.to_string()
    }
    fn format_string(&self, value: &str) -> String {
        format!("'{}'", self.escape_string(value))
    }
    fn format_i64(&self, value: &i64) -> String {
        value.to_string()
    }
    fn format_f64(&self, value: &f64) -> String {
        if value.is_nan() {
            "NULL".into()
        } else if value.is_infinite() {
            if value.is_sign_positive() {
                "'Infinity'".into()
            } else {
                "'-Infinity'".into()
            }
        } else {
            value.to_string()
        }
    }
    fn format_ip(&self, value: &std::net::IpAddr) -> String {
        self.format_string(&value.to_string())
    }
    fn format_datetime(&self, value: &chrono::NaiveDateTime) -> String {
        self.format_string(&value.to_string())
    }
    fn format_object(&self, value: &ObjectValue) -> String {
        let inner = match &self.obj_formatter {
            crate::SqlFormat::Json(f) => f.format_object(value),
            crate::SqlFormat::Kv(f) => f.format_object(value),
            crate::SqlFormat::Raw(f) => f.format_object(value),
            crate::SqlFormat::ProtoText(f) => f.format_object(value),
        };
        format!("'{}'", self.escape_string(&inner))
    }
    fn format_array(&self, value: &[DataField]) -> String {
        let inner = match &self.obj_formatter {
            crate::SqlFormat::Json(f) => f.format_array(value),
            crate::SqlFormat::Kv(f) => f.format_array(value),
            crate::SqlFormat::Raw(f) => f.format_array(value),
            crate::SqlFormat::ProtoText(f) => f.format_array(value),
        };
        format!("'{}'", self.escape_string(&inner))
    }
    fn format_record(&self, record: &DataRecord) -> String {
        let columns: Vec<String> = record
            .items
            .iter()
            .filter(|f| *f.get_meta() != DataType::Ignore)
            .map(|f| self.quote_identifier(f.get_name()))
            .collect();
        let values: Vec<String> = record
            .items
            .iter()
            .filter(|f| *f.get_meta() != DataType::Ignore)
            .map(|f| self.format_field(f))
            .collect();
        format!(
            "INSERT INTO {} ({}) VALUES ({});",
            self.quote_identifier(&self.table_name),
            columns.join(", "),
            values.join(", ")
        )
    }
    fn format_field(&self, field: &DataField) -> String {
        if *field.get_meta() == DataType::Ignore {
            String::new()
        } else {
            self.fmt_value(field.get_value())
        }
    }
}

impl SqlInsert {
    pub fn format_batch(&self, records: &[DataRecord]) -> String {
        if records.is_empty() {
            return String::new();
        }
        let mut output = String::new();
        let columns: Vec<String> = records[0]
            .items
            .iter()
            .filter(|f| *f.get_meta() != DataType::Ignore)
            .map(|f| self.quote_identifier(f.get_name()))
            .collect();
        use std::fmt::Write;
        writeln!(
            output,
            "INSERT INTO {} ({}) VALUES",
            self.quote_identifier(&self.table_name),
            columns.join(", ")
        )
        .unwrap();
        for (i, record) in records.iter().enumerate() {
            if i > 0 {
                output.push_str(",\n");
            }
            let values: Vec<String> = record
                .items
                .iter()
                .filter(|f| *f.get_meta() != DataType::Ignore)
                .map(|f| self.format_field(f))
                .collect();
            write!(output, "  ({})", values.join(", ")).unwrap();
        }
        output.push(';');
        output
    }
    pub fn generate_create_table(&self, records: &[DataRecord]) -> String {
        if records.is_empty() {
            return String::new();
        }
        let mut columns = Vec::new();
        for field in &records[0].items {
            if *field.get_meta() == DataType::Ignore {
                continue;
            }
            let sql_type = &match field.get_value() {
                Value::Bool(_) => "BOOLEAN",
                Value::Chars(_) => "TEXT",
                Value::Digit(_) => "BIGINT",
                Value::Float(_) => "DOUBLE PRECISION",
                Value::Time(_) => "TIMESTAMP",
                Value::IpAddr(_) => "INET",
                Value::Obj(_) | Value::Array(_) => "JSONB",
                _ => "TEXT",
            };
            columns.push(format!(
                "  {} {}",
                self.quote_identifier(field.get_name()),
                sql_type
            ));
        }
        format!(
            "CREATE TABLE IF NOT EXISTS {} (\n{}\n);",
            self.quote_identifier(&self.table_name),
            columns.join(",\n")
        )
    }
    pub fn format_upsert(&self, record: &DataRecord, conflict_columns: &[&str]) -> String {
        let insert = self.format_record(record);
        let mut update_parts = Vec::new();
        for field in record
            .items
            .iter()
            .filter(|f| *f.get_meta() != DataType::Ignore)
        {
            let name = field.get_name();
            if !conflict_columns.contains(&name) {
                let col = self.quote_identifier(name);
                update_parts.push(format!("{} = EXCLUDED.{}", &col, &col));
            }
        }
        if update_parts.is_empty() {
            insert
        } else {
            let quoted_conflicts: Vec<String> = conflict_columns
                .iter()
                .map(|c| self.quote_identifier(c))
                .collect();
            format!(
                "{} ON CONFLICT ({}) DO UPDATE SET {};",
                insert.trim_end_matches(';'),
                quoted_conflicts.join(", "),
                update_parts.join(", ")
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formatter::DataFormat;
    use wp_model_core::model::{DataField, DataRecord};
    #[test]
    fn test_sql_basic() {
        let f = SqlInsert {
            table_name: "t".into(),
            quote_identifiers: true,
            obj_formatter: crate::SqlFormat::from(&TextFmt::Json),
        };
        let r = DataRecord {
            items: vec![
                DataField::from_chars("name", "Alice"),
                DataField::from_digit("age", 30),
            ],
        };
        let s = f.format_record(&r);
        assert!(s.contains("INSERT INTO \"t\" (\"name\", \"age\") VALUES"));
    }
}
