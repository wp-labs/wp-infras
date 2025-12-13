//! Lightweight mapping from system error code (SysErrorCode) to HTTP status.
//! This module is framework-agnostic and does not depend on any HTTP stack.
use crate::SysErrorCode;

/// Map a system error code (u16) to an HTTP status code (u16).
/// Suggested contract:
/// - 404xx -> 404
/// - 422xx -> 422
/// - 204xx -> 204
/// - 499xx -> 499 (Client Closed Request) or 503; we choose 499 to distinguish
/// - 502xx -> 502
/// - 503xx -> 503
/// - 500xx -> 500
/// - else -> 500
pub fn http_status_for_sys(sys: u16) -> u16 {
    let head = sys / 100; // e.g., 42211 -> 422
    match head {
        404 => 404,
        422 => 422,
        204 => 204,
        499 => 499,
        502 => 502,
        503 => 503,
        500 => 500,
        _ => 500,
    }
}

/// Convenience: map a reason to HTTP status.
pub fn http_status_for_reason<R: SysErrorCode>(r: &R) -> u16 {
    http_status_for_sys(r.sys_code())
}
