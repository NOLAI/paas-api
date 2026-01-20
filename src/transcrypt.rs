use libpep::core::data::{EncryptedAttribute, EncryptedPseudonym};
use libpep::core::json::data::EncryptedPEPJSONValue;
use libpep::core::long::batch::LongEncryptedData;
use libpep::core::long::data::{LongEncryptedAttribute, LongEncryptedPseudonym};
use libpep::core::transcryption::{EncryptionContext, PseudonymizationDomain};
use libpep::core::transcryption::batch::EncryptedData;
use serde::{Deserialize, Serialize};

/// Error returned when trying to convert an encrypted variant to the wrong concrete type
#[derive(Debug, Clone, thiserror::Error)]
#[error("Unexpected variant type: expected {expected}, got {actual}")]
pub struct VariantConversionError {
    pub expected: &'static str,
    pub actual: &'static str,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "data")]
///  wrapper for encrypted pseudonyms (normal or long)
pub enum EncryptedPseudonymVariant {
    #[serde(rename = "normal")]
    Normal(EncryptedPseudonym),
    #[serde(rename = "long")]
    Long(LongEncryptedPseudonym)
}

impl From<EncryptedPseudonym> for EncryptedPseudonymVariant {
    fn from(value: EncryptedPseudonym) -> Self {
        EncryptedPseudonymVariant::Normal(value)
    }
}

impl From<LongEncryptedPseudonym> for EncryptedPseudonymVariant {
    fn from(value: LongEncryptedPseudonym) -> Self {
        EncryptedPseudonymVariant::Long(value)
    }
}

impl TryFrom<EncryptedPseudonymVariant> for EncryptedPseudonym {
    type Error = VariantConversionError;

    fn try_from(value: EncryptedPseudonymVariant) -> Result<Self, Self::Error> {
        match value {
            EncryptedPseudonymVariant::Normal(ep) => Ok(ep),
            EncryptedPseudonymVariant::Long(_) => Err(VariantConversionError {
                expected: "Normal",
                actual: "Long",
            }),
        }
    }
}

impl TryFrom<EncryptedPseudonymVariant> for LongEncryptedPseudonym {
    type Error = VariantConversionError;

    fn try_from(value: EncryptedPseudonymVariant) -> Result<Self, Self::Error> {
        match value {
            EncryptedPseudonymVariant::Long(ep) => Ok(ep),
            EncryptedPseudonymVariant::Normal(_) => Err(VariantConversionError {
                expected: "Long",
                actual: "Normal",
            }),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "data")]
///  wrapper for encrypted attributes (normal or long)
pub enum EncryptedAttributeVariant {
    #[serde(rename = "normal")]
    Normal(EncryptedAttribute),
    #[serde(rename = "long")]
    Long(LongEncryptedAttribute),
}

impl From<EncryptedAttribute> for EncryptedAttributeVariant {
    fn from(value: EncryptedAttribute) -> Self {
        EncryptedAttributeVariant::Normal(value)
    }
}

impl From<LongEncryptedAttribute> for EncryptedAttributeVariant {
    fn from(value: LongEncryptedAttribute) -> Self {
        EncryptedAttributeVariant::Long(value)
    }
}

impl TryFrom<EncryptedAttributeVariant> for EncryptedAttribute {
    type Error = VariantConversionError;

    fn try_from(value: EncryptedAttributeVariant) -> Result<Self, Self::Error> {
        match value {
            EncryptedAttributeVariant::Normal(ea) => Ok(ea),
            EncryptedAttributeVariant::Long(_) => Err(VariantConversionError {
                expected: "Normal",
                actual: "Long",
            }),
        }
    }
}

impl TryFrom<EncryptedAttributeVariant> for LongEncryptedAttribute {
    type Error = VariantConversionError;

    fn try_from(value: EncryptedAttributeVariant) -> Result<Self, Self::Error> {
        match value {
            EncryptedAttributeVariant::Long(ea) => Ok(ea),
            EncryptedAttributeVariant::Normal(_) => Err(VariantConversionError {
                expected: "Long",
                actual: "Normal",
            }),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "data")]
///  wrapper for encrypted data (normal, long, or json)
pub enum EncryptedDataVariant {
    #[serde(rename = "normal")]
    Normal(EncryptedData),
    #[serde(rename = "long")]
    Long(LongEncryptedData),
    #[serde(rename = "json")]
    Json(EncryptedPEPJSONValue),
}

impl From<EncryptedData> for EncryptedDataVariant {
    fn from(value: EncryptedData) -> Self {
        EncryptedDataVariant::Normal(value)
    }
}

impl From<LongEncryptedData> for EncryptedDataVariant {
    fn from(value: LongEncryptedData) -> Self {
        EncryptedDataVariant::Long(value)
    }
}

impl From<EncryptedPEPJSONValue> for EncryptedDataVariant {
    fn from(value: EncryptedPEPJSONValue) -> Self {
        EncryptedDataVariant::Json(value)
    }
}

impl TryFrom<EncryptedDataVariant> for EncryptedData {
    type Error = VariantConversionError;

    fn try_from(value: EncryptedDataVariant) -> Result<Self, Self::Error> {
        match value {
            EncryptedDataVariant::Normal(ed) => Ok(ed),
            EncryptedDataVariant::Long(_) => Err(VariantConversionError {
                expected: "Normal",
                actual: "Long",
            }),
            EncryptedDataVariant::Json(_) => Err(VariantConversionError {
                expected: "Normal",
                actual: "Json",
            }),
        }
    }
}

impl TryFrom<EncryptedDataVariant> for LongEncryptedData {
    type Error = VariantConversionError;

    fn try_from(value: EncryptedDataVariant) -> Result<Self, Self::Error> {
        match value {
            EncryptedDataVariant::Long(ed) => Ok(ed),
            EncryptedDataVariant::Normal(_) => Err(VariantConversionError {
                expected: "Long",
                actual: "Normal",
            }),
            EncryptedDataVariant::Json(_) => Err(VariantConversionError {
                expected: "Long",
                actual: "Json",
            }),
        }
    }
}

impl TryFrom<EncryptedDataVariant> for EncryptedPEPJSONValue {
    type Error = VariantConversionError;

    fn try_from(value: EncryptedDataVariant) -> Result<Self, Self::Error> {
        match value {
            EncryptedDataVariant::Json(ed) => Ok(ed),
            EncryptedDataVariant::Normal(_) => Err(VariantConversionError {
                expected: "Json",
                actual: "Normal",
            }),
            EncryptedDataVariant::Long(_) => Err(VariantConversionError {
                expected: "Json",
                actual: "Long",
            }),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
///  API request to pseudonymize a single encrypted pseudonym
pub struct PseudonymizationRequest {
    /// The encrypted pseudonym (normal or long)
    pub encrypted_pseudonym: EncryptedPseudonymVariant,
    /// The domain of the encrypted pseudonym
    pub domain_from: PseudonymizationDomain,
    /// The domain to transcrypt the pseudonym to
    pub domain_to: PseudonymizationDomain,
    /// The session the pseudonym was encrypted in
    pub session_from: EncryptionContext,
    /// The session the pseudonym should be decryptable in
    pub session_to: EncryptionContext,
}

#[derive(Serialize, Deserialize)]
pub struct PseudonymizationResponse {
    /// The transcrypted pseudonym (same type as request)
    pub encrypted_pseudonym: EncryptedPseudonymVariant,
}

#[derive(Serialize, Deserialize, Debug)]
///  API request to pseudonymize multiple encrypted pseudonyms
pub struct PseudonymizationBatchRequest {
    /// The encrypted pseudonyms (all must be same type)
    pub encrypted_pseudonyms: Vec<EncryptedPseudonymVariant>,
    /// The domain of the encrypted pseudonyms
    pub domain_from: PseudonymizationDomain,
    /// The domain to transcrypt the pseudonyms to
    pub domain_to: PseudonymizationDomain,
    /// The session the pseudonyms were encrypted in
    pub session_from: EncryptionContext,
    /// The session the pseudonyms should be decryptable in
    pub session_to: EncryptionContext,
}

#[derive(Serialize, Deserialize)]
pub struct PseudonymizationBatchResponse {
    /// The transcrypted pseudonyms (order randomly permuted)
    pub encrypted_pseudonyms: Vec<EncryptedPseudonymVariant>,
}

#[derive(Serialize, Deserialize, Debug)]
///  API request to rekey a single encrypted attribute
pub struct RekeyRequest {
    /// The encrypted attribute (normal or long)
    pub encrypted_attribute: EncryptedAttributeVariant,
    /// The session the attribute was encrypted in
    pub session_from: EncryptionContext,
    /// The session the attribute should be decryptable in
    pub session_to: EncryptionContext,
}

#[derive(Serialize, Deserialize)]
pub struct RekeyResponse {
    /// The rekeyed attribute (same type as request)
    pub encrypted_attribute: EncryptedAttributeVariant,
}

#[derive(Serialize, Deserialize, Debug)]
///  API request to rekey multiple encrypted attributes
pub struct RekeyBatchRequest {
    /// The encrypted attributes (all must be same type)
    pub encrypted_attributes: Vec<EncryptedAttributeVariant>,
    /// The session the attributes were encrypted in
    pub session_from: EncryptionContext,
    /// The session the attributes should be decryptable in
    pub session_to: EncryptionContext,
}

#[derive(Serialize, Deserialize)]
pub struct RekeyBatchResponse {
    /// The rekeyed attributes
    pub encrypted_attributes: Vec<EncryptedAttributeVariant>,
}

#[derive(Serialize, Deserialize, Debug)]
///  API request to transcrypt encrypted data (supports normal, long, and json)
pub struct TranscryptionRequest {
    /// The encrypted data (normal, long, or json)
    pub encrypted: EncryptedDataVariant,
    /// The domain of the encrypted pseudonyms
    pub domain_from: PseudonymizationDomain,
    /// The domain to transcrypt the pseudonyms to
    pub domain_to: PseudonymizationDomain,
    /// The session the data was encrypted in
    pub session_from: EncryptionContext,
    /// The session the data should be decryptable in
    pub session_to: EncryptionContext,
}

#[derive(Serialize, Deserialize)]
pub struct TranscryptionResponse {
    /// The transcrypted data (same type as request)
    pub encrypted: EncryptedDataVariant,
}

#[derive(Serialize, Deserialize, Debug)]
///  API request to transcrypt multiple encrypted data items
pub struct TranscryptionBatchRequest {
    /// The encrypted data items (all must be same type)
    pub encrypted: Vec<EncryptedDataVariant>,
    /// The domain of the encrypted pseudonyms
    pub domain_from: PseudonymizationDomain,
    /// The domain to transcrypt the pseudonyms to
    pub domain_to: PseudonymizationDomain,
    /// The session the data was encrypted in
    pub session_from: EncryptionContext,
    /// The session the data should be decryptable in
    pub session_to: EncryptionContext,
}

#[derive(Serialize, Deserialize)]
pub struct TranscryptionBatchResponse {
    /// The transcrypted data (reordered to break linkability).
    pub encrypted: Vec<EncryptedDataVariant>,
}
