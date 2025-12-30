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
use crate::KnowledgeReason;
use crate::config_error::{ConfCore, ConfDynamic, ConfFeature, ConfReason};
use crate::parse_error::{DataErrKind, OMLCodeReason};

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

#[cfg(test)]
mod tests {
    use super::*;
    use orion_error::{UvsLogicFrom, UvsReason};
    use std::marker::PhantomData;

    #[test]
    fn test_sys_error_code_default_tag() {
        struct DummyReason;
        impl SysErrorCode for DummyReason {
            fn sys_code(&self) -> u16 {
                12345
            }
        }
        let dummy = DummyReason;
        assert_eq!(dummy.sys_tag(), "wp.err");
        assert_eq!(dummy.sys_code(), 12345);
    }

    // ConfReason<ConfCore> tests
    #[test]
    fn test_conf_core_syntax_code() {
        let reason: ConfReason<ConfCore> = ConfReason::Syntax("test".into());
        assert_eq!(reason.sys_code(), 42201);
        assert_eq!(reason.sys_tag(), "conf.core");
    }

    #[test]
    fn test_conf_core_not_found_code() {
        let reason: ConfReason<ConfCore> = ConfReason::NotFound("missing".into());
        assert_eq!(reason.sys_code(), 40401);
    }

    #[test]
    fn test_conf_core_uvs_code() {
        let reason: ConfReason<ConfCore> =
            ConfReason::Uvs(UvsReason::from_logic("test".to_string()));
        assert_eq!(reason.sys_code(), 50001);
    }

    #[test]
    fn test_conf_core_take_code() {
        let reason: ConfReason<ConfCore> = ConfReason::_Take(PhantomData);
        assert_eq!(reason.sys_code(), 50009);
    }

    // ConfReason<ConfFeature> tests
    #[test]
    fn test_conf_feature_syntax_code() {
        let reason: ConfReason<ConfFeature> = ConfReason::Syntax("test".into());
        assert_eq!(reason.sys_code(), 42202);
        assert_eq!(reason.sys_tag(), "conf.feature");
    }

    #[test]
    fn test_conf_feature_not_found_code() {
        let reason: ConfReason<ConfFeature> = ConfReason::NotFound("missing".into());
        assert_eq!(reason.sys_code(), 40402);
    }

    #[test]
    fn test_conf_feature_uvs_code() {
        let reason: ConfReason<ConfFeature> =
            ConfReason::Uvs(UvsReason::from_logic("test".to_string()));
        assert_eq!(reason.sys_code(), 50002);
    }

    // ConfReason<ConfDynamic> tests
    #[test]
    fn test_conf_dynamic_syntax_code() {
        let reason: ConfReason<ConfDynamic> = ConfReason::Syntax("test".into());
        assert_eq!(reason.sys_code(), 42203);
        assert_eq!(reason.sys_tag(), "conf.dynamic");
    }

    #[test]
    fn test_conf_dynamic_not_found_code() {
        let reason: ConfReason<ConfDynamic> = ConfReason::NotFound("missing".into());
        assert_eq!(reason.sys_code(), 40403);
    }

    #[test]
    fn test_conf_dynamic_uvs_code() {
        let reason: ConfReason<ConfDynamic> =
            ConfReason::Uvs(UvsReason::from_logic("test".to_string()));
        assert_eq!(reason.sys_code(), 50003);
    }

    // OMLCodeReason tests
    #[test]
    fn test_oml_syntax_code() {
        let reason = OMLCodeReason::Syntax("parse error".into());
        assert_eq!(reason.sys_code(), 42211);
        assert_eq!(reason.sys_tag(), "parse.oml");
    }

    #[test]
    fn test_oml_not_found_code() {
        let reason = OMLCodeReason::NotFound("file.oml".into());
        assert_eq!(reason.sys_code(), 40411);
    }

    #[test]
    fn test_oml_uvs_code() {
        let reason = OMLCodeReason::Uvs(UvsReason::from_logic("test".to_string()));
        assert_eq!(reason.sys_code(), 50011);
    }

    // DataErrKind tests
    #[test]
    fn test_data_err_format_error_code() {
        let reason = DataErrKind::FormatError("bad format".into(), None);
        assert_eq!(reason.sys_code(), 42212);
        assert_eq!(reason.sys_tag(), "parse.data");
    }

    #[test]
    fn test_data_err_not_complete_code() {
        let reason = DataErrKind::NotComplete;
        assert_eq!(reason.sys_code(), 42213);
    }

    #[test]
    fn test_data_err_unparse_code() {
        let reason = DataErrKind::UnParse("unparseable".into());
        assert_eq!(reason.sys_code(), 40412);
    }

    #[test]
    fn test_data_err_less_data_code() {
        let reason = DataErrKind::LessData;
        assert_eq!(reason.sys_code(), 42214);
    }

    #[test]
    fn test_data_err_empty_data_code() {
        let reason = DataErrKind::EmptyData;
        assert_eq!(reason.sys_code(), 42215);
    }

    #[test]
    fn test_data_err_less_stc_code() {
        let reason = DataErrKind::LessStc("struct".into());
        assert_eq!(reason.sys_code(), 42216);
    }

    #[test]
    fn test_data_err_less_def_code() {
        let reason = DataErrKind::LessDef("define".into());
        assert_eq!(reason.sys_code(), 42217);
    }

    // SourceReason tests
    #[test]
    fn test_source_not_data_code() {
        let reason = SourceReason::NotData;
        assert_eq!(reason.sys_code(), 20401);
        assert_eq!(reason.sys_tag(), "source");
    }

    #[test]
    fn test_source_eof_code() {
        let reason = SourceReason::EOF;
        assert_eq!(reason.sys_code(), 20402);
    }

    #[test]
    fn test_source_supplier_error_code() {
        let reason = SourceReason::SupplierError("supplier failed".into());
        assert_eq!(reason.sys_code(), 50201);
    }

    #[test]
    fn test_source_disconnect_code() {
        let reason = SourceReason::Disconnect("connection lost".into());
        assert_eq!(reason.sys_code(), 49901);
    }

    #[test]
    fn test_source_other_code() {
        let reason = SourceReason::Other("unknown".into());
        assert_eq!(reason.sys_code(), 50209);
    }

    #[test]
    fn test_source_uvs_code() {
        let reason = SourceReason::Uvs(UvsReason::from_logic("test".to_string()));
        assert_eq!(reason.sys_code(), 50021);
    }

    // SinkReason tests
    #[test]
    fn test_sink_sink_code() {
        let reason = SinkReason::Sink("sink failed".into());
        assert_eq!(reason.sys_code(), 50211);
        assert_eq!(reason.sys_tag(), "dist");
    }

    #[test]
    fn test_sink_mock_code() {
        let reason = SinkReason::Mock;
        assert_eq!(reason.sys_code(), 50312);
    }

    #[test]
    fn test_sink_stg_ctrl_code() {
        let reason = SinkReason::StgCtrl;
        assert_eq!(reason.sys_code(), 50311);
    }

    #[test]
    fn test_sink_uvs_code() {
        let reason = SinkReason::Uvs(UvsReason::from_logic("test".to_string()));
        assert_eq!(reason.sys_code(), 50031);
    }

    // KnowledgeReason tests
    #[test]
    fn test_knowledge_not_data_code() {
        let reason = KnowledgeReason::NotData;
        assert_eq!(reason.sys_code(), 50042);
        assert_eq!(reason.sys_tag(), "knowledge");
    }

    #[test]
    fn test_knowledge_uvs_code() {
        let reason = KnowledgeReason::Uvs(UvsReason::from_logic("test".to_string()));
        assert_eq!(reason.sys_code(), 50041);
    }

    // RunReason tests
    #[test]
    fn test_run_dist_sink_error_code() {
        let reason = RR::Dist(DistFocus::SinkError("error".into()));
        assert_eq!(reason.sys_code(), 50211);
        assert_eq!(reason.sys_tag(), "run");
    }

    #[test]
    fn test_run_dist_stg_ctrl_code() {
        let reason = RR::Dist(DistFocus::StgCtrl);
        assert_eq!(reason.sys_code(), 50311);
    }

    #[test]
    fn test_run_source_no_data_code() {
        let reason = RR::Source(SourceFocus::NoData);
        assert_eq!(reason.sys_code(), 20401);
    }

    #[test]
    fn test_run_source_eof_code() {
        let reason = RR::Source(SourceFocus::Eof);
        assert_eq!(reason.sys_code(), 20402);
    }

    #[test]
    fn test_run_source_supplier_error_code() {
        let reason = RR::Source(SourceFocus::SupplierError("error".into()));
        assert_eq!(reason.sys_code(), 50201);
    }

    #[test]
    fn test_run_source_other_code() {
        let reason = RR::Source(SourceFocus::Other("other".into()));
        assert_eq!(reason.sys_code(), 50209);
    }

    #[test]
    fn test_run_source_disconnect_code() {
        let reason = RR::Source(SourceFocus::Disconnect("disconnect".into()));
        assert_eq!(reason.sys_code(), 49901);
    }

    #[test]
    fn test_run_uvs_code() {
        let reason = RR::Uvs(UvsReason::from_logic("test".to_string()));
        assert_eq!(reason.sys_code(), 50041);
    }
}
