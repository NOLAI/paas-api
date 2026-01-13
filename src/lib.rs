/// The current version of the API
pub const CURRENT_PROTOCOL_VERSION: &str = env!("CARGO_PKG_VERSION");
/// The minimal supported compatible version
pub const MIN_SUPPORTED_VERSION: &str = "0.7.0"; // If our protocol changes in a breaking way, we should update this number accordingly

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
