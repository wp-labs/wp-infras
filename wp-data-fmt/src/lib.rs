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
            TextFmt::ProtoText => FormatType::ProtoText(ProtoTxt::default()),
            TextFmt::Show => FormatType::Raw(Raw),
            TextFmt::Proto => FormatType::ProtoText(ProtoTxt::default()),
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
            TextFmt::ProtoText => SqlFormat::ProtoText(ProtoTxt::default()),
            _ => SqlFormat::Raw(Raw),
        }
    }
}
