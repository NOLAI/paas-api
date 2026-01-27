use libpep::data::json::EncryptedPEPJSONValue;
use libpep::data::long::{LongEncryptedAttribute, LongEncryptedPseudonym};
use libpep::data::records::{EncryptedRecord, LongEncryptedRecord};
use libpep::data::simple::{EncryptedAttribute, EncryptedPseudonym};
use libpep::data::traits::HasStructure;

/// Trait for encrypted types to provide their API path segment.
/// The path format is: /{operation}[_batch]/{type_segment}
/// Examples: /pseudonymize/pseudonym, /rekey_batch/long_attribute, /transcrypt/json
pub trait ApiPath {
    /// Returns the full path segment for this encrypted type (e.g., "pseudonym", "long_attribute", "json").
    fn path_segment() -> &'static str;
}

// Simple types
impl ApiPath for EncryptedPseudonym {
    fn path_segment() -> &'static str {
        "pseudonym"
    }
}

impl ApiPath for EncryptedAttribute {
    fn path_segment() -> &'static str {
        "attribute"
    }
}

impl ApiPath for EncryptedRecord {
    fn path_segment() -> &'static str {
        "record"
    }
}

// Long types
impl ApiPath for LongEncryptedPseudonym {
    fn path_segment() -> &'static str {
        "long_pseudonym"
    }
}

impl ApiPath for LongEncryptedAttribute {
    fn path_segment() -> &'static str {
        "long_attribute"
    }
}

impl ApiPath for LongEncryptedRecord {
    fn path_segment() -> &'static str {
        "long_record"
    }
}

// JSON types
impl ApiPath for EncryptedPEPJSONValue {
    fn path_segment() -> &'static str {
        "json"
    }
}

pub const API_BASE: &str = ""; // Currently empty (may need to change)

pub const STATUS: &str = "/status";
pub const CONFIG: &str = "/config";
pub const SESSIONS_GET: &str = "/sessions";
pub const SESSIONS_START: &str = "/sessions/start";
pub const SESSIONS_END: &str = "/sessions/end";

/// Generate the pseudonymization path for a given encrypted type.
pub fn pseudonymize_path<T: ApiPath>() -> String {
    format!("/pseudonymize/{}", T::path_segment())
}

/// Generate the pseudonymization batch path for a given encrypted type.
/// Requires the type to implement HasStructure for batch validation.
pub fn pseudonymize_batch_path<T: ApiPath + HasStructure>() -> String {
    format!("/pseudonymize_batch/{}", T::path_segment())
}

/// Generate the rekey path for a given encrypted type.
pub fn rekey_path<T: ApiPath>() -> String {
    format!("/rekey/{}", T::path_segment())
}

/// Generate the rekey batch path for a given encrypted type.
/// Requires the type to implement HasStructure for batch validation.
pub fn rekey_batch_path<T: ApiPath + HasStructure>() -> String {
    format!("/rekey_batch/{}", T::path_segment())
}

/// Generate the transcrypt path for a given encrypted type.
pub fn transcrypt_path<T: ApiPath>() -> String {
    format!("/transcrypt/{}", T::path_segment())
}

/// Generate the transcrypt batch path for a given encrypted type.
/// Requires the type to implement HasStructure for batch validation.
pub fn transcrypt_batch_path<T: ApiPath + HasStructure>() -> String {
    format!("/transcrypt_batch/{}", T::path_segment())
}

#[cfg(test)]
mod tests {
    use super::*;
    use libpep::data::json::EncryptedPEPJSONValue;
    use libpep::data::long::{LongEncryptedAttribute, LongEncryptedPseudonym};
    use libpep::data::records::{EncryptedRecord, LongEncryptedRecord};
    use libpep::data::simple::{EncryptedAttribute, EncryptedPseudonym};

    #[test]
    fn test_simple_paths() {
        assert_eq!(
            pseudonymize_path::<EncryptedPseudonym>(),
            "/pseudonymize/pseudonym"
        );
        assert_eq!(
            pseudonymize_batch_path::<EncryptedPseudonym>(),
            "/pseudonymize_batch/pseudonym"
        );
        assert_eq!(rekey_path::<EncryptedAttribute>(), "/rekey/attribute");
        assert_eq!(
            rekey_batch_path::<EncryptedAttribute>(),
            "/rekey_batch/attribute"
        );
        assert_eq!(transcrypt_path::<EncryptedRecord>(), "/transcrypt/record");
        assert_eq!(
            transcrypt_batch_path::<EncryptedRecord>(),
            "/transcrypt_batch/record"
        );
    }

    #[test]
    fn test_long_paths() {
        assert_eq!(
            pseudonymize_path::<LongEncryptedPseudonym>(),
            "/pseudonymize/long_pseudonym"
        );
        assert_eq!(
            pseudonymize_batch_path::<LongEncryptedPseudonym>(),
            "/pseudonymize_batch/long_pseudonym"
        );
        assert_eq!(
            rekey_path::<LongEncryptedAttribute>(),
            "/rekey/long_attribute"
        );
        assert_eq!(
            rekey_batch_path::<LongEncryptedAttribute>(),
            "/rekey_batch/long_attribute"
        );
        assert_eq!(
            transcrypt_path::<LongEncryptedRecord>(),
            "/transcrypt/long_record"
        );
        assert_eq!(
            transcrypt_batch_path::<LongEncryptedRecord>(),
            "/transcrypt_batch/long_record"
        );
    }

    #[test]
    fn test_json_paths() {
        assert_eq!(
            transcrypt_path::<EncryptedPEPJSONValue>(),
            "/transcrypt/json"
        );
        assert_eq!(
            transcrypt_batch_path::<EncryptedPEPJSONValue>(),
            "/transcrypt_batch/json"
        );
    }

    #[test]
    fn test_pseudonym_can_be_rekeyed() {
        // Pseudonyms can be rekeyed too, not just pseudonymized
        assert_eq!(rekey_path::<EncryptedPseudonym>(), "/rekey/pseudonym");
        assert_eq!(
            rekey_path::<LongEncryptedPseudonym>(),
            "/rekey/long_pseudonym"
        );
    }
}
