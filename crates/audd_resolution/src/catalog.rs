//! Resolution catalog support: versioned rules and domain equivalences.

use audd_compare::{Conflict, ConflictType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::suggestion::{Confidence, Impact, Suggestion, SuggestionKind};

/// A single resolution rule.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolutionRule {
    pub id: String,
    pub conflict_type: String,
    #[serde(default)]
    pub when: HashMap<String, String>,
    /// `from`/`to` also match with A and B swapped (e.g. "Int32 vs Int64")
    #[serde(default)]
    pub symmetric: bool,
    pub suggestion: String,
    #[serde(default)]
    pub params: HashMap<String, String>,
    pub confidence: f64,
    pub rationale: String,
    #[serde(default = "default_source")]
    pub source: String,
    #[serde(default = "default_version")]
    pub version: u32,
}

fn default_source() -> String {
    "default".to_string()
}

fn default_version() -> u32 {
    1
}

/// Domain equivalence entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Equivalence {
    pub canonical: String,
    pub aliases: Vec<String>,
}

/// Domain catalog (e.g., point of sale).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainCatalog {
    pub domain: DomainHeader,
    #[serde(default)]
    pub equivalence: Vec<Equivalence>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainHeader {
    pub name: String,
    #[serde(default)]
    pub description: String,
}

/// Wrapper for parsing `resolutions.toml` with `[[rule]]` arrays.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ResolutionFile {
    rule: Vec<ResolutionRule>,
}

/// Loaded catalog: rules plus optional domain equivalences.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ResolutionCatalog {
    pub rules: Vec<ResolutionRule>,
    pub domains: HashMap<String, DomainCatalog>,
    pub path: Option<PathBuf>,
}

impl ResolutionCatalog {
    /// Create an empty catalog.
    pub fn new() -> Self {
        Self::default()
    }

    /// Load catalog from a directory.
    ///
    /// Expected layout:
    /// - `<dir>/resolutions.toml`
    /// - `<dir>/domains/*.toml`
    pub fn load_from_dir<P: AsRef<Path>>(dir: P) -> Result<Self, Box<dyn std::error::Error>> {
        let dir = dir.as_ref();
        let mut catalog = Self::new();
        catalog.path = Some(dir.to_path_buf());

        let resolutions_path = dir.join("resolutions.toml");
        if resolutions_path.exists() {
            let content = std::fs::read_to_string(&resolutions_path)?;
            let file: ResolutionFile = toml::from_str(&content)?;
            catalog.rules = file.rule;
        }

        let domains_dir = dir.join("domains");
        if domains_dir.exists() {
            for entry in std::fs::read_dir(&domains_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("toml") {
                    let content = std::fs::read_to_string(&path)?;
                    let domain: DomainCatalog = toml::from_str(&content)?;
                    catalog.domains.insert(domain.domain.name.clone(), domain);
                }
            }
        }

        Ok(catalog)
    }

    /// Load the default catalog from `./catalog` if it exists.
    pub fn load_default() -> Self {
        Self::load_from_dir("catalog").unwrap_or_default()
    }

    /// Return aliases for a given active domain, mapped to their canonical form.
    pub fn domain_aliases(&self, domain_name: &str) -> HashMap<String, String> {
        let mut aliases = HashMap::new();
        if let Some(domain) = self.domains.get(domain_name) {
            for equiv in &domain.equivalence {
                for alias in &equiv.aliases {
                    aliases.insert(alias.clone(), equiv.canonical.clone());
                }
            }
        }
        aliases
    }

    /// Find the first matching rule for a conflict.
    pub fn find_rule(&self, conflict: &Conflict) -> Option<&ResolutionRule> {
        self.rules.iter().find(|rule| rule_matches(rule, conflict))
    }

    /// Build a suggestion from a matching rule.
    pub fn suggest(&self, conflict: &Conflict, suggestion_id: String) -> Option<Suggestion> {
        let rule = self.find_rule(conflict)?;
        Some(rule_to_suggestion(rule, conflict, suggestion_id))
    }
}

fn rule_matches(rule: &ResolutionRule, conflict: &Conflict) -> bool {
    let expected_type = match conflict.conflict_type {
        ConflictType::TypeIncompatible => "TypeIncompatible",
        ConflictType::NullabilityMismatch => "NullabilityMismatch",
        ConflictType::ConstraintMismatch => "ConstraintMismatch",
        ConflictType::LengthMismatch => "LengthMismatch",
        ConflictType::NormalizationCollision => "NormalizationCollision",
    };

    if rule.conflict_type != expected_type {
        return false;
    }

    // Types are compared by name in the A -> B direction; `|` separates alternatives
    let type_a = evidence_type(&conflict.evidence.from_a);
    let type_b = evidence_type(&conflict.evidence.from_b);
    let from = rule.when.get("from");
    let to = rule.when.get("to");
    if from.is_some() || to.is_some() {
        let accepts = |cond: Option<&String>, ty: &str| {
            cond.is_none_or(|c| c.split('|').any(|alt| alt.trim() == ty))
        };
        let forward = accepts(from, type_a) && accepts(to, type_b);
        let backward = rule.symmetric && accepts(from, type_b) && accepts(to, type_a);
        if !forward && !backward {
            return false;
        }
    }

    for (key, value) in &rule.when {
        match key.as_str() {
            "from" | "to" => {}
            // Constraint kind or preferred value: accept if the evidence mentions it
            _ => {
                if !conflict.evidence.from_a.contains(value)
                    && !conflict.evidence.from_b.contains(value)
                {
                    return false;
                }
            }
        }
    }

    true
}

/// Base type name from evidence such as `Type: Decimal(10, 2)` -> `Decimal`
fn evidence_type(evidence: &str) -> &str {
    let ty = evidence.strip_prefix("Type: ").unwrap_or(evidence);
    ty.split('(').next().unwrap_or(ty).trim()
}

fn rule_to_suggestion(
    rule: &ResolutionRule,
    conflict: &Conflict,
    suggestion_id: String,
) -> Suggestion {
    let field_name = conflict
        .field_name
        .clone()
        .unwrap_or_else(|| "unknown".to_string());

    let kind = match rule.suggestion.as_str() {
        "CastSafe" => {
            let target = rule.params.get("target").cloned().unwrap_or_default();
            let (from_type, to_type) = extract_types(conflict, &target);
            SuggestionKind::CastSafe { from_type, to_type }
        }
        "CastRisky" => {
            let target = rule.params.get("target").cloned().unwrap_or_default();
            let (from_type, to_type) = extract_types(conflict, &target);
            let warning = rule
                .params
                .get("warning")
                .cloned()
                .unwrap_or_else(|| "Risky conversion".to_string());
            SuggestionKind::CastRisky {
                from_type,
                to_type,
                warning,
            }
        }
        "PreferType" => {
            let preferred = rule
                .params
                .get("preferred")
                .cloned()
                .unwrap_or_else(|| "preferred".to_string());
            let alternative = rule
                .params
                .get("alternative")
                .cloned()
                .unwrap_or_else(|| "alternative".to_string());
            SuggestionKind::PreferType {
                preferred_type: preferred,
                alternative_type: alternative,
                rule: rule.id.clone(),
            }
        }
        "RenameField" => {
            let suffix = rule
                .params
                .get("suffix")
                .cloned()
                .unwrap_or_else(|| "_a_or_b".to_string());
            SuggestionKind::RenameField {
                original_name: field_name.clone(),
                suggested_name: format!("{}{}", field_name, suffix),
                reason: rule.rationale.clone(),
            }
        }
        _ => SuggestionKind::NoSuggestion {
            reason: rule.rationale.clone(),
        },
    };

    Suggestion::new(
        suggestion_id,
        conflict.entity_name.clone(),
        Some(field_name),
        kind,
        Confidence::new(rule.confidence),
        rule.rationale.clone(),
        vec![format!("catalog:{}", rule.id)],
        Impact::Medium,
    )
}

fn extract_types(conflict: &Conflict, fallback_target: &str) -> (String, String) {
    let from_a = conflict.evidence.from_a.clone();
    let from_b = conflict.evidence.from_b.clone();

    let type_a = if from_a.starts_with("Type: ") {
        from_a.strip_prefix("Type: ").unwrap().to_string()
    } else {
        from_a.clone()
    };

    let type_b = if from_b.starts_with("Type: ") {
        from_b.strip_prefix("Type: ").unwrap().to_string()
    } else {
        from_b.clone()
    };

    if fallback_target.is_empty() {
        (type_a, type_b)
    } else {
        (type_a, fallback_target.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use audd_compare::{ConflictEvidence, ConflictSeverity};

    fn sample_conflict() -> Conflict {
        Conflict::new(
            "users".to_string(),
            Some("id".to_string()),
            ConflictType::TypeIncompatible,
            ConflictSeverity::High,
            ConflictEvidence::new(
                "Type: Int32".to_string(),
                "Type: Int64".to_string(),
                "Types must be compatible".to_string(),
            ),
            0,
            1,
        )
    }

    #[test]
    fn test_rule_matches_type() {
        let rule = ResolutionRule {
            id: "R-TYPE-001".to_string(),
            conflict_type: "TypeIncompatible".to_string(),
            when: [
                ("from".to_string(), "Int32".to_string()),
                ("to".to_string(), "Int64".to_string()),
            ]
            .into(),
            symmetric: false,
            suggestion: "CastSafe".to_string(),
            params: [("target".to_string(), "Int64".to_string())].into(),
            confidence: 0.95,
            rationale: "Safe widening".to_string(),
            source: "default".to_string(),
            version: 1,
        };

        assert!(rule_matches(&rule, &sample_conflict()));
    }

    #[test]
    fn test_rule_to_suggestion() {
        let rule = ResolutionRule {
            id: "R-TYPE-001".to_string(),
            conflict_type: "TypeIncompatible".to_string(),
            when: HashMap::new(),
            symmetric: false,
            suggestion: "CastSafe".to_string(),
            params: [("target".to_string(), "Int64".to_string())].into(),
            confidence: 0.95,
            rationale: "Safe widening".to_string(),
            source: "default".to_string(),
            version: 1,
        };

        let suggestion = rule_to_suggestion(&rule, &sample_conflict(), "sug_1".to_string());
        assert!(matches!(suggestion.kind, SuggestionKind::CastSafe { .. }));
        assert!(suggestion.evidence.iter().any(|e| e.contains("catalog:")));
    }

    fn type_rule(from: &str, to: &str, symmetric: bool) -> ResolutionRule {
        ResolutionRule {
            id: "R-TEST".to_string(),
            conflict_type: "TypeIncompatible".to_string(),
            when: [
                ("from".to_string(), from.to_string()),
                ("to".to_string(), to.to_string()),
            ]
            .into(),
            symmetric,
            suggestion: "CastSafe".to_string(),
            params: HashMap::new(),
            confidence: 0.9,
            rationale: "test".to_string(),
            source: "default".to_string(),
            version: 1,
        }
    }

    fn type_conflict(a: &str, b: &str) -> Conflict {
        Conflict::type_incompatible(
            "t".to_string(),
            "f".to_string(),
            a.to_string(),
            b.to_string(),
            0,
            0,
        )
    }

    #[test]
    fn test_type_rules_are_directional_unless_symmetric() {
        let directional = type_rule("Int32", "String", false);
        assert!(rule_matches(
            &directional,
            &type_conflict("Int32", "String")
        ));
        assert!(!rule_matches(
            &directional,
            &type_conflict("String", "Int32")
        ));

        let symmetric = type_rule("Int32", "Int64", true);
        assert!(rule_matches(&symmetric, &type_conflict("Int64", "Int32")));
    }

    #[test]
    fn test_type_rules_compare_whole_type_names() {
        // "Date" must not match "DateTime"; parameters are ignored; `|` lists alternatives
        assert!(!rule_matches(
            &type_rule("Date", "Timestamp", false),
            &type_conflict("DateTime", "Timestamp")
        ));
        assert!(rule_matches(
            &type_rule("Int32|Int64", "Decimal", false),
            &type_conflict("Int64", "Decimal(10, 2)")
        ));
        assert!(rule_matches(
            &type_rule("Date", "DateTime|Timestamp", false),
            &type_conflict("Date", "DateTime")
        ));
    }
}
