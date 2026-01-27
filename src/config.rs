use crate::status::SystemId;
use libpep::keys::distribution::BlindedGlobalKeys;
use libpep::keys::GlobalPublicKeys;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
/// The details of a single transcryptor
pub struct TranscryptorConfig {
    pub system_id: SystemId,
    pub url: Url,
}
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
/// The configuration of a PAAS system
pub struct PAASConfig {
    pub blinded_global_keys: BlindedGlobalKeys,
    pub global_public_keys: GlobalPublicKeys,
    pub transcryptors: Vec<TranscryptorConfig>,
}
