pub mod connector;
pub mod structure;
pub mod tags;
mod traits;

// Re-export types, traits, and functions for tests
pub use connector::ConnectorKindAdapter;
pub use structure::{Protocol, Validate, de_bool_onoff};
pub use traits::ConfParser;
