use serde::{Deserialize, Serialize};

/// Ultra-light core specs shared across config and plugin layers.
/// Keep dependencies minimal and avoid tying to config-domain types.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CoreSinkSpec {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub params: ParamMap,
    #[serde(default)]
    pub filter: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// CoreSourceSpec：与 CoreSinkSpec 一致的极简规格（扁平 params，不包含运行期上下文）。
/// 用于 Sources 的统一 Factory 路径输入。
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CoreSourceSpec {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub params: ParamMap,
    #[serde(default)]
    pub tags: Vec<String>,
}

// Lightweight pattern helpers shared across config/spec layers
pub mod pattern;

// Re-export commonly used types at crate root for convenience
pub use pattern::WildArray;
use wp_connector_api::ParamMap;
