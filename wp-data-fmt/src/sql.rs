use crate::formatter::DataFormat;
use wp_model_core::model::fmt_def::TextFmt;
use wp_model_core::model::{DataField, DataRecord, DataType, Value, types::value::ObjectValue};

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

    #[test]
    fn test_sql_default() {
        let sql = SqlInsert::default();
        assert_eq!(sql.table_name, "");
        assert!(sql.quote_identifiers);
    }

    #[test]
    fn test_sql_new_with_json() {
        let sql = SqlInsert::new_with_json("users");
        assert_eq!(sql.table_name, "users");
        assert!(sql.quote_identifiers);
    }

    #[test]
    fn test_format_null() {
        let sql = SqlInsert::default();
        assert_eq!(sql.format_null(), "NULL");
    }

    #[test]
    fn test_format_bool() {
        let sql = SqlInsert::default();
        assert_eq!(sql.format_bool(&true), "TRUE");
        assert_eq!(sql.format_bool(&false), "FALSE");
    }

    #[test]
    fn test_format_string() {
        let sql = SqlInsert::default();
        assert_eq!(sql.format_string("hello"), "'hello'");
        assert_eq!(sql.format_string(""), "''");
    }

    #[test]
    fn test_format_string_escape() {
        let sql = SqlInsert::default();
        // Single quotes should be escaped by doubling
        assert_eq!(sql.format_string("it's"), "'it''s'");
        assert_eq!(sql.format_string("say 'hi'"), "'say ''hi'''");
    }

    #[test]
    fn test_format_i64() {
        let sql = SqlInsert::default();
        assert_eq!(sql.format_i64(&0), "0");
        assert_eq!(sql.format_i64(&42), "42");
        assert_eq!(sql.format_i64(&-100), "-100");
    }

    #[test]
    fn test_format_f64_normal() {
        let sql = SqlInsert::default();
        assert_eq!(sql.format_f64(&3.24), "3.24");
        assert_eq!(sql.format_f64(&0.0), "0");
    }

    #[test]
    fn test_format_f64_special() {
        let sql = SqlInsert::default();
        assert_eq!(sql.format_f64(&f64::NAN), "NULL");
        assert_eq!(sql.format_f64(&f64::INFINITY), "'Infinity'");
        assert_eq!(sql.format_f64(&f64::NEG_INFINITY), "'-Infinity'");
    }

    #[test]
    fn test_format_ip() {
        use std::net::IpAddr;
        use std::str::FromStr;
        let sql = SqlInsert::default();
        let ip = IpAddr::from_str("192.168.1.1").unwrap();
        assert_eq!(sql.format_ip(&ip), "'192.168.1.1'");
    }

    #[test]
    fn test_format_datetime() {
        let sql = SqlInsert::default();
        let dt = chrono::NaiveDateTime::parse_from_str("2024-01-15 10:30:45", "%Y-%m-%d %H:%M:%S")
            .unwrap();
        let result = sql.format_datetime(&dt);
        assert!(result.starts_with('\''));
        assert!(result.ends_with('\''));
        assert!(result.contains("2024"));
    }

    #[test]
    fn test_quote_identifier() {
        let sql = SqlInsert::new_with_json("t");
        assert_eq!(sql.quote_identifier("name"), "\"name\"");
        assert_eq!(sql.quote_identifier("user_id"), "\"user_id\"");
    }

    #[test]
    fn test_quote_identifier_escape() {
        let sql = SqlInsert::new_with_json("t");
        // Double quotes in identifier should be escaped by doubling
        assert_eq!(sql.quote_identifier("col\"name"), "\"col\"\"name\"");
    }

    #[test]
    fn test_quote_identifier_disabled() {
        let sql = SqlInsert {
            table_name: "t".into(),
            quote_identifiers: false,
            obj_formatter: crate::SqlFormat::from(&TextFmt::Json),
        };
        assert_eq!(sql.quote_identifier("name"), "name");
    }

    #[test]
    fn test_format_record() {
        let sql = SqlInsert::new_with_json("users");
        let record = DataRecord {
            items: vec![
                DataField::from_chars("name", "Alice"),
                DataField::from_digit("age", 30),
                DataField::from_bool("active", true),
            ],
        };
        let result = sql.format_record(&record);
        assert!(result.starts_with("INSERT INTO \"users\""));
        assert!(result.contains("(\"name\", \"age\", \"active\")"));
        assert!(result.contains("VALUES ('Alice', 30, TRUE)"));
        assert!(result.ends_with(';'));
    }

    #[test]
    fn test_format_batch_empty() {
        let sql = SqlInsert::new_with_json("users");
        let records: Vec<DataRecord> = vec![];
        assert_eq!(sql.format_batch(&records), "");
    }

    #[test]
    fn test_format_batch() {
        let sql = SqlInsert::new_with_json("users");
        let records = vec![
            DataRecord {
                items: vec![
                    DataField::from_chars("name", "Alice"),
                    DataField::from_digit("age", 30),
                ],
            },
            DataRecord {
                items: vec![
                    DataField::from_chars("name", "Bob"),
                    DataField::from_digit("age", 25),
                ],
            },
        ];
        let result = sql.format_batch(&records);
        assert!(result.contains("INSERT INTO \"users\""));
        assert!(result.contains("('Alice', 30)"));
        assert!(result.contains("('Bob', 25)"));
        assert!(result.ends_with(';'));
    }

    #[test]
    fn test_generate_create_table_empty() {
        let sql = SqlInsert::new_with_json("users");
        let records: Vec<DataRecord> = vec![];
        assert_eq!(sql.generate_create_table(&records), "");
    }

    #[test]
    fn test_generate_create_table() {
        let sql = SqlInsert::new_with_json("users");
        let records = vec![DataRecord {
            items: vec![
                DataField::from_chars("name", "Alice"),
                DataField::from_digit("age", 30),
                DataField::from_bool("active", true),
                DataField::from_float("score", 95.5),
            ],
        }];
        let result = sql.generate_create_table(&records);
        assert!(result.contains("CREATE TABLE IF NOT EXISTS \"users\""));
        assert!(result.contains("\"name\" TEXT"));
        assert!(result.contains("\"age\" BIGINT"));
        assert!(result.contains("\"active\" BOOLEAN"));
        assert!(result.contains("\"score\" DOUBLE PRECISION"));
    }

    #[test]
    fn test_format_upsert() {
        let sql = SqlInsert::new_with_json("users");
        let record = DataRecord {
            items: vec![
                DataField::from_chars("id", "u1"),
                DataField::from_chars("name", "Alice"),
                DataField::from_digit("age", 30),
            ],
        };
        let result = sql.format_upsert(&record, &["id"]);
        assert!(result.contains("INSERT INTO \"users\""));
        assert!(result.contains("ON CONFLICT (\"id\")"));
        assert!(result.contains("DO UPDATE SET"));
        assert!(result.contains("\"name\" = EXCLUDED.\"name\""));
        assert!(result.contains("\"age\" = EXCLUDED.\"age\""));
    }

    #[test]
    fn test_format_upsert_no_update_columns() {
        let sql = SqlInsert::new_with_json("users");
        let record = DataRecord {
            items: vec![DataField::from_chars("id", "u1")],
        };
        // When all columns are conflict columns, no update is needed
        let result = sql.format_upsert(&record, &["id"]);
        // Should just be a regular insert with semicolon
        assert!(result.contains("INSERT INTO"));
        assert!(!result.contains("ON CONFLICT"));
    }
}
