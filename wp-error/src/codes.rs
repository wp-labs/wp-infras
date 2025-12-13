//! System-wide error code contract for wp-error.
//! Provide a single place to convert domain reasons into stable numeric codes.

pub trait SysErrorCode {
    fn sys_code(&self) -> u16;
    fn sys_tag(&self) -> &'static str {
        "wp.err"
    }
}

use wp_connector_api::{SinkReason, SourceReason};

// ----------------- Config -----------------
use crate::config_error::{ConfCore, ConfDynamic, ConfFeature, ConfReason};
use crate::parse_error::{DataErrKind, OMLCodeReason};
use crate::KnowledgeReason;

impl SysErrorCode for ConfReason<ConfCore> {
    fn sys_code(&self) -> u16 {
        match self {
            ConfReason::Syntax(_) => 42201,
            ConfReason::NotFound(_) => 40401,
            ConfReason::Uvs(_) => 50001,
            ConfReason::_Take(_) => 50009,
        }
    }
    fn sys_tag(&self) -> &'static str {
        "conf.core"
    }
}

impl SysErrorCode for ConfReason<ConfFeature> {
    fn sys_code(&self) -> u16 {
        match self {
            ConfReason::Syntax(_) => 42202,
            ConfReason::NotFound(_) => 40402,
            ConfReason::Uvs(_) => 50002,
            ConfReason::_Take(_) => 50009,
        }
    }
    fn sys_tag(&self) -> &'static str {
        "conf.feature"
    }
}

impl SysErrorCode for ConfReason<ConfDynamic> {
    fn sys_code(&self) -> u16 {
        match self {
            ConfReason::Syntax(_) => 42203,
            ConfReason::NotFound(_) => 40403,
            ConfReason::Uvs(_) => 50003,
            ConfReason::_Take(_) => 50009,
        }
    }
    fn sys_tag(&self) -> &'static str {
        "conf.dynamic"
    }
}

// ----------------- Parse / OML -----------------
impl SysErrorCode for OMLCodeReason {
    fn sys_code(&self) -> u16 {
        match self {
            OMLCodeReason::Syntax(_) => 42211,
            OMLCodeReason::NotFound(_) => 40411,
            OMLCodeReason::Uvs(_) => 50011,
        }
    }
    fn sys_tag(&self) -> &'static str {
        "parse.oml"
    }
}

impl SysErrorCode for DataErrKind {
    fn sys_code(&self) -> u16 {
        match self {
            DataErrKind::FormatError(_, _) => 42212,
            DataErrKind::NotComplete => 42213,
            DataErrKind::UnParse(_) => 40412,
            DataErrKind::LessData => 42214,
            DataErrKind::EmptyData => 42215,
            DataErrKind::LessStc(_) => 42216,
            DataErrKind::LessDef(_) => 42217,
        }
    }
    fn sys_tag(&self) -> &'static str {
        "parse.data"
    }
}

// ----------------- Source -----------------
impl SysErrorCode for SourceReason {
    fn sys_code(&self) -> u16 {
        match self {
            SourceReason::NotData => 20401,
            SourceReason::EOF => 20402,
            SourceReason::SupplierError(_) => 50201,
            SourceReason::Disconnect(_) => 49901,
            SourceReason::Other(_) => 50209,
            SourceReason::Uvs(_) => 50021,
        }
    }
    fn sys_tag(&self) -> &'static str {
        "source"
    }
}

// ----------------- Dist -----------------
impl SysErrorCode for SinkReason {
    fn sys_code(&self) -> u16 {
        match self {
            SinkReason::Sink(_) => 50211,
            SinkReason::Mock => 50312,
            SinkReason::StgCtrl => 50311,
            SinkReason::Uvs(_) => 50031,
        }
    }
    fn sys_tag(&self) -> &'static str {
        "dist"
    }
}

// ----------------- Dist -----------------
impl SysErrorCode for KnowledgeReason {
    fn sys_code(&self) -> u16 {
        match self {
            KnowledgeReason::Uvs(_) => 50041,
            KnowledgeReason::NotData => 50042,
        }
    }
    fn sys_tag(&self) -> &'static str {
        "knowledge"
    }
}

// ----------------- Run (aggregate) -----------------
use crate::run_error::{DistFocus, RunReason as RR, SourceFocus};
impl SysErrorCode for RR {
    fn sys_code(&self) -> u16 {
        match self {
            RR::Dist(DistFocus::SinkError(_)) => 50211,
            RR::Dist(DistFocus::StgCtrl) => 50311,
            RR::Source(SourceFocus::NoData) => 20401,
            RR::Source(SourceFocus::Eof) => 20402,
            RR::Source(SourceFocus::SupplierError(_)) => 50201,
            RR::Source(SourceFocus::Other(_)) => 50209,
            RR::Source(SourceFocus::Disconnect(_)) => 49901,
            RR::Uvs(_) => 50041,
        }
    }
    fn sys_tag(&self) -> &'static str {
        "run"
    }
}
