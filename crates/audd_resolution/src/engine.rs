//! Suggestion engine for generating resolution suggestions from conflicts

use audd_compare::{Conflict, ConflictType};

use crate::config::ResolutionConfig;
use crate::suggestion::{Confidence, Impact, Suggestion, SuggestionKind};

/// Counter for generating unique suggestion IDs
static SUGGESTION_COUNTER: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

/// Generate a unique suggestion ID
fn generate_suggestion_id() -> String {
    let count = SUGGESTION_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    format!("sug_{}", count)
}

/// Engine for generating suggestions from conflicts
pub struct SuggestionEngine {
    /// Configuration for resolution policies
    config: ResolutionConfig,
}

impl SuggestionEngine {
    /// Create a new suggestion engine with default settings
    pub fn new() -> Self {
        Self {
            config: ResolutionConfig::default(),
        }
    }

    /// Create an engine with custom configuration
    pub fn with_config(config: ResolutionConfig) -> Self {
        Self { config }
    }

    /// Create an engine that only generates safe suggestions
    pub fn safe_only() -> Self {
        Self {
            config: ResolutionConfig::conservative(),
        }
    }

    /// Get the current configuration
    pub fn config(&self) -> &ResolutionConfig {
        &self.config
    }

    /// Generate suggestions for a single conflict
    pub fn suggest(&self, conflict: &Conflict) -> Vec<Suggestion> {
        match conflict.conflict_type {
            ConflictType::TypeIncompatible => self.suggest_type_cast(conflict),
            ConflictType::NormalizationCollision => self.suggest_rename(conflict),
            ConflictType::NullabilityMismatch => self.suggest_nullability(conflict),
            ConflictType::ConstraintMismatch => self.suggest_constraint(conflict),
            ConflictType::LengthMismatch => self.suggest_length(conflict),
        }
    }

    /// Generate suggestions for type incompatibility
    fn suggest_type_cast(&self, conflict: &Conflict) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        // Extract type information from evidence
        let type_a = extract_type_from_evidence(&conflict.evidence.from_a);
        let type_b = extract_type_from_evidence(&conflict.evidence.from_b);

        let field_name = conflict
            .field_name
            .clone()
            .unwrap_or_else(|| "unknown".to_string());

        // Check if cast is safe or risky
        if let Some((safe, warning)) = analyze_cast_safety(&type_a, &type_b) {
            if safe {
                // Safe cast (e.g., Int32 -> Int64, Int -> Float)
                suggestions.push(Suggestion::cast_safe(
                    generate_suggestion_id(),
                    conflict.entity_name.clone(),
                    field_name.clone(),
                    type_a.clone(),
                    type_b.clone(),
                    format!(
                        "Safe widening cast from {} to {} - no data loss expected",
                        type_a, type_b
                    ),
                ));
            } else if self.config.allow_risky_suggestions {
                // Risky cast (e.g., Int64 -> Int32, Float -> Int)
                suggestions.push(Suggestion::cast_risky(
                    generate_suggestion_id(),
                    conflict.entity_name.clone(),
                    field_name.clone(),
                    type_a.clone(),
                    type_b.clone(),
                    warning.clone(),
                    format!(
                        "Narrowing or lossy cast from {} to {} - {}",
                        type_a, type_b, warning
                    ),
                ));
            }
        }

        // If no cast suggestions, provide a no-suggestion marker
        if suggestions.is_empty() {
            suggestions.push(Suggestion::no_suggestion(
                generate_suggestion_id(),
                conflict.entity_name.clone(),
                Some(field_name),
                format!(
                    "Type incompatibility between {} and {} cannot be automatically resolved",
                    type_a, type_b
                ),
            ));
        }

        suggestions
    }

    /// Generate suggestions for normalization collisions
    fn suggest_rename(&self, conflict: &Conflict) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        // Extract field names from collision evidence
        let (name_a, name_b) = extract_collision_names(&conflict.evidence);

        // Generate rename suggestions with different strategies
        // Strategy 1: Suffix with _a and _b
        suggestions.push(Suggestion::rename_field(
            generate_suggestion_id(),
            conflict.entity_name.clone(),
            name_a.clone(),
            format!("{}_a", normalize_name(&name_a)),
            "Normalization collision".to_string(),
            format!(
                "Add suffix '_a' to distinguish from conflicting field '{}'",
                name_b
            ),
        ));

        suggestions.push(Suggestion::rename_field(
            generate_suggestion_id(),
            conflict.entity_name.clone(),
            name_b.clone(),
            format!("{}_b", normalize_name(&name_b)),
            "Normalization collision".to_string(),
            format!(
                "Add suffix '_b' to distinguish from conflicting field '{}'",
                name_a
            ),
        ));

        // Strategy 2: Preserve original casing in suffix
        if name_a != normalize_name(&name_a) {
            suggestions.push(Suggestion::rename_field(
                generate_suggestion_id(),
                conflict.entity_name.clone(),
                name_a.clone(),
                format!("{}_{}", normalize_name(&name_a), "src_a"),
                "Preserve original distinction".to_string(),
                format!("Keep normalized name but add source suffix to preserve distinction"),
            ));
        }

        suggestions
    }

    /// Generate suggestions for nullability mismatches
    fn suggest_nullability(&self, conflict: &Conflict) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        let field_name = conflict
            .field_name
            .clone()
            .unwrap_or_else(|| "unknown".to_string());

        // Extract nullability from evidence
        let nullable_a = conflict.evidence.from_a.contains("true");
        let nullable_b = conflict.evidence.from_b.contains("true");

        if !nullable_a && nullable_b {
            // A is NOT NULL, B is NULL -> can make unified field nullable
            suggestions.push(Suggestion::new(
                generate_suggestion_id(),
                conflict.entity_name.clone(),
                Some(field_name.clone()),
                SuggestionKind::PreferType {
                    preferred_type: "NULLABLE".to_string(),
                    alternative_type: "NOT NULL".to_string(),
                    rule: "Allow nullable to accommodate both schemas".to_string(),
                },
                Confidence::high(),
                "Make field nullable to accommodate schema B".to_string(),
                vec![
                    "Schema A: NOT NULL".to_string(),
                    "Schema B: NULLABLE".to_string(),
                    "Nullable is more permissive".to_string(),
                ],
                Impact::Low,
            ));
        } else if nullable_a && !nullable_b {
            // A is NULL, B is NOT NULL -> can make unified field nullable
            suggestions.push(Suggestion::new(
                generate_suggestion_id(),
                conflict.entity_name.clone(),
                Some(field_name.clone()),
                SuggestionKind::PreferType {
                    preferred_type: "NULLABLE".to_string(),
                    alternative_type: "NOT NULL".to_string(),
                    rule: "Allow nullable to accommodate both schemas".to_string(),
                },
                Confidence::high(),
                "Make field nullable to accommodate schema A".to_string(),
                vec![
                    "Schema A: NULLABLE".to_string(),
                    "Schema B: NOT NULL".to_string(),
                    "Nullable is more permissive".to_string(),
                ],
                Impact::Low,
            ));
        }

        if suggestions.is_empty() {
            suggestions.push(Suggestion::no_suggestion(
                generate_suggestion_id(),
                conflict.entity_name.clone(),
                Some(field_name),
                "Nullability conflict resolution requires manual decision".to_string(),
            ));
        }

        suggestions
    }

    /// Generate suggestions for constraint mismatches
    fn suggest_constraint(&self, _conflict: &Conflict) -> Vec<Suggestion> {
        // For MVP, constraint mismatches require manual resolution
        vec![Suggestion::no_suggestion(
            generate_suggestion_id(),
            _conflict.entity_name.clone(),
            _conflict.field_name.clone(),
            "Constraint mismatches require manual review and resolution".to_string(),
        )]
    }

    /// Generate suggestions for length/precision mismatches
    fn suggest_length(&self, conflict: &Conflict) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        let field_name = conflict
            .field_name
            .clone()
            .unwrap_or_else(|| "unknown".to_string());

        // Suggest using the larger length/precision
        suggestions.push(Suggestion::new(
            generate_suggestion_id(),
            conflict.entity_name.clone(),
            Some(field_name.clone()),
            SuggestionKind::PreferType {
                preferred_type: "Larger length/precision".to_string(),
                alternative_type: "Smaller length/precision".to_string(),
                rule: "Use maximum length to accommodate both schemas".to_string(),
            },
            Confidence::high(),
            "Use the larger length/precision to ensure no data truncation".to_string(),
            vec![
                conflict.evidence.from_a.clone(),
                conflict.evidence.from_b.clone(),
                "Larger size is safer".to_string(),
            ],
            Impact::Low,
        ));

        suggestions
    }
}

impl Default for SuggestionEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Extract type name from evidence string
fn extract_type_from_evidence(evidence: &str) -> String {
    // Evidence format is typically "Type: TypeName"
    if let Some(pos) = evidence.find("Type: ") {
        evidence[pos + 6..].trim().to_string()
    } else {
        evidence.trim().to_string()
    }
}

/// Extract field names from collision evidence
fn extract_collision_names(evidence: &audd_compare::ConflictEvidence) -> (String, String) {
    // Evidence format: "Field 'name' normalizes to 'normalized'"
    let extract_name = |s: &str| -> String {
        if let Some(start) = s.find("Field '") {
            if let Some(end) = s[start + 7..].find('\'') {
                return s[start + 7..start + 7 + end].to_string();
            }
        }
        "unknown".to_string()
    };

    (
        extract_name(&evidence.from_a),
        extract_name(&evidence.from_b),
    )
}

/// Normalize a field name (lowercase, no special chars)
fn normalize_name(name: &str) -> String {
    name.to_lowercase()
}

/// Analyze whether a type cast is safe or risky
/// Returns (is_safe, warning_message)
fn analyze_cast_safety(from_type: &str, to_type: &str) -> Option<(bool, String)> {
    let from_lower = from_type.to_lowercase();
    let to_lower = to_type.to_lowercase();

    // Integer widening (safe)
    if (from_lower.contains("int8") || from_lower.contains("tinyint"))
        && (to_lower.contains("int16")
            || to_lower.contains("int32")
            || to_lower.contains("int64")
            || to_lower.contains("bigint"))
    {
        return Some((true, String::new()));
    }

    if (from_lower.contains("int16") || from_lower.contains("smallint"))
        && (to_lower.contains("int32") || to_lower.contains("int64") || to_lower.contains("bigint"))
    {
        return Some((true, String::new()));
    }

    if (from_lower.contains("int32") || from_lower == "int")
        && (to_lower.contains("int64") || to_lower.contains("bigint"))
    {
        return Some((true, String::new()));
    }

    // Integer to float (safe with potential precision loss for very large integers)
    if (from_lower.contains("int") || from_lower.contains("integer"))
        && (to_lower.contains("float") || to_lower.contains("double") || to_lower.contains("decimal"))
    {
        return Some((
            true,
            "Possible precision loss for very large integers".to_string(),
        ));
    }

    // Integer narrowing (risky)
    if (from_lower.contains("int64") || from_lower.contains("bigint"))
        && (to_lower.contains("int32")
            || to_lower.contains("int16")
            || to_lower.contains("int8")
            || to_lower == "int")
    {
        return Some((
            false,
            "Potential overflow if values exceed target range".to_string(),
        ));
    }

    if (from_lower.contains("int32") || from_lower == "int")
        && (to_lower.contains("int16") || to_lower.contains("int8"))
    {
        return Some((
            false,
            "Potential overflow if values exceed target range".to_string(),
        ));
    }

    // Float to integer (risky - loses decimal)
    if (from_lower.contains("float") || from_lower.contains("double") || from_lower.contains("decimal"))
        && to_lower.contains("int")
    {
        return Some((false, "Loss of decimal precision".to_string()));
    }

    // String length increase (safe)
    if from_lower.contains("varchar") && to_lower.contains("text") {
        return Some((true, String::new()));
    }

    // Text to varchar (risky)
    if from_lower.contains("text") && to_lower.contains("varchar") {
        return Some((false, "Potential truncation of long text".to_string()));
    }

    // No cast analysis available
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use audd_compare::ConflictEvidence;

    #[test]
    fn test_safe_integer_widening() {
        let conflict = Conflict::type_incompatible(
            "users".to_string(),
            "id".to_string(),
            "Int32".to_string(),
            "Int64".to_string(),
            0,
            1,
        );

        let engine = SuggestionEngine::new();
        let suggestions = engine.suggest(&conflict);

        assert!(!suggestions.is_empty());
        assert!(matches!(
            suggestions[0].kind,
            SuggestionKind::CastSafe { .. }
        ));
    }

    #[test]
    fn test_risky_integer_narrowing() {
        let conflict = Conflict::type_incompatible(
            "users".to_string(),
            "big_number".to_string(),
            "Int64".to_string(),
            "Int32".to_string(),
            0,
            1,
        );

        let engine = SuggestionEngine::new();
        let suggestions = engine.suggest(&conflict);

        assert!(!suggestions.is_empty());
        assert!(matches!(
            suggestions[0].kind,
            SuggestionKind::CastRisky { .. }
        ));
    }

    #[test]
    fn test_safe_only_engine() {
        let conflict = Conflict::type_incompatible(
            "users".to_string(),
            "big_number".to_string(),
            "Int64".to_string(),
            "Int32".to_string(),
            0,
            1,
        );

        let engine = SuggestionEngine::safe_only();
        let suggestions = engine.suggest(&conflict);

        // Should provide NoSuggestion since risky casts are disabled
        assert!(!suggestions.is_empty());
        assert!(matches!(
            suggestions[0].kind,
            SuggestionKind::NoSuggestion { .. }
        ));
    }

    #[test]
    fn test_normalization_collision_suggestions() {
        let conflict = Conflict::normalization_collision(
            "users".to_string(),
            "UserID".to_string(),
            "user_id".to_string(),
            "user_id".to_string(),
            0,
            1,
        );

        let engine = SuggestionEngine::new();
        let suggestions = engine.suggest(&conflict);

        // Should suggest multiple rename strategies
        assert!(suggestions.len() >= 2);
        assert!(suggestions
            .iter()
            .all(|s| matches!(s.kind, SuggestionKind::RenameField { .. })));
    }

    #[test]
    fn test_nullability_mismatch_suggestions() {
        let conflict = Conflict::nullability_mismatch(
            "users".to_string(),
            "email".to_string(),
            false,
            true,
            0,
            1,
        );

        let engine = SuggestionEngine::new();
        let suggestions = engine.suggest(&conflict);

        assert!(!suggestions.is_empty());
        // Should suggest making it nullable
        assert!(matches!(
            suggestions[0].kind,
            SuggestionKind::PreferType { .. }
        ));
    }

    #[test]
    fn test_constraint_mismatch_no_suggestion() {
        let conflict = Conflict::constraint_mismatch(
            "users".to_string(),
            "email".to_string(),
            "UNIQUE".to_string(),
            "NOT UNIQUE".to_string(),
            0,
            1,
        );

        let engine = SuggestionEngine::new();
        let suggestions = engine.suggest(&conflict);

        assert_eq!(suggestions.len(), 1);
        assert!(matches!(
            suggestions[0].kind,
            SuggestionKind::NoSuggestion { .. }
        ));
    }

    #[test]
    fn test_extract_type_from_evidence() {
        assert_eq!(
            extract_type_from_evidence("Type: Int32"),
            "Int32".to_string()
        );
        assert_eq!(
            extract_type_from_evidence("Type: VARCHAR(255)"),
            "VARCHAR(255)".to_string()
        );
    }

    #[test]
    fn test_analyze_cast_safety() {
        // Safe casts
        assert_eq!(analyze_cast_safety("Int32", "Int64"), Some((true, String::new())));
        assert_eq!(
            analyze_cast_safety("Int16", "Int32"),
            Some((true, String::new()))
        );

        // Risky casts
        let (safe, warning) = analyze_cast_safety("Int64", "Int32").unwrap();
        assert!(!safe);
        assert!(warning.contains("overflow"));

        let (safe, warning) = analyze_cast_safety("Float", "Int").unwrap();
        assert!(!safe);
        assert!(warning.contains("decimal"));
    }

    #[test]
    fn test_engine_with_custom_config() {
        use crate::config::ResolutionConfig;

        let conflict = Conflict::type_incompatible(
            "users".to_string(),
            "big_number".to_string(),
            "Int64".to_string(),
            "Int32".to_string(),
            0,
            1,
        );

        // Conservative config should not suggest risky casts
        let conservative_config = ResolutionConfig::conservative();
        let engine = SuggestionEngine::with_config(conservative_config);
        let suggestions = engine.suggest(&conflict);

        assert!(!suggestions.is_empty());
        assert!(matches!(
            suggestions[0].kind,
            SuggestionKind::NoSuggestion { .. }
        ));

        // Default config should suggest risky casts
        let default_engine = SuggestionEngine::new();
        let suggestions = default_engine.suggest(&conflict);

        assert!(!suggestions.is_empty());
        assert!(matches!(
            suggestions[0].kind,
            SuggestionKind::CastRisky { .. }
        ));
    }
}
