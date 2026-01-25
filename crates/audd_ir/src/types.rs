//! Canonical data types for the IR

use serde::{Deserialize, Serialize};

/// Canonical data types that abstract over source-specific types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CanonicalType {
    /// Boolean/bit value
    Boolean,

    /// 32-bit signed integer
    Int32,

    /// 64-bit signed integer
    Int64,

    /// 32-bit floating point
    Float32,

    /// 64-bit floating point
    Float64,

    /// Arbitrary precision decimal with precision and scale
    Decimal {
        /// Total number of digits
        precision: u16,
        /// Number of digits after decimal point
        scale: u16,
    },

    /// Variable-length string
    String,

    /// Large text field (CLOB, TEXT, etc.)
    Text,

    /// Binary data (BLOB)
    Binary,

    /// Date without time component
    Date,

    /// Time without date component
    Time,

    /// Date and time without timezone
    DateTime,

    /// Timestamp with timezone
    Timestamp,

    /// JSON data
    Json,

    /// UUID/GUID
    Uuid,

    /// Unknown or unmapped type
    Unknown {
        /// Original type name for reference
        original_type: String,
    },
}

impl CanonicalType {
    /// Check if this type is compatible with another type
    ///
    /// # Examples
    ///
    /// ```
    /// use audd_ir::CanonicalType;
    ///
    /// assert!(CanonicalType::Int32.is_compatible_with(&CanonicalType::Int32));
    /// assert!(CanonicalType::Int32.is_compatible_with(&CanonicalType::Int64));
    /// ```
    pub fn is_compatible_with(&self, other: &CanonicalType) -> bool {
        use CanonicalType::*;

        match (self, other) {
            // Exact matches
            (Boolean, Boolean) => true,
            (Int32, Int32) => true,
            (Int64, Int64) => true,
            (Float32, Float32) => true,
            (Float64, Float64) => true,
            (String, String) => true,
            (Text, Text) => true,
            (Binary, Binary) => true,
            (Date, Date) => true,
            (Time, Time) => true,
            (DateTime, DateTime) => true,
            (Timestamp, Timestamp) => true,
            (Json, Json) => true,
            (Uuid, Uuid) => true,

            // Numeric widening is compatible
            (Int32, Int64) => true,
            (Float32, Float64) => true,

            // Decimal compatibility if scale/precision match
            (
                Decimal {
                    precision: p1,
                    scale: s1,
                },
                Decimal {
                    precision: p2,
                    scale: s2,
                },
            ) => p1 == p2 && s1 == s2,

            // String and Text are compatible
            (String, Text) => true,
            (Text, String) => true,

            // DateTime and Timestamp are compatible
            (DateTime, Timestamp) => true,
            (Timestamp, DateTime) => true,

            _ => false,
        }
    }

    /// Get a human-readable name for this type
    pub fn type_name(&self) -> String {
        match self {
            Self::Boolean => "Boolean".to_string(),
            Self::Int32 => "Int32".to_string(),
            Self::Int64 => "Int64".to_string(),
            Self::Float32 => "Float32".to_string(),
            Self::Float64 => "Float64".to_string(),
            Self::Decimal { precision, scale } => format!("Decimal({}, {})", precision, scale),
            Self::String => "String".to_string(),
            Self::Text => "Text".to_string(),
            Self::Binary => "Binary".to_string(),
            Self::Date => "Date".to_string(),
            Self::Time => "Time".to_string(),
            Self::DateTime => "DateTime".to_string(),
            Self::Timestamp => "Timestamp".to_string(),
            Self::Json => "Json".to_string(),
            Self::Uuid => "Uuid".to_string(),
            Self::Unknown { original_type } => format!("Unknown({})", original_type),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_compatibility() {
        assert!(CanonicalType::Int32.is_compatible_with(&CanonicalType::Int32));
        assert!(CanonicalType::Int32.is_compatible_with(&CanonicalType::Int64));
        assert!(!CanonicalType::Int64.is_compatible_with(&CanonicalType::Int32));

        assert!(CanonicalType::String.is_compatible_with(&CanonicalType::Text));
        assert!(CanonicalType::Text.is_compatible_with(&CanonicalType::String));

        assert!(CanonicalType::DateTime.is_compatible_with(&CanonicalType::Timestamp));
    }

    #[test]
    fn test_decimal_compatibility() {
        let dec1 = CanonicalType::Decimal {
            precision: 10,
            scale: 2,
        };
        let dec2 = CanonicalType::Decimal {
            precision: 10,
            scale: 2,
        };
        let dec3 = CanonicalType::Decimal {
            precision: 10,
            scale: 3,
        };

        assert!(dec1.is_compatible_with(&dec2));
        assert!(!dec1.is_compatible_with(&dec3));
    }

    #[test]
    fn test_type_name() {
        assert_eq!(CanonicalType::Int32.type_name(), "Int32");
        assert_eq!(
            CanonicalType::Decimal {
                precision: 10,
                scale: 2
            }
            .type_name(),
            "Decimal(10, 2)"
        );
        assert_eq!(
            CanonicalType::Unknown {
                original_type: "CUSTOM".to_string()
            }
            .type_name(),
            "Unknown(CUSTOM)"
        );
    }

    #[test]
    fn test_serialization() {
        let type_int = CanonicalType::Int32;
        let json = serde_json::to_string(&type_int).unwrap();
        assert!(json.contains("int32"));

        let type_decimal = CanonicalType::Decimal {
            precision: 10,
            scale: 2,
        };
        let json = serde_json::to_string(&type_decimal).unwrap();
        let deserialized: CanonicalType = serde_json::from_str(&json).unwrap();
        assert_eq!(type_decimal, deserialized);
    }
}
