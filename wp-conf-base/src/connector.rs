use wp_connector_api::ParamMap;
use wp_model_core::model::fmt_def::TextFmt;

/// Adapter trait for connector kinds (e.g., "mysql", "kafka").
/// This trait provides a unified interface without involving a registry.
///
/// # Examples
///
/// ```rust
/// use serde_json::json;
/// use wp_conf_base::connector::ConnectorKindAdapter;
/// use wp_connector_api::ParamMap;
///
/// struct MysqlAdapter;
///
/// impl ConnectorKindAdapter for MysqlAdapter {
///     fn kind(&self) -> &'static str {
///         "mysql"
///     }
///
///     fn defaults(&self) -> ParamMap {
///         let mut table = ParamMap::new();
///         table.insert("host".to_string(), json!("localhost"));
///         table.insert("port".to_string(), json!(3306));
///         table
///     }
/// }
/// ```
pub trait ConnectorKindAdapter: Send + Sync {
    /// Returns the kind name (e.g., "mysql", "kafka")
    fn kind(&self) -> &'static str;

    /// Returns default parameters for this kind (used for scaffolding initial tables)
    fn defaults(&self) -> ParamMap {
        ParamMap::new()
    }

    /// Parses a connection URL into the parameters required by this kind.
    /// Returns Err if parsing fails.
    fn url_to_params(&self, _url: &str) -> anyhow::Result<ParamMap> {
        Ok(ParamMap::new())
    }

    /// Returns the default text output format.
    /// Only meaningful for file-based connectors; others should return None.
    fn default_fmt(&self) -> Option<TextFmt> {
        None
    }
}
