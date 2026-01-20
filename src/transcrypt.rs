use libpep::core::data::{EncryptedAttribute, EncryptedPseudonym};
use libpep::core::json::data::EncryptedPEPJSONValue;
use libpep::core::long::batch::LongEncryptedData;
use libpep::core::long::data::{LongEncryptedAttribute, LongEncryptedPseudonym};
use libpep::core::transcryption::batch::EncryptedData;
use libpep::core::transcryption::{EncryptionContext, PseudonymizationDomain};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
/// An API request to transcrypt a single encrypted pseudonym.
pub struct PseudonymizationRequest {
    /// The encrypted pseudonym.
    pub encrypted_pseudonym: EncryptedPseudonym,
    /// The domain of the encrypted pseudonym.
    pub domain_from: PseudonymizationDomain,
    /// The domain to transcrypt the pseudonym to.
    pub domain_to: PseudonymizationDomain,
    /// The session the pseudonym was encrypted in associated with this server.
    pub session_from: EncryptionContext,
    /// The session the pseudonym should be decryptable in associated with this server.
    pub session_to: EncryptionContext,
}
#[derive(Serialize, Deserialize)]
pub struct PseudonymizationResponse {
    /// The transcrypted pseudonym.
    pub encrypted_pseudonym: EncryptedPseudonym,
}

#[derive(Serialize, Deserialize, Debug)]
/// An API request to transcrypt a single encrypted pseudonym.
pub struct LongPseudonymizationRequest {
    /// The encrypted pseudonym.
    pub encrypted_pseudonym: LongEncryptedPseudonym,
    /// The domain of the encrypted pseudonym.
    pub domain_from: PseudonymizationDomain,
    /// The domain to transcrypt the pseudonym to.
    pub domain_to: PseudonymizationDomain,
    /// The session the pseudonym was encrypted in associated with this server.
    pub session_from: EncryptionContext,
    /// The session the pseudonym should be decryptable in associated with this server.
    pub session_to: EncryptionContext,
}
#[derive(Serialize, Deserialize)]
pub struct LongPseudonymizationResponse {
    /// The transcrypted pseudonym.
    pub encrypted_pseudonym: LongEncryptedPseudonym,
}

#[derive(Serialize, Deserialize, Debug)]
/// An API request to transcrypt multiple encrypted pseudonyms.
pub struct PseudonymizationBatchRequest {
    /// The encrypted pseudonyms.
    pub encrypted_pseudonyms: Vec<EncryptedPseudonym>,
    /// The domain of the encrypted pseudonyms.
    pub domain_from: PseudonymizationDomain,
    /// The domain to transcrypt the pseudonyms to.
    pub domain_to: PseudonymizationDomain,
    /// The session the pseudonyms were encrypted in associated with this server.
    pub session_from: EncryptionContext,
    /// The session the pseudonyms should be decryptable in associated with this server.
    pub session_to: EncryptionContext,
}

#[derive(Serialize, Deserialize)]
pub struct PseudonymizationBatchResponse {
    /// The transcrypted pseudonyms.
    /// Watch out: the order of the encrypted pseudonyms will be randomly permuted to break linkability.
    pub encrypted_pseudonyms: Vec<EncryptedPseudonym>,
}

#[derive(Serialize, Deserialize, Debug)]
/// An API request to transcrypt multiple encrypted pseudonyms.
pub struct LongPseudonymizationBatchRequest {
    /// The encrypted pseudonyms.
    pub encrypted_pseudonyms: Vec<LongEncryptedPseudonym>,
    /// The domain of the encrypted pseudonyms.
    pub domain_from: PseudonymizationDomain,
    /// The domain to transcrypt the pseudonyms to.
    pub domain_to: PseudonymizationDomain,
    /// The session the pseudonyms were encrypted in associated with this server.
    pub session_from: EncryptionContext,
    /// The session the pseudonyms should be decryptable in associated with this server.
    pub session_to: EncryptionContext,
}

#[derive(Serialize, Deserialize)]
pub struct LongPseudonymizationBatchResponse {
    /// The transcrypted pseudonyms.
    /// Watch out: the order of the encrypted pseudonyms will be randomly permuted to break linkability.
    pub encrypted_pseudonyms: Vec<LongEncryptedPseudonym>,
}

#[derive(Serialize, Deserialize, Debug)]
/// An API request to transcrypt a single encrypted attribute.
pub struct RekeyRequest {
    /// The encrypted data.
    pub encrypted_attribute: EncryptedAttribute,
    /// The session the attribute was encrypted in associated with this server.
    pub session_from: EncryptionContext,
    /// The session the attribute should be decryptable in associated with this server.
    pub session_to: EncryptionContext,
}
#[derive(Serialize, Deserialize)]
pub struct RekeyResponse {
    /// The rekeyed attribute
    pub encrypted_attribute: EncryptedAttribute,
}

#[derive(Serialize, Deserialize, Debug)]
/// An API request to transcrypt a single encrypted attribute.
pub struct LongRekeyRequest {
    /// The encrypted data.
    pub encrypted_attribute: LongEncryptedAttribute,
    /// The session the attribute was encrypted in associated with this server.
    pub session_from: EncryptionContext,
    /// The session the attribute should be decryptable in associated with this server.
    pub session_to: EncryptionContext,
}
#[derive(Serialize, Deserialize)]
pub struct LongRekeyResponse {
    /// The rekeyed attribute
    pub encrypted_attribute: LongEncryptedAttribute,
}

#[derive(Serialize, Deserialize, Debug)]
/// An API request to transcrypt a single encrypted attribute.
pub struct RekeyBatchRequest {
    /// The encrypted datas.
    pub encrypted_attributes: Vec<EncryptedAttribute>,
    /// The session the attributes were encrypted in associated with this server.
    pub session_from: EncryptionContext,
    /// The session the attributes should be decryptable in associated with this server.
    pub session_to: EncryptionContext,
}
#[derive(Serialize, Deserialize)]
pub struct RekeyBatchResponse {
    /// The rekeyed attribute
    pub encrypted_attributes: Vec<EncryptedAttribute>,
}

#[derive(Serialize, Deserialize, Debug)]
/// An API request to transcrypt a single encrypted attribute.
pub struct LongRekeyBatchRequest {
    /// The encrypted datas.
    pub encrypted_attributes: Vec<LongEncryptedAttribute>,
    /// The session the attributes were encrypted in associated with this server.
    pub session_from: EncryptionContext,
    /// The session the attributes should be decryptable in associated with this server.
    pub session_to: EncryptionContext,
}
#[derive(Serialize, Deserialize)]
pub struct LongRekeyBatchResponse {
    /// The rekeyed attribute
    pub encrypted_attributes: Vec<LongEncryptedAttribute>,
}

#[derive(Serialize, Deserialize, Debug)]
/// An API request to transcrypt a single encrypted pseudonym.
pub struct TranscryptionRequest {
    /// The encrypted data.
    pub encrypted: EncryptedData,
    /// The domain of the encrypted pseudonyms.
    pub domain_from: PseudonymizationDomain,
    /// The domain to transcrypt the pseudonyms to.
    pub domain_to: PseudonymizationDomain,
    /// The session the messages were encrypted in associated with this server.
    pub session_from: EncryptionContext,
    /// The session the messages should be decryptable in associated with this server.
    pub session_to: EncryptionContext,
}
#[derive(Serialize, Deserialize)]
pub struct TranscryptionResponse {
    /// The transcrypted data.
    pub encrypted: EncryptedData,
}

#[derive(Serialize, Deserialize, Debug)]
/// An API request to transcrypt a single encrypted pseudonym.
pub struct LongTranscryptionRequest {
    /// The encrypted data.
    pub encrypted: LongEncryptedData,
    /// The domain of the encrypted pseudonyms.
    pub domain_from: PseudonymizationDomain,
    /// The domain to transcrypt the pseudonyms to.
    pub domain_to: PseudonymizationDomain,
    /// The session the messages were encrypted in associated with this server.
    pub session_from: EncryptionContext,
    /// The session the messages should be decryptable in associated with this server.
    pub session_to: EncryptionContext,
}
#[derive(Serialize, Deserialize)]
pub struct LongTranscryptionResponse {
    /// The transcrypted data.
    pub encrypted: LongEncryptedData,
}

#[derive(Serialize, Deserialize, Debug)]
/// An API request to transcrypt a batch of encrypted data.
pub struct TranscryptionBatchRequest {
    /// The encrypted data.
    pub encrypted: Vec<EncryptedData>,
    /// The domain of the encrypted pseudonyms.
    pub domain_from: PseudonymizationDomain,
    /// The domain to transcrypt the pseudonyms to.
    pub domain_to: PseudonymizationDomain,
    /// The session the messages were encrypted in associated with this server.
    pub session_from: EncryptionContext,
    /// The session the messages should be decryptable in associated with this server.
    pub session_to: EncryptionContext,
}
#[derive(Serialize, Deserialize)]
pub struct TranscryptionBatchResponse {
    /// The transcrypted data (reordered to break linkability).
    pub encrypted: Vec<EncryptedData>,
}

#[derive(Serialize, Deserialize, Debug)]
/// An API request to transcrypt a single encrypted pseudonym.
pub struct LongTranscryptionBatchRequest {
    /// The encrypted data.
    pub encrypted: Vec<LongEncryptedData>,
    /// The domain of the encrypted pseudonyms.
    pub domain_from: PseudonymizationDomain,
    /// The domain to transcrypt the pseudonyms to.
    pub domain_to: PseudonymizationDomain,
    /// The session the messages were encrypted in associated with this server.
    pub session_from: EncryptionContext,
    /// The session the messages should be decryptable in associated with this server.
    pub session_to: EncryptionContext,
}
#[derive(Serialize, Deserialize)]
pub struct LongTranscryptionBatchResponse {
    /// The transcrypted data (reordered to break linkability).
    pub encrypted: Vec<LongEncryptedData>,
}

#[derive(Serialize, Deserialize, Debug)]
/// An API request to transcrypt a single encrypted pseudonym.
pub struct JsonTranscryptionRequest {
    /// The encrypted data.
    pub encrypted: EncryptedPEPJSONValue,
    /// The domain of the encrypted pseudonyms.
    pub domain_from: PseudonymizationDomain,
    /// The domain to transcrypt the pseudonyms to.
    pub domain_to: PseudonymizationDomain,
    /// The session the messages were encrypted in associated with this server.
    pub session_from: EncryptionContext,
    /// The session the messages should be decryptable in associated with this server.
    pub session_to: EncryptionContext,
}
#[derive(Serialize, Deserialize)]
pub struct JsonTranscryptionResponse {
    /// The transcrypted data.
    pub encrypted: EncryptedPEPJSONValue,
}

#[derive(Serialize, Deserialize, Debug)]
/// An API request to transcrypt a batch of encrypted data.
pub struct JsonTranscryptionBatchRequest {
    /// The encrypted data.
    pub encrypted: Vec<EncryptedPEPJSONValue>,
    /// The domain of the encrypted pseudonyms.
    pub domain_from: PseudonymizationDomain,
    /// The domain to transcrypt the pseudonyms to.
    pub domain_to: PseudonymizationDomain,
    /// The session the messages were encrypted in associated with this server.
    pub session_from: EncryptionContext,
    /// The session the messages should be decryptable in associated with this server.
    pub session_to: EncryptionContext,
}
#[derive(Serialize, Deserialize)]
pub struct JsonTranscryptionBatchResponse {
    /// The transcrypted data (reordered to break linkability).
    pub encrypted: Vec<EncryptedPEPJSONValue>,
}
