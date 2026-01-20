/// The current version of the API
pub const CURRENT_PROTOCOL_VERSION: &str = env!("CARGO_PKG_VERSION");
/// The minimal supported compatible version
pub const MIN_SUPPORTED_VERSION: &str = "0.8.0";

/// Transcryptor config
pub mod config;
/// API paths
pub mod paths;
/// Managing PEP sessions
pub mod sessions;
/// Server status checks
pub mod status;
/// Pseudonym transcryption
pub mod transcrypt;
