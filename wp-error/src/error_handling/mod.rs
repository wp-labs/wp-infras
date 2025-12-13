mod strategy;
pub mod target;

pub use strategy::{
    switch_sys_robust_mode, sys_robust_mode, ErrorHandlingStrategy, RobustnessMode,
};
