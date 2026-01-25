//! Test fixtures for conflict scenarios

use audd_compare::Conflict;

/// Generate a comprehensive set of conflict fixtures covering all conflict types
pub fn comprehensive_conflict_fixtures() -> Vec<Conflict> {
    vec![
        // Type incompatibility - safe widening
        Conflict::type_incompatible(
            "users".to_string(),
            "id".to_string(),
            "Int32".to_string(),
            "Int64".to_string(),
            0,
            1,
        ),
        // Type incompatibility - risky narrowing
        Conflict::type_incompatible(
            "products".to_string(),
            "price".to_string(),
            "Decimal(10,2)".to_string(),
            "Float".to_string(),
            0,
            1,
        ),
        // Type incompatibility - incompatible types
        Conflict::type_incompatible(
            "orders".to_string(),
            "status".to_string(),
            "String".to_string(),
            "Integer".to_string(),
            0,
            1,
        ),
        // Normalization collision - case difference
        Conflict::normalization_collision(
            "customers".to_string(),
            "UserID".to_string(),
            "user_id".to_string(),
            "user_id".to_string(),
            0,
            1,
        ),
        // Normalization collision - different names
        Conflict::normalization_collision(
            "employees".to_string(),
            "emp_id".to_string(),
            "EmpID".to_string(),
            "emp_id".to_string(),
            0,
            1,
        ),
        // Nullability mismatch - NOT NULL vs NULL
        Conflict::nullability_mismatch(
            "users".to_string(),
            "email".to_string(),
            false,
            true,
            0,
            1,
        ),
        // Nullability mismatch - NULL vs NOT NULL
        Conflict::nullability_mismatch(
            "products".to_string(),
            "description".to_string(),
            true,
            false,
            0,
            1,
        ),
        // Constraint mismatch
        Conflict::constraint_mismatch(
            "users".to_string(),
            "username".to_string(),
            "UNIQUE".to_string(),
            "NOT UNIQUE".to_string(),
            0,
            1,
        ),
        // Constraint mismatch - primary key
        Conflict::constraint_mismatch(
            "orders".to_string(),
            "id".to_string(),
            "PRIMARY KEY".to_string(),
            "INDEX".to_string(),
            0,
            1,
        ),
    ]
}

/// Generate fixtures for type cast scenarios
pub fn type_cast_fixtures() -> Vec<Conflict> {
    vec![
        // Integer widening (safe)
        Conflict::type_incompatible(
            "table1".to_string(),
            "field1".to_string(),
            "Int8".to_string(),
            "Int16".to_string(),
            0,
            1,
        ),
        Conflict::type_incompatible(
            "table1".to_string(),
            "field2".to_string(),
            "Int16".to_string(),
            "Int32".to_string(),
            0,
            1,
        ),
        Conflict::type_incompatible(
            "table1".to_string(),
            "field3".to_string(),
            "Int32".to_string(),
            "Int64".to_string(),
            0,
            1,
        ),
        // Integer narrowing (risky)
        Conflict::type_incompatible(
            "table2".to_string(),
            "field1".to_string(),
            "Int64".to_string(),
            "Int32".to_string(),
            0,
            1,
        ),
        Conflict::type_incompatible(
            "table2".to_string(),
            "field2".to_string(),
            "Int32".to_string(),
            "Int16".to_string(),
            0,
            1,
        ),
        // Float to integer (risky)
        Conflict::type_incompatible(
            "table3".to_string(),
            "field1".to_string(),
            "Float".to_string(),
            "Int32".to_string(),
            0,
            1,
        ),
        Conflict::type_incompatible(
            "table3".to_string(),
            "field2".to_string(),
            "Double".to_string(),
            "Int64".to_string(),
            0,
            1,
        ),
    ]
}

/// Generate fixtures for naming collision scenarios
pub fn naming_collision_fixtures() -> Vec<Conflict> {
    vec![
        // Case variations
        Conflict::normalization_collision(
            "users".to_string(),
            "UserName".to_string(),
            "user_name".to_string(),
            "user_name".to_string(),
            0,
            1,
        ),
        Conflict::normalization_collision(
            "users".to_string(),
            "EMAIL".to_string(),
            "email".to_string(),
            "email".to_string(),
            0,
            1,
        ),
        // Underscore vs camelCase
        Conflict::normalization_collision(
            "products".to_string(),
            "product_id".to_string(),
            "ProductID".to_string(),
            "product_id".to_string(),
            0,
            1,
        ),
        // Mixed variations
        Conflict::normalization_collision(
            "orders".to_string(),
            "order-id".to_string(),
            "order_id".to_string(),
            "order_id".to_string(),
            0,
            1,
        ),
    ]
}

/// Generate fixtures for nullability scenarios
pub fn nullability_fixtures() -> Vec<Conflict> {
    vec![
        Conflict::nullability_mismatch(
            "users".to_string(),
            "first_name".to_string(),
            false,
            true,
            0,
            1,
        ),
        Conflict::nullability_mismatch(
            "users".to_string(),
            "last_name".to_string(),
            true,
            false,
            0,
            1,
        ),
        Conflict::nullability_mismatch(
            "orders".to_string(),
            "notes".to_string(),
            false,
            true,
            0,
            1,
        ),
    ]
}

/// Generate fixtures for constraint scenarios
pub fn constraint_fixtures() -> Vec<Conflict> {
    vec![
        Conflict::constraint_mismatch(
            "users".to_string(),
            "email".to_string(),
            "UNIQUE".to_string(),
            "INDEX".to_string(),
            0,
            1,
        ),
        Conflict::constraint_mismatch(
            "products".to_string(),
            "sku".to_string(),
            "UNIQUE NOT NULL".to_string(),
            "NULL".to_string(),
            0,
            1,
        ),
        Conflict::constraint_mismatch(
            "orders".to_string(),
            "id".to_string(),
            "PRIMARY KEY".to_string(),
            "UNIQUE".to_string(),
            0,
            1,
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comprehensive_fixtures_not_empty() {
        let fixtures = comprehensive_conflict_fixtures();
        assert!(!fixtures.is_empty());
        assert!(fixtures.len() >= 5, "Should have diverse conflict types");
    }

    #[test]
    fn test_type_cast_fixtures_coverage() {
        let fixtures = type_cast_fixtures();
        assert!(!fixtures.is_empty());
        
        // Should have both safe and risky casts
        let all_type_conflicts = fixtures.iter().all(|f| {
            matches!(f.conflict_type, audd_compare::ConflictType::TypeIncompatible)
        });
        assert!(all_type_conflicts);
    }

    #[test]
    fn test_naming_collision_fixtures() {
        let fixtures = naming_collision_fixtures();
        assert!(!fixtures.is_empty());
        
        let all_collisions = fixtures.iter().all(|f| {
            matches!(f.conflict_type, audd_compare::ConflictType::NormalizationCollision)
        });
        assert!(all_collisions);
    }

    #[test]
    fn test_nullability_fixtures() {
        let fixtures = nullability_fixtures();
        assert!(!fixtures.is_empty());
        
        let all_nullability = fixtures.iter().all(|f| {
            matches!(f.conflict_type, audd_compare::ConflictType::NullabilityMismatch)
        });
        assert!(all_nullability);
    }

    #[test]
    fn test_constraint_fixtures() {
        let fixtures = constraint_fixtures();
        assert!(!fixtures.is_empty());
        
        let all_constraints = fixtures.iter().all(|f| {
            matches!(f.conflict_type, audd_compare::ConflictType::ConstraintMismatch)
        });
        assert!(all_constraints);
    }
}
