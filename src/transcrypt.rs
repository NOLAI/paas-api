use libpep::data::traits::{Pseudonymizable, Rekeyable, Transcryptable};
use libpep::factors::{EncryptionContext, PseudonymizationDomain};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
/// An API request to transcrypt an encrypted pseudonym.
pub struct PseudonymizationRequest<T: Pseudonymizable> {
    /// The encrypted pseudonym.
    pub encrypted: T,
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
pub struct PseudonymizationResponse<T: Pseudonymizable> {
    /// The transcrypted pseudonym.
    pub result: T,
}

#[derive(Serialize, Deserialize, Debug)]
/// An API request to transcrypt a batch of encrypted pseudonyms.
pub struct PseudonymizationBatchRequest<T: Pseudonymizable> {
    /// The encrypted pseudonyms.
    pub encrypted: Vec<T>,
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
pub struct PseudonymizationBatchResponse<T: Pseudonymizable> {
    /// The transcrypted pseudonyms.
    /// Watch out: the order may be randomly permuted to break linkability.
    pub result: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
/// An API request to rekey an encrypted attribute.
pub struct RekeyRequest<T: Rekeyable> {
    /// The encrypted data.
    pub encrypted: T,
    /// The session the attribute was encrypted in associated with this server.
    pub session_from: EncryptionContext,
    /// The session the attribute should be decryptable in associated with this server.
    pub session_to: EncryptionContext,
}

#[derive(Serialize, Deserialize)]
pub struct RekeyResponse<T: Rekeyable> {
    /// The rekeyed attribute.
    pub result: T,
}

#[derive(Serialize, Deserialize, Debug)]
/// An API request to rekey a batch of encrypted attributes.
pub struct RekeyBatchRequest<T: Rekeyable> {
    /// The encrypted data.
    pub encrypted: Vec<T>,
    /// The session the attributes were encrypted in associated with this server.
    pub session_from: EncryptionContext,
    /// The session the attributes should be decryptable in associated with this server.
    pub session_to: EncryptionContext,
}

#[derive(Serialize, Deserialize)]
pub struct RekeyBatchResponse<T: Rekeyable> {
    /// The rekeyed attributes.
    pub result: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
/// An API request to transcrypt encrypted data.
pub struct TranscryptionRequest<T: Transcryptable> {
    /// The encrypted data.
    pub encrypted: T,
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
pub struct TranscryptionResponse<T: Transcryptable> {
    /// The transcrypted data.
    pub result: T,
}

#[derive(Serialize, Deserialize, Debug)]
/// An API request to transcrypt a batch of encrypted data.
pub struct TranscryptionBatchRequest<T: Transcryptable> {
    /// The encrypted data.
    pub encrypted: Vec<T>,
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
pub struct TranscryptionBatchResponse<T: Transcryptable> {
    /// The transcrypted data.
    /// Watch out: the order may be randomly permuted to break linkability.
    pub result: Vec<T>,
}
