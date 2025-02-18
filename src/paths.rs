pub const API_BASE: &str = ""; // Currently empty (may need to change)

pub const STATUS: &str = "/status";
pub const CONFIG: &str = "/config";
pub const SESSIONS_GET: &str = "/sessions";
pub const SESSIONS_START: &str = "/sessions/start";
pub const SESSIONS_END: &str = "/sessions/end";

pub mod transcrypt {
    pub const PSEUDONYMIZE: &str = "/pseudonymize";
    pub const PSEUDONYMIZE_BATCH: &str = "/pseudonymize_batch";
    pub const REKEY: &str = "/rekey";
    pub const REKEY_BATCH: &str = "/rekey_batch";
    pub const TRANSCRYPT: &str = "/transcrypt";
}
