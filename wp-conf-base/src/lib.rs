pub mod structure;
pub mod connector;

// Re-export types, traits, and functions for tests
pub use structure::{Protocol, TagParse, Validate, GetTagStr, de_bool_onoff};
pub use connector::ConnectorKindAdapter;
