//! Helpers to build consistent error responses (JSON/text) without
//! coupling to any specific web framework.
use crate::{SysErrorCode, http_status_for_reason};
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Serialize)]
pub struct ErrorResponse<'a> {
    pub status: u16,
    pub sys_code: u16,
    pub tag: &'a str,
    pub message: String,
}

pub fn build_error_response<'a, R: SysErrorCode + Display>(r: &'a R) -> ErrorResponse<'a> {
    ErrorResponse {
        status: http_status_for_reason(r),
        sys_code: r.sys_code(),
        tag: r.sys_tag(),
        message: r.to_string(),
    }
}

pub fn error_response_json<R: SysErrorCode + Display>(r: &R) -> (u16, String) {
    let er = build_error_response(r);
    (
        er.status,
        serde_json::to_string(&er).unwrap_or_else(|_| {
            format!(
                "{{\"status\":{},\"sys_code\":{},\"tag\":\"{}\",\"message\":\"{}\"}}",
                er.status,
                er.sys_code,
                er.tag,
                er.message.replace('"', "'")
            )
        }),
    )
}

pub fn error_response_text<R: SysErrorCode + Display>(r: &R) -> (u16, String) {
    let er = build_error_response(r);
    (
        er.status,
        format!("[{}] {}: {}", er.tag, er.sys_code, er.message),
    )
}
