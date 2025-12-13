use orion_error::ErrStrategy;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Formatter,
    sync::atomic::{AtomicU8, Ordering},
};
use wp_log::{error_ctrl, warn_ctrl};

pub enum ErrorHandlingStrategy {
    FixRetry,
    Tolerant,
    Throw,
    Ignore,
    Terminate,
}

impl From<ErrStrategy> for ErrorHandlingStrategy {
    fn from(value: ErrStrategy) -> Self {
        match value {
            ErrStrategy::Retry => ErrorHandlingStrategy::FixRetry,
            ErrStrategy::Ignore => ErrorHandlingStrategy::Ignore,
            ErrStrategy::Throw => ErrorHandlingStrategy::Throw,
        }
    }
}

static ROBUST_MODE: AtomicU8 = AtomicU8::new(0);
//static mut ROBUST_MODE: RobustnessMode = RobustnessMode::Debug;

pub fn sys_robust_mode() -> RobustnessMode {
    match ROBUST_MODE.load(Ordering::SeqCst) {
        0 => RobustnessMode::Debug,
        1 => RobustnessMode::Normal,
        2 => RobustnessMode::Strict,
        _ => unreachable!("Invalid robustness mode"),
    }
}

pub fn switch_sys_robust_mode(mode: RobustnessMode) -> RobustnessMode {
    let old = sys_robust_mode();
    let new_mode = match mode {
        RobustnessMode::Debug => 0,
        RobustnessMode::Normal => 1,
        RobustnessMode::Strict => 2,
    };
    // runtime-level change is part of engine control plane
    warn_ctrl!("switch robust mode from {} to {}", old, mode);
    ROBUST_MODE.store(new_mode, Ordering::SeqCst);
    old
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize, Clone)]
pub enum RobustnessMode {
    #[default]
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "strict")]
    Strict,
}

impl From<&str> for RobustnessMode {
    fn from(s: &str) -> Self {
        match s {
            "debug" => RobustnessMode::Debug,
            "normal" => RobustnessMode::Normal,
            "strict" => RobustnessMode::Strict,
            other => {
                // Fallback to debug to avoid panic in production; log a warning.
                error_ctrl!("unknown robust mode '{}', fallback to 'debug'", other);
                RobustnessMode::Debug
            }
        }
    }
}

impl std::fmt::Display for RobustnessMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RobustnessMode::Debug => write!(f, "debug"),
            RobustnessMode::Normal => write!(f, "normal"),
            RobustnessMode::Strict => write!(f, "strict"),
        }
    }
}
