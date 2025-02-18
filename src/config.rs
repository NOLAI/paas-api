use libpep::distributed::key_blinding::BlindedGlobalSecretKey;
use libpep::high_level::keys::GlobalPublicKey;
use serde::{Deserialize, Serialize};
use crate::status::SystemId;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
/// The details of a single transcryptor
pub struct TranscryptorConfig {
    pub system_id: SystemId,
    pub url: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
/// The configuration of a PAAS system
pub struct PAASConfig {
    pub blinded_global_secret_key: BlindedGlobalSecretKey,
    pub global_public_key: GlobalPublicKey,
    pub transcryptors: Vec<TranscryptorConfig>,
}
