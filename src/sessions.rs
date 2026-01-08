use libpep::core::transcryption::EncryptionContext;
use libpep::distributed::server::keys::SessionKeyShares;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
/// Return all sessions of a user
pub struct SessionResponse {
    pub sessions: Vec<EncryptionContext>,
}

#[derive(Serialize, Deserialize, Debug)]
/// Start a new PEP session
pub struct StartSessionResponse {
    /// A session id
    pub session_id: EncryptionContext,
    /// The secret session key share for this session. Will be provided just once.
    pub session_key_shares: SessionKeyShares,
}

#[derive(Serialize, Deserialize, Debug)]
/// Terminate a session
pub struct EndSessionRequest {
    pub session_id: EncryptionContext,
}
