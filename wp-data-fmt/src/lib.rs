mod csv;
pub mod fmt_meta;
mod formatter;
mod json;
mod kv;
mod proto;
mod raw;
mod sql;

pub use csv::Csv;
pub use formatter::{DataFormat, StaticDataFormatter};
pub use json::Json;
pub use kv::KeyValue;
pub use proto::ProtoTxt;
pub use raw::Raw;
pub use sql::SqlInsert;

use wp_model_core::model::fmt_def::TextFmt;

pub enum FormatType {
    Json(Json),
    Csv(Csv),
    Kv(KeyValue),
    Sql(SqlInsert),
    Raw(Raw),
    ProtoText(ProtoTxt),
}

impl From<&TextFmt> for FormatType {
    fn from(fmt: &TextFmt) -> Self {
        match fmt {
            TextFmt::Json => FormatType::Json(Json),
            TextFmt::Csv => FormatType::Csv(Csv::default()),
            TextFmt::Kv => FormatType::Kv(KeyValue::default()),
            TextFmt::Raw => FormatType::Raw(Raw),
            TextFmt::ProtoText => FormatType::ProtoText(ProtoTxt),
            TextFmt::Show => FormatType::Raw(Raw),
            TextFmt::Proto => FormatType::ProtoText(ProtoTxt),
        }
    }
}

pub enum SqlFormat {
    Json(Json),
    Kv(KeyValue),
    Raw(Raw),
    ProtoText(ProtoTxt),
}

impl From<&TextFmt> for SqlFormat {
    fn from(fmt: &TextFmt) -> Self {
        match fmt {
            TextFmt::Json => SqlFormat::Json(Json),
            TextFmt::Kv => SqlFormat::Kv(KeyValue::default()),
            TextFmt::Raw => SqlFormat::Raw(Raw),
            TextFmt::ProtoText => SqlFormat::ProtoText(ProtoTxt),
            _ => SqlFormat::Raw(Raw),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use formatter::DataFormat;
    use wp_model_core::model::{DataField, DataRecord};

    #[test]
    fn test_format_type_from_text_fmt_json() {
        let fmt = FormatType::from(&TextFmt::Json);
        matches!(fmt, FormatType::Json(_));
    }

    #[test]
    fn test_format_type_from_text_fmt_csv() {
        let fmt = FormatType::from(&TextFmt::Csv);
        matches!(fmt, FormatType::Csv(_));
    }

    #[test]
    fn test_format_type_from_text_fmt_kv() {
        let fmt = FormatType::from(&TextFmt::Kv);
        matches!(fmt, FormatType::Kv(_));
    }

    #[test]
    fn test_format_type_from_text_fmt_raw() {
        let fmt = FormatType::from(&TextFmt::Raw);
        matches!(fmt, FormatType::Raw(_));
    }

    #[test]
    fn test_format_type_from_text_fmt_proto_text() {
        let fmt = FormatType::from(&TextFmt::ProtoText);
        matches!(fmt, FormatType::ProtoText(_));
    }

    #[test]
    fn test_format_type_from_text_fmt_show() {
        let fmt = FormatType::from(&TextFmt::Show);
        matches!(fmt, FormatType::Raw(_));
    }

    #[test]
    fn test_format_type_from_text_fmt_proto() {
        let fmt = FormatType::from(&TextFmt::Proto);
        matches!(fmt, FormatType::ProtoText(_));
    }

    #[test]
    fn test_sql_format_from_text_fmt_json() {
        let fmt = SqlFormat::from(&TextFmt::Json);
        matches!(fmt, SqlFormat::Json(_));
    }

    #[test]
    fn test_sql_format_from_text_fmt_kv() {
        let fmt = SqlFormat::from(&TextFmt::Kv);
        matches!(fmt, SqlFormat::Kv(_));
    }

    #[test]
    fn test_sql_format_from_text_fmt_raw() {
        let fmt = SqlFormat::from(&TextFmt::Raw);
        matches!(fmt, SqlFormat::Raw(_));
    }

    #[test]
    fn test_sql_format_from_text_fmt_proto_text() {
        let fmt = SqlFormat::from(&TextFmt::ProtoText);
        matches!(fmt, SqlFormat::ProtoText(_));
    }

    #[test]
    fn test_sql_format_from_text_fmt_csv_fallback() {
        // Csv is not supported in SqlFormat, should fallback to Raw
        let fmt = SqlFormat::from(&TextFmt::Csv);
        matches!(fmt, SqlFormat::Raw(_));
    }

    #[test]
    fn test_format_type_dataformat_null() {
        let fmt = FormatType::from(&TextFmt::Json);
        assert_eq!(fmt.format_null(), "null");

        let fmt = FormatType::from(&TextFmt::Csv);
        assert_eq!(fmt.format_null(), "");

        let fmt = FormatType::from(&TextFmt::Raw);
        assert_eq!(fmt.format_null(), "");
    }

    #[test]
    fn test_format_type_dataformat_bool() {
        let fmt = FormatType::from(&TextFmt::Json);
        assert_eq!(fmt.format_bool(&true), "true");
        assert_eq!(fmt.format_bool(&false), "false");
    }

    #[test]
    fn test_format_type_dataformat_string() {
        let json_fmt = FormatType::from(&TextFmt::Json);
        assert_eq!(json_fmt.format_string("hello"), "\"hello\"");

        let raw_fmt = FormatType::from(&TextFmt::Raw);
        assert_eq!(raw_fmt.format_string("hello"), "hello");
    }

    #[test]
    fn test_format_type_dataformat_i64() {
        let fmt = FormatType::from(&TextFmt::Json);
        assert_eq!(fmt.format_i64(&42), "42");
        assert_eq!(fmt.format_i64(&-100), "-100");
    }

    #[test]
    fn test_format_type_dataformat_f64() {
        let fmt = FormatType::from(&TextFmt::Json);
        assert_eq!(fmt.format_f64(&3.24), "3.24");
    }

    #[test]
    fn test_format_type_format_record() {
        let json_fmt = FormatType::from(&TextFmt::Json);
        let record = DataRecord {
            items: vec![
                DataField::from_chars("name", "Alice"),
                DataField::from_digit("age", 30),
            ],
        };
        let result = json_fmt.format_record(&record);
        assert!(result.starts_with('{'));
        assert!(result.ends_with('}'));
        assert!(result.contains("\"name\":\"Alice\""));
    }

    #[test]
    fn test_format_type_format_field() {
        let fmt = FormatType::from(&TextFmt::Json);
        let field = DataField::from_chars("key", "value");
        let result = fmt.format_field(&field);
        assert!(result.contains("key"));
        assert!(result.contains("value"));
    }

    #[test]
    fn test_csv_format_type() {
        let csv_fmt = FormatType::from(&TextFmt::Csv);
        let record = DataRecord {
            items: vec![
                DataField::from_chars("a", "x"),
                DataField::from_chars("b", "y"),
            ],
        };
        let result = csv_fmt.format_record(&record);
        assert_eq!(result, "x,y");
    }

    #[test]
    fn test_kv_format_type() {
        let kv_fmt = FormatType::from(&TextFmt::Kv);
        let record = DataRecord {
            items: vec![DataField::from_chars("name", "Alice")],
        };
        let result = kv_fmt.format_record(&record);
        assert!(result.contains("name"));
        assert!(result.contains("Alice"));
    }
}
