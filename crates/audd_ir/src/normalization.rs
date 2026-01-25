//! Identifier normalization and type mapping

use crate::CanonicalType;

/// Normalize an identifier (table/column name)
///
/// Applies the following transformations:
/// 1. Trim whitespace
/// 2. Convert to lowercase
/// 3. Convert to snake_case
/// 4. Collapse multiple spaces to single underscore
/// 5. Remove special characters (keeping alphanumeric and underscores)
///
/// # Examples
///
/// ```
/// use audd_ir::normalize_identifier;
///
/// assert_eq!(normalize_identifier("UserEmail"), "user_email");
/// assert_eq!(normalize_identifier("  Product Name  "), "product_name");
/// assert_eq!(normalize_identifier("firstName"), "first_name");
/// assert_eq!(normalize_identifier("Customer ID"), "customer_id");
/// ```
pub fn normalize_identifier(name: &str) -> String {
    let mut result = String::new();
    let trimmed = name.trim();

    let mut prev_was_lower = false;
    let mut prev_was_underscore = false;

    for (i, ch) in trimmed.chars().enumerate() {
        match ch {
            // Uppercase letters
            'A'..='Z' => {
                // Insert underscore before uppercase if preceded by lowercase
                if i > 0 && prev_was_lower && !prev_was_underscore {
                    result.push('_');
                }
                result.push(ch.to_ascii_lowercase());
                prev_was_lower = false;
                prev_was_underscore = false;
            }
            // Lowercase letters and numbers
            'a'..='z' | '0'..='9' => {
                result.push(ch);
                prev_was_lower = true;
                prev_was_underscore = false;
            }
            // Spaces and special chars -> underscore
            ' ' | '-' | '.' | '/' | '\\' => {
                if !result.is_empty() && !prev_was_underscore {
                    result.push('_');
                    prev_was_underscore = true;
                }
                prev_was_lower = false;
            }
            // Underscores
            '_' => {
                if !result.is_empty() && !prev_was_underscore {
                    result.push('_');
                    prev_was_underscore = true;
                }
                prev_was_lower = false;
            }
            // Other characters are ignored
            _ => {}
        }
    }

    // Remove trailing underscores
    result.trim_end_matches('_').to_string()
}

/// Map a source-specific type to a canonical type
///
/// Supports MySQL, PostgreSQL, and SQLite type mappings.
///
/// # Examples
///
/// ```
/// use audd_ir::{map_type_to_canonical, CanonicalType};
///
/// assert_eq!(
///     map_type_to_canonical("mysql", "VARCHAR(255)"),
///     CanonicalType::String
/// );
/// assert_eq!(
///     map_type_to_canonical("postgresql", "integer"),
///     CanonicalType::Int32
/// );
/// ```
pub fn map_type_to_canonical(source_type: &str, type_name: &str) -> CanonicalType {
    let normalized = type_name.to_lowercase();
    let base_type = extract_base_type(&normalized);

    match source_type.to_lowercase().as_str() {
        "mysql" => map_mysql_type(&base_type, &normalized, type_name),
        "postgresql" | "postgres" => map_postgres_type(&base_type, &normalized, type_name),
        "sqlite" => map_sqlite_type(&base_type, &normalized, type_name),
        _ => CanonicalType::Unknown {
            original_type: type_name.to_string(),
        },
    }
}

/// Extract base type name (before parentheses or other modifiers)
fn extract_base_type(type_str: &str) -> String {
    type_str
        .split(['(', ' '])
        .next()
        .unwrap_or(type_str)
        .to_string()
}

/// Extract precision and scale from DECIMAL(10,2)
fn extract_precision_scale(type_str: &str) -> Option<(u16, u16)> {
    let params = type_str.split('(').nth(1)?.split(')').next()?;
    let parts: Vec<&str> = params.split(',').collect();
    if parts.len() == 2 {
        let precision = parts[0].trim().parse().ok()?;
        let scale = parts[1].trim().parse().ok()?;
        Some((precision, scale))
    } else {
        None
    }
}

/// Map MySQL types to canonical types
fn map_mysql_type(base_type: &str, full_type: &str, original_type: &str) -> CanonicalType {
    match base_type {
        // Boolean
        "tinyint" if full_type.contains("(1)") => CanonicalType::Boolean,
        "bool" | "boolean" => CanonicalType::Boolean,

        // Integers
        "tinyint" | "smallint" | "mediumint" | "int" | "integer" => CanonicalType::Int32,
        "bigint" => CanonicalType::Int64,

        // Floats
        "float" => CanonicalType::Float32,
        "double" | "real" => CanonicalType::Float64,

        // Decimal
        "decimal" | "numeric" => {
            if let Some((precision, scale)) = extract_precision_scale(full_type) {
                CanonicalType::Decimal { precision, scale }
            } else {
                CanonicalType::Decimal {
                    precision: 10,
                    scale: 0,
                }
            }
        }

        // Strings
        "char" | "varchar" => CanonicalType::String,
        "text" | "tinytext" | "mediumtext" | "longtext" => CanonicalType::Text,

        // Binary
        "binary" | "varbinary" | "blob" | "tinyblob" | "mediumblob" | "longblob" => {
            CanonicalType::Binary
        }

        // Date/Time
        "date" => CanonicalType::Date,
        "time" => CanonicalType::Time,
        "datetime" => CanonicalType::DateTime,
        "timestamp" => CanonicalType::Timestamp,

        // JSON
        "json" => CanonicalType::Json,

        _ => CanonicalType::Unknown {
            original_type: original_type.to_string(),
        },
    }
}

/// Map PostgreSQL types to canonical types
fn map_postgres_type(base_type: &str, full_type: &str, original_type: &str) -> CanonicalType {
    match base_type {
        // Boolean
        "boolean" | "bool" => CanonicalType::Boolean,

        // Integers
        "smallint" | "int2" => CanonicalType::Int32,
        "integer" | "int" | "int4" => CanonicalType::Int32,
        "bigint" | "int8" => CanonicalType::Int64,

        // Floats
        "real" | "float4" => CanonicalType::Float32,
        "double" | "float8" => CanonicalType::Float64,

        // Decimal
        "decimal" | "numeric" => {
            if let Some((precision, scale)) = extract_precision_scale(full_type) {
                CanonicalType::Decimal { precision, scale }
            } else {
                CanonicalType::Decimal {
                    precision: 38,
                    scale: 10,
                }
            }
        }

        // Strings
        "char" | "character" | "varchar" | "character varying" => CanonicalType::String,
        "text" => CanonicalType::Text,

        // Binary
        "bytea" => CanonicalType::Binary,

        // Date/Time
        "date" => CanonicalType::Date,
        "time" => CanonicalType::Time,
        "timestamp" => CanonicalType::DateTime,
        "timestamptz" | "timestamp with time zone" => CanonicalType::Timestamp,

        // JSON
        "json" | "jsonb" => CanonicalType::Json,

        // UUID
        "uuid" => CanonicalType::Uuid,

        _ => CanonicalType::Unknown {
            original_type: original_type.to_string(),
        },
    }
}

/// Map SQLite types to canonical types
fn map_sqlite_type(base_type: &str, _full_type: &str, original_type: &str) -> CanonicalType {
    match base_type {
        "integer" | "int" => CanonicalType::Int64, // SQLite uses 64-bit integers
        "real" | "float" | "double" => CanonicalType::Float64,
        "text" | "varchar" | "char" => CanonicalType::Text,
        "blob" => CanonicalType::Binary,
        "numeric" | "decimal" => CanonicalType::Decimal {
            precision: 38,
            scale: 10,
        },
        "boolean" | "bool" => CanonicalType::Boolean,
        "date" => CanonicalType::Date,
        "datetime" | "timestamp" => CanonicalType::DateTime,
        _ => CanonicalType::Unknown {
            original_type: original_type.to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_identifier_simple() {
        assert_eq!(normalize_identifier("UserEmail"), "user_email");
        assert_eq!(normalize_identifier("user_email"), "user_email");
        assert_eq!(normalize_identifier("USEREMAIL"), "useremail");
    }

    #[test]
    fn test_normalize_identifier_spaces() {
        assert_eq!(normalize_identifier("Product Name"), "product_name");
        assert_eq!(normalize_identifier("  Product   Name  "), "product_name");
        assert_eq!(normalize_identifier("Customer ID"), "customer_id");
    }

    #[test]
    fn test_normalize_identifier_camelcase() {
        assert_eq!(normalize_identifier("firstName"), "first_name");
        assert_eq!(normalize_identifier("lastName"), "last_name");
        assert_eq!(normalize_identifier("getUserById"), "get_user_by_id");
    }

    #[test]
    fn test_normalize_identifier_special_chars() {
        assert_eq!(normalize_identifier("user-name"), "user_name");
        assert_eq!(normalize_identifier("user.name"), "user_name");
        assert_eq!(normalize_identifier("user/name"), "user_name");
        assert_eq!(normalize_identifier("user\\name"), "user_name");
    }

    #[test]
    fn test_normalize_identifier_already_normalized() {
        assert_eq!(normalize_identifier("user_id"), "user_id");
        assert_eq!(normalize_identifier("created_at"), "created_at");
    }

    #[test]
    fn test_normalize_identifier_numbers() {
        assert_eq!(normalize_identifier("user123"), "user123");
        assert_eq!(normalize_identifier("User123Name"), "user123_name");
    }

    #[test]
    fn test_mysql_types() {
        assert_eq!(map_type_to_canonical("mysql", "INT"), CanonicalType::Int32);
        assert_eq!(
            map_type_to_canonical("mysql", "BIGINT"),
            CanonicalType::Int64
        );
        assert_eq!(
            map_type_to_canonical("mysql", "VARCHAR(255)"),
            CanonicalType::String
        );
        assert_eq!(map_type_to_canonical("mysql", "TEXT"), CanonicalType::Text);
        assert_eq!(
            map_type_to_canonical("mysql", "TINYINT(1)"),
            CanonicalType::Boolean
        );
        assert_eq!(
            map_type_to_canonical("mysql", "DECIMAL(10,2)"),
            CanonicalType::Decimal {
                precision: 10,
                scale: 2
            }
        );
    }

    #[test]
    fn test_postgres_types() {
        assert_eq!(
            map_type_to_canonical("postgresql", "integer"),
            CanonicalType::Int32
        );
        assert_eq!(
            map_type_to_canonical("postgresql", "bigint"),
            CanonicalType::Int64
        );
        assert_eq!(
            map_type_to_canonical("postgresql", "text"),
            CanonicalType::Text
        );
        assert_eq!(
            map_type_to_canonical("postgresql", "boolean"),
            CanonicalType::Boolean
        );
        assert_eq!(
            map_type_to_canonical("postgresql", "uuid"),
            CanonicalType::Uuid
        );
        assert_eq!(
            map_type_to_canonical("postgresql", "timestamptz"),
            CanonicalType::Timestamp
        );
    }

    #[test]
    fn test_sqlite_types() {
        assert_eq!(
            map_type_to_canonical("sqlite", "INTEGER"),
            CanonicalType::Int64
        );
        assert_eq!(
            map_type_to_canonical("sqlite", "REAL"),
            CanonicalType::Float64
        );
        assert_eq!(map_type_to_canonical("sqlite", "TEXT"), CanonicalType::Text);
        assert_eq!(
            map_type_to_canonical("sqlite", "BLOB"),
            CanonicalType::Binary
        );
    }

    #[test]
    fn test_unknown_types() {
        let result = map_type_to_canonical("mysql", "CUSTOM_TYPE");
        match result {
            CanonicalType::Unknown { original_type } => {
                assert_eq!(original_type, "CUSTOM_TYPE");
            }
            _ => panic!("Expected Unknown type"),
        }
    }

    #[test]
    fn test_case_insensitive_mapping() {
        assert_eq!(
            map_type_to_canonical("mysql", "int"),
            map_type_to_canonical("mysql", "INT")
        );
        assert_eq!(
            map_type_to_canonical("postgresql", "INTEGER"),
            map_type_to_canonical("postgresql", "integer")
        );
    }

    #[test]
    fn test_date_time_types() {
        assert_eq!(map_type_to_canonical("mysql", "DATE"), CanonicalType::Date);
        assert_eq!(map_type_to_canonical("mysql", "TIME"), CanonicalType::Time);
        assert_eq!(
            map_type_to_canonical("mysql", "DATETIME"),
            CanonicalType::DateTime
        );
        assert_eq!(
            map_type_to_canonical("mysql", "TIMESTAMP"),
            CanonicalType::Timestamp
        );
    }
}
