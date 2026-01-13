pub const API_BASE: &str = ""; // Currently empty (may need to change)

pub const STATUS: &str = "/status";
pub const CONFIG: &str = "/config";
pub const SESSIONS_GET: &str = "/sessions";
pub const SESSIONS_START: &str = "/sessions/start";
pub const SESSIONS_END: &str = "/sessions/end";

pub const PSEUDONYMIZE: &str = "/pseudonymize";
pub const PSEUDONYMIZE_BATCH: &str = "/pseudonymize/batch";
pub const REKEY: &str = "/rekey";
pub const REKEY_BATCH: &str = "/rekey/batch";
pub const TRANSCRYPT: &str = "/transcrypt";
pub const TRANSCRYPT_BATCH: &str = "/transcrypt/batch";

pub const PSEUDONYMIZE_LONG: &str = "/long/pseudonymize";
pub const PSEUDONYMIZE_BATCH_LONG: &str = "/long/pseudonymize/batch";
pub const REKEY_LONG: &str = "/long/rekey";
pub const REKEY_BATCH_LONG: &str = "/long/rekey/batch";
pub const TRANSCRYPT_LONG: &str = "/long/transcrypt";
pub const TRANSCRYPT_BATCH_LONG: &str = "/long/transcrypt/batch";

pub const TRANSCRYPT_JSON: &str = "/json/transcrypt";
pub const TRANSCRYPT_JSON_BATCH: &str = "/json/transcrypt/batch";
