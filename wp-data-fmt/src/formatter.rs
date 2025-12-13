use wp_model_core::model::{DataField, DataRecord, Value, types::value::ObjectValue};

use crate::{FormatType, SqlFormat};

pub trait DataFormat {
    type Output;

    fn format_null(&self) -> Self::Output;
    fn format_bool(&self, value: &bool) -> Self::Output;
    fn format_string(&self, value: &str) -> Self::Output;
    fn format_i64(&self, value: &i64) -> Self::Output;
    fn format_f64(&self, value: &f64) -> Self::Output;
    fn format_ip(&self, value: &std::net::IpAddr) -> Self::Output;
    fn format_datetime(&self, value: &chrono::NaiveDateTime) -> Self::Output;

    fn format_object(&self, value: &ObjectValue) -> Self::Output;
    fn format_array(&self, value: &[DataField]) -> Self::Output;

    fn fmt_value(&self, value: &Value) -> Self::Output {
        match value {
            Value::Null => self.format_null(),
            Value::Bool(v) => self.format_bool(v),
            Value::Chars(v) => self.format_string(v),
            Value::Digit(v) => self.format_i64(v),
            Value::Float(v) => self.format_f64(v),
            Value::IpAddr(v) => self.format_ip(v),
            Value::Time(v) => self.format_datetime(v),
            Value::Obj(v) => self.format_object(v),
            Value::Array(v) => self.format_array(v),
            _ => self.format_string(&value.to_string()),
        }
    }

    fn format_field(&self, field: &DataField) -> Self::Output;
    fn format_record(&self, record: &DataRecord) -> Self::Output;
}

pub trait StaticDataFormatter {
    type Output;

    fn stdfmt_null() -> Self::Output;
    fn stdfmt_bool(value: &bool) -> Self::Output;
    fn stdfmt_string(value: &str) -> Self::Output;
    fn stdfmt_i64(value: &i64) -> Self::Output;
    fn stdfmt_f64(value: &f64) -> Self::Output;
    fn stdfmt_ip_addr(value: &std::net::IpAddr) -> Self::Output;
    fn stdfmt_datetime(value: &chrono::NaiveDateTime) -> Self::Output;

    fn stdfmt_object(value: &ObjectValue) -> Self::Output;
    fn stdfmt_array(value: &[DataField]) -> Self::Output;

    fn stdfmt_value(value: &Value) -> Self::Output {
        match value {
            Value::Null => Self::stdfmt_null(),
            Value::Bool(v) => Self::stdfmt_bool(v),
            Value::Chars(v) => Self::stdfmt_string(v),
            Value::Digit(v) => Self::stdfmt_i64(v),
            Value::Float(v) => Self::stdfmt_f64(v),
            Value::IpAddr(v) => Self::stdfmt_ip_addr(v),
            Value::Time(v) => Self::stdfmt_datetime(v),
            Value::Obj(v) => Self::stdfmt_object(v),
            Value::Array(v) => Self::stdfmt_array(v),
            _ => Self::stdfmt_string(&value.to_string()),
        }
    }

    fn stdfmt_field(field: &DataField) -> Self::Output;
    fn stdfmt_record(record: &DataRecord) -> Self::Output;
}

trait AsDataFormatter {
    fn as_formatter(&self) -> &dyn DataFormat<Output = String>;
}
impl AsDataFormatter for FormatType {
    fn as_formatter(&self) -> &dyn DataFormat<Output = String> {
        match self {
            FormatType::Csv(f) => f,
            FormatType::Json(f) => f,
            FormatType::Kv(f) => f,
            FormatType::Sql(f) => f,
            FormatType::Raw(f) => f,
            FormatType::ProtoText(f) => f,
        }
    }
}

impl AsDataFormatter for SqlFormat {
    fn as_formatter(&self) -> &dyn DataFormat<Output = String> {
        match self {
            SqlFormat::Json(f) => f,
            SqlFormat::Kv(f) => f,
            SqlFormat::Raw(f) => f,
            SqlFormat::ProtoText(f) => f,
        }
    }
}
impl DataFormat for FormatType {
    type Output = String;
    fn format_null(&self) -> Self::Output {
        self.as_formatter().format_null()
    }
    fn format_bool(&self, value: &bool) -> Self::Output {
        self.as_formatter().format_bool(value)
    }
    fn format_string(&self, value: &str) -> Self::Output {
        self.as_formatter().format_string(value)
    }
    fn format_i64(&self, value: &i64) -> Self::Output {
        self.as_formatter().format_i64(value)
    }
    fn format_f64(&self, value: &f64) -> Self::Output {
        self.as_formatter().format_f64(value)
    }
    fn format_ip(&self, value: &std::net::IpAddr) -> Self::Output {
        self.as_formatter().format_ip(value)
    }
    fn format_datetime(&self, value: &chrono::NaiveDateTime) -> Self::Output {
        self.as_formatter().format_datetime(value)
    }
    fn format_object(&self, value: &ObjectValue) -> Self::Output {
        self.as_formatter().format_object(value)
    }
    fn format_array(&self, value: &[DataField]) -> Self::Output {
        self.as_formatter().format_array(value)
    }
    fn format_field(&self, field: &DataField) -> Self::Output {
        self.as_formatter().format_field(field)
    }
    fn format_record(&self, record: &DataRecord) -> Self::Output {
        self.as_formatter().format_record(record)
    }
}
