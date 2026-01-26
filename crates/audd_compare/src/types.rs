//! Type compatibility checking

use audd_ir::CanonicalType;
use serde::{Deserialize, Serialize};

/// Type compatibility classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypeCompatibility {
    /// Types are exactly the same
    Identical,
    /// Types are compatible (e.g., String and Text)
    Compatible,
    /// Widening conversion is safe (e.g., Int32 to Int64)
    WideningSafe,
    /// Requires explicit cast (may lose precision)
    RequiresCast,
    /// Types are incompatible
    Incompatible,
}

/// Compare two canonical types for compatibility
///
/// # Examples
///
/// ```
/// use audd_compare::compare_types;
/// use audd_ir::CanonicalType;
///
/// let compat = compare_types(&CanonicalType::Int32, &CanonicalType::Int64);
/// // Returns TypeCompatibility::WideningSafe
/// ```
pub fn compare_types(type_a: &CanonicalType, type_b: &CanonicalType) -> TypeCompatibility {
    use CanonicalType::*;

    // Exact match
    if type_a == type_b {
        return TypeCompatibility::Identical;
    }

    match (type_a, type_b) {
        // Numeric widening (safe)
        (Int32, Int64) => TypeCompatibility::WideningSafe,
        (Float32, Float64) => TypeCompatibility::WideningSafe,

        // Numeric narrowing (requires cast, may lose precision)
        (Int64, Int32) => TypeCompatibility::RequiresCast,
        (Float64, Float32) => TypeCompatibility::RequiresCast,

        // String types are compatible
        (String, Text) | (Text, String) => TypeCompatibility::Compatible,

        // DateTime types are compatible
        (DateTime, Timestamp) | (Timestamp, DateTime) => TypeCompatibility::Compatible,

        // Date to DateTime is widening
        (Date, DateTime) | (Date, Timestamp) => TypeCompatibility::WideningSafe,

        // DateTime to Date requires cast (loses time info)
        (DateTime, Date) | (Timestamp, Date) => TypeCompatibility::RequiresCast,

        // Decimal compatibility (if precision/scale match from is_compatible_with)
        (
            Decimal {
                precision: p1,
                scale: s1,
            },
            Decimal {
                precision: p2,
                scale: s2,
            },
        ) => {
            if p1 == p2 && s1 == s2 {
                TypeCompatibility::Identical
            } else if p1 <= p2 && s1 <= s2 {
                TypeCompatibility::WideningSafe
            } else {
                TypeCompatibility::RequiresCast
            }
        }

        // Numeric to Decimal conversions
        (Int32, Decimal { .. }) | (Int64, Decimal { .. }) => TypeCompatibility::WideningSafe,
        (Decimal { .. }, Int32) | (Decimal { .. }, Int64) => TypeCompatibility::RequiresCast,
        (Float32, Decimal { .. }) | (Float64, Decimal { .. }) => TypeCompatibility::RequiresCast,
        (Decimal { .. }, Float32) | (Decimal { .. }, Float64) => TypeCompatibility::RequiresCast,

        // Integer to Float conversions
        (Int32, Float32) | (Int32, Float64) | (Int64, Float64) => {
            TypeCompatibility::RequiresCast
        }
        (Float32, Int32) | (Float64, Int32) | (Float64, Int64) => {
            TypeCompatibility::RequiresCast
        }

        // Boolean conversions
        (Boolean, Int32) | (Int32, Boolean) => TypeCompatibility::RequiresCast,

        // Unknown types
        (Unknown { .. }, _) | (_, Unknown { .. }) => TypeCompatibility::Incompatible,

        // All other combinations are incompatible
        _ => TypeCompatibility::Incompatible,
    }
}

/// Check if two types are compatible (not incompatible)
#[allow(dead_code)]
pub fn are_types_compatible(type_a: &CanonicalType, type_b: &CanonicalType) -> bool {
    !matches!(
        compare_types(type_a, type_b),
        TypeCompatibility::Incompatible
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identical_types() {
        assert_eq!(
            compare_types(&CanonicalType::Int32, &CanonicalType::Int32),
            TypeCompatibility::Identical
        );
        assert_eq!(
            compare_types(&CanonicalType::String, &CanonicalType::String),
            TypeCompatibility::Identical
        );
    }

    #[test]
    fn test_numeric_widening() {
        assert_eq!(
            compare_types(&CanonicalType::Int32, &CanonicalType::Int64),
            TypeCompatibility::WideningSafe
        );
        assert_eq!(
            compare_types(&CanonicalType::Float32, &CanonicalType::Float64),
            TypeCompatibility::WideningSafe
        );
    }

    #[test]
    fn test_numeric_narrowing() {
        assert_eq!(
            compare_types(&CanonicalType::Int64, &CanonicalType::Int32),
            TypeCompatibility::RequiresCast
        );
        assert_eq!(
            compare_types(&CanonicalType::Float64, &CanonicalType::Float32),
            TypeCompatibility::RequiresCast
        );
    }

    #[test]
    fn test_string_compatibility() {
        assert_eq!(
            compare_types(&CanonicalType::String, &CanonicalType::Text),
            TypeCompatibility::Compatible
        );
        assert_eq!(
            compare_types(&CanonicalType::Text, &CanonicalType::String),
            TypeCompatibility::Compatible
        );
    }

    #[test]
    fn test_datetime_compatibility() {
        assert_eq!(
            compare_types(&CanonicalType::DateTime, &CanonicalType::Timestamp),
            TypeCompatibility::Compatible
        );
        assert_eq!(
            compare_types(&CanonicalType::Date, &CanonicalType::DateTime),
            TypeCompatibility::WideningSafe
        );
        assert_eq!(
            compare_types(&CanonicalType::DateTime, &CanonicalType::Date),
            TypeCompatibility::RequiresCast
        );
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
            precision: 12,
            scale: 3,
        };

        assert_eq!(
            compare_types(&dec1, &dec2),
            TypeCompatibility::Identical
        );
        assert_eq!(
            compare_types(&dec1, &dec3),
            TypeCompatibility::WideningSafe
        );
    }

    #[test]
    fn test_incompatible_types() {
        assert_eq!(
            compare_types(&CanonicalType::Int32, &CanonicalType::String),
            TypeCompatibility::Incompatible
        );
        assert_eq!(
            compare_types(&CanonicalType::Boolean, &CanonicalType::Date),
            TypeCompatibility::Incompatible
        );
        assert_eq!(
            compare_types(&CanonicalType::Binary, &CanonicalType::Json),
            TypeCompatibility::Incompatible
        );
    }

    #[test]
    fn test_are_types_compatible() {
        assert!(are_types_compatible(
            &CanonicalType::Int32,
            &CanonicalType::Int64
        ));
        assert!(are_types_compatible(
            &CanonicalType::String,
            &CanonicalType::Text
        ));
        assert!(!are_types_compatible(
            &CanonicalType::Int32,
            &CanonicalType::String
        ));
    }

    #[test]
    fn test_unknown_types() {
        let unknown = CanonicalType::Unknown {
            original_type: "CUSTOM".to_string(),
        };
        assert_eq!(
            compare_types(&unknown, &CanonicalType::Int32),
            TypeCompatibility::Incompatible
        );
    }
}
