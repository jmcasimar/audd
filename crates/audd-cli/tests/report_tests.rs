//! Integration tests for report generation

use std::fs;
use std::path::PathBuf;

fn get_fixtures_dir() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // Go up from audd-cli
    path.pop(); // Go up from crates
    path.push("fixtures");
    path
}

fn get_golden_dir() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("golden");
    path
}

// Helper to normalize timestamps and other variable content in reports
fn normalize_report(content: &str) -> String {
    let mut normalized = String::new();
    for line in content.lines() {
        // Skip timestamp lines as they vary
        if line.starts_with("**Generated:**") {
            normalized.push_str("**Generated:** [TIMESTAMP]\n");
        } else if line.contains("- **Timestamp**:") {
            // Normalize decision log timestamps - keep prefix, replace timestamp
            let parts: Vec<&str> = line.split("- **Timestamp**:").collect();
            if parts.len() == 2 {
                normalized.push_str(parts[0]);
                normalized.push_str("- **Timestamp**: [TIMESTAMP]\n");
            } else {
                normalized.push_str(line);
                normalized.push('\n');
            }
        } else {
            normalized.push_str(line);
            normalized.push('\n');
        }
    }
    normalized
}

#[test]
fn test_report_generation_users_csv_vs_json() {
    use audd_adapters_file::load_schema_from_file;
    use audd_compare::{compare, CompareConfig};
    use audd_resolution::{DecisionLog, SuggestionEngine};
    use audd_cli::report::generate_report;

    // Load schemas
    let fixtures = get_fixtures_dir();
    let csv_path = fixtures.join("adapters").join("users.csv");
    let json_path = fixtures.join("adapters").join("users.json");

    let schema_a = load_schema_from_file(&csv_path).expect("Failed to load CSV");
    let schema_b = load_schema_from_file(&json_path).expect("Failed to load JSON");

    // Compare
    let config = CompareConfig::default();
    let result = compare(&schema_a, &schema_b, &config);

    // Generate suggestions
    let engine = SuggestionEngine::new();
    let mut decision_log = DecisionLog::new()
        .with_schema_ids(schema_a.source_name.clone(), schema_b.source_name.clone());

    for (i, conflict) in result.conflicts.iter().enumerate() {
        let suggestions = engine.suggest(conflict);
        for suggestion in suggestions {
            if suggestion.confidence.value() >= 0.85 {
                let decision = audd_resolution::Decision::by_system(
                    format!("auto_dec_{}", i + 1),
                    suggestion,
                    true,
                    "high_confidence_auto_accept".to_string(),
                );
                decision_log.add_decision(decision);
            }
        }
    }

    // Generate report
    let report = generate_report(
        &schema_a.source_name,
        &schema_b.source_name,
        &result,
        Some(&decision_log),
    );

    // Normalize report for comparison
    let normalized = normalize_report(&report);

    // Check key sections exist
    assert!(normalized.contains("# AUDD Comparison Report"));
    assert!(normalized.contains("## Executive Summary"));
    assert!(normalized.contains("### Compatibility Overview"));
    assert!(normalized.contains("### Risk Assessment"));
    assert!(normalized.contains("## Detailed Breakdown"));
    assert!(normalized.contains("## Technical Details"));
    assert!(normalized.contains("### Matches"));
    assert!(normalized.contains("### Exclusives"));
    assert!(normalized.contains("### Conflicts"));
    assert!(normalized.contains("## Resolution Suggestions"));
    assert!(normalized.contains("## Decision Log"));
    assert!(normalized.contains("## Recommendations"));

    // Check specific metrics
    assert!(normalized.contains("**Matches**: 6"));
    assert!(normalized.contains("**Conflicts**: 3"));
    assert!(normalized.contains("**Compatibility Score**: 66.7%"));

    // Optional: Update golden file if needed
    // To update golden files, set environment variable: UPDATE_GOLDEN=1
    let golden_path = get_golden_dir().join("users_csv_vs_json.md");
    
    if std::env::var("UPDATE_GOLDEN").is_ok() {
        fs::create_dir_all(get_golden_dir()).ok();
        fs::write(&golden_path, &normalized).expect("Failed to write golden file");
        println!("Updated golden file: {:?}", golden_path);
    }

    // Compare with golden file if it exists
    if golden_path.exists() {
        let golden_content = fs::read_to_string(&golden_path)
            .expect("Failed to read golden file");
        
        // Compare line by line for better diff output
        let golden_lines: Vec<&str> = golden_content.lines().collect();
        let report_lines: Vec<&str> = normalized.lines().collect();
        
        if golden_lines != report_lines {
            // Show differences
            println!("\n=== GOLDEN FILE MISMATCH ===");
            println!("Expected {} lines, got {} lines", golden_lines.len(), report_lines.len());
            
            for (i, (expected, actual)) in golden_lines.iter().zip(report_lines.iter()).enumerate() {
                if expected != actual {
                    println!("\nLine {} differs:", i + 1);
                    println!("  Expected: {}", expected);
                    println!("  Actual:   {}", actual);
                }
            }
            
            panic!("Report content does not match golden file. Run with UPDATE_GOLDEN=1 to update.");
        }
    }
}

#[test]
fn test_report_sections_structure() {
    use audd_compare::{ComparisonResult, Match, Exclusive, Conflict};
    use audd_cli::report::generate_report;

    // Create minimal test data
    let matches = vec![
        Match::exact("test_entity".to_string(), Some("field1".to_string()), 0, 0),
    ];
    let exclusives = vec![
        Exclusive::from_a("test_entity".to_string(), Some("field2".to_string()), 1),
    ];
    let conflicts = vec![
        Conflict::type_incompatible(
            "test_entity".to_string(),
            "field3".to_string(),
            "String".to_string(),
            "Int32".to_string(),
            2,
            2,
        ),
    ];

    let result = ComparisonResult::new(matches, exclusives, conflicts);
    
    // Generate report without decision log
    let report = generate_report("schema_a", "schema_b", &result, None);

    // Verify structure
    assert!(report.contains("# AUDD Comparison Report"));
    assert!(report.contains("**Schema A:** schema_a"));
    assert!(report.contains("**Schema B:** schema_b"));
    assert!(report.contains("## Executive Summary"));
    assert!(report.contains("## Detailed Breakdown"));
    assert!(report.contains("## Technical Details"));
    assert!(report.contains("## Recommendations"));
    
    // Should not contain suggestion sections when no decision log provided
    assert!(!report.contains("## Resolution Suggestions"));
    assert!(!report.contains("## Decision Log"));
}

#[test]
fn test_risk_level_indicators() {
    use audd_compare::{ComparisonResult, Conflict, ConflictSeverity, ConflictEvidence, ConflictType};
    use audd_cli::report::{generate_report, ReportMetrics};

    // Test critical risk level
    let critical_conflicts = vec![
        Conflict::new(
            "test".to_string(),
            Some("field1".to_string()),
            ConflictType::TypeIncompatible,
            ConflictSeverity::Critical,
            ConflictEvidence::new("a".to_string(), "b".to_string(), "rule".to_string()),
            0,
            0,
        ),
        Conflict::new(
            "test".to_string(),
            Some("field2".to_string()),
            ConflictType::TypeIncompatible,
            ConflictSeverity::Critical,
            ConflictEvidence::new("a".to_string(), "b".to_string(), "rule".to_string()),
            1,
            1,
        ),
        Conflict::new(
            "test".to_string(),
            Some("field3".to_string()),
            ConflictType::TypeIncompatible,
            ConflictSeverity::Critical,
            ConflictEvidence::new("a".to_string(), "b".to_string(), "rule".to_string()),
            2,
            2,
        ),
    ];
    
    let result = ComparisonResult::new(vec![], vec![], critical_conflicts);
    let report = generate_report("a", "b", &result, None);
    
    // Should show critical risk level with appropriate emoji
    assert!(report.contains("💀 **Critical**") || report.contains("🔥 **High**"));
    assert!(report.contains("critical-severity conflict"));
}

#[test]
fn test_metrics_calculation_edge_cases() {
    use audd_compare::ComparisonResult;
    use audd_cli::report::ReportMetrics;

    // Test with empty result
    let empty_result = ComparisonResult::new(vec![], vec![], vec![]);
    let metrics = ReportMetrics::from_comparison(&empty_result);
    
    assert_eq!(metrics.total_matches, 0);
    assert_eq!(metrics.total_conflicts, 0);
    assert_eq!(metrics.compatibility_score, 100.0); // No conflicts means 100% compatible
    assert_eq!(metrics.conflict_rate, 0.0);
}

#[test]
fn test_json_report_generation() {
    use audd_compare::{ComparisonResult, Match, Exclusive, Conflict};
    use audd_cli::report::generate_json_report;

    // Create minimal test data
    let matches = vec![
        Match::exact("test_entity".to_string(), Some("field1".to_string()), 0, 0),
    ];
    let exclusives = vec![
        Exclusive::from_a("test_entity".to_string(), Some("field2".to_string()), 1),
    ];
    let conflicts = vec![
        Conflict::type_incompatible(
            "test_entity".to_string(),
            "field3".to_string(),
            "String".to_string(),
            "Int32".to_string(),
            2,
            2,
        ),
    ];

    let result = ComparisonResult::new(matches, exclusives, conflicts);
    
    // Generate JSON report
    let json_report = generate_json_report("schema_a", "schema_b", &result, None);

    // Verify structure
    assert_eq!(json_report.metadata.schema_a, "schema_a");
    assert_eq!(json_report.metadata.schema_b, "schema_b");
    assert_eq!(json_report.metadata.report_version, "1.0.0");
    
    assert_eq!(json_report.executive_summary.compatibility_overview.total_matches, 1);
    assert_eq!(json_report.executive_summary.compatibility_overview.total_conflicts, 1);
    assert_eq!(json_report.executive_summary.compatibility_overview.total_exclusives, 1);
    
    assert!(json_report.technical_details.matches.len() == 1);
    assert!(json_report.technical_details.exclusives.len() == 1);
    assert!(json_report.technical_details.conflicts.len() == 1);
    
    // Should not have resolution section when no decision log
    assert!(json_report.resolution.is_none());
    
    // Verify it serializes to valid JSON
    let json_str = serde_json::to_string(&json_report).expect("Failed to serialize");
    assert!(!json_str.is_empty());
    
    // Verify deserialization works
    let _: audd_cli::JsonReport = serde_json::from_str(&json_str).expect("Failed to deserialize");
}
