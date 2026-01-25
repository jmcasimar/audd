//! Coverage tests for suggestion generation

mod fixtures;

use audd_resolution::SuggestionEngine;
use fixtures::*;

#[test]
fn test_comprehensive_fixtures_coverage() {
    let engine = SuggestionEngine::new();
    let fixtures = comprehensive_conflict_fixtures();
    
    let mut total_conflicts = 0;
    let mut conflicts_with_suggestions = 0;
    
    for conflict in &fixtures {
        total_conflicts += 1;
        let suggestions = engine.suggest(conflict);
        
        if !suggestions.is_empty() {
            conflicts_with_suggestions += 1;
        }
        
        // Every conflict should produce at least one suggestion (even if NoSuggestion)
        assert!(
            !suggestions.is_empty(),
            "Conflict {:?} for entity '{}' should produce at least one suggestion",
            conflict.conflict_type,
            conflict.entity_name
        );
    }
    
    let coverage = (conflicts_with_suggestions as f64 / total_conflicts as f64) * 100.0;
    
    // MVP goal: ≥90% coverage
    assert!(
        coverage >= 90.0,
        "Suggestion coverage is {:.1}%, expected ≥90%",
        coverage
    );
    
    println!("✓ Comprehensive fixtures coverage: {:.1}% ({}/{})", 
             coverage, conflicts_with_suggestions, total_conflicts);
}

#[test]
fn test_type_cast_fixtures_100_percent_coverage() {
    let engine = SuggestionEngine::new();
    let fixtures = type_cast_fixtures();
    
    for conflict in &fixtures {
        let suggestions = engine.suggest(conflict);
        
        assert!(
            !suggestions.is_empty(),
            "Type conflict for field '{}' should have suggestions",
            conflict.field_name.as_ref().unwrap_or(&"unknown".to_string())
        );
        
        // Type conflicts should always generate actionable suggestions
        let has_actionable = suggestions.iter().any(|s| {
            !matches!(s.kind, audd_resolution::SuggestionKind::NoSuggestion { .. })
        });
        
        assert!(
            has_actionable,
            "Type conflict should have at least one actionable suggestion"
        );
    }
    
    println!("✓ Type cast fixtures: 100% coverage ({} conflicts)", fixtures.len());
}

#[test]
fn test_naming_collision_fixtures_100_percent_coverage() {
    let engine = SuggestionEngine::new();
    let fixtures = naming_collision_fixtures();
    
    for conflict in &fixtures {
        let suggestions = engine.suggest(conflict);
        
        assert!(
            !suggestions.is_empty(),
            "Naming collision for entity '{}' should have suggestions",
            conflict.entity_name
        );
        
        // Collisions should generate rename suggestions
        let has_rename = suggestions.iter().any(|s| {
            matches!(s.kind, audd_resolution::SuggestionKind::RenameField { .. })
        });
        
        assert!(
            has_rename,
            "Naming collision should suggest rename strategies"
        );
    }
    
    println!("✓ Naming collision fixtures: 100% coverage ({} conflicts)", fixtures.len());
}

#[test]
fn test_nullability_fixtures_coverage() {
    let engine = SuggestionEngine::new();
    let fixtures = nullability_fixtures();
    
    for conflict in &fixtures {
        let suggestions = engine.suggest(conflict);
        
        assert!(
            !suggestions.is_empty(),
            "Nullability conflict for field '{}' should have suggestions",
            conflict.field_name.as_ref().unwrap_or(&"unknown".to_string())
        );
    }
    
    println!("✓ Nullability fixtures: 100% coverage ({} conflicts)", fixtures.len());
}

#[test]
fn test_constraint_fixtures_coverage() {
    let engine = SuggestionEngine::new();
    let fixtures = constraint_fixtures();
    
    for conflict in &fixtures {
        let suggestions = engine.suggest(conflict);
        
        // Even if we can't auto-resolve, we should provide a NoSuggestion with reason
        assert!(
            !suggestions.is_empty(),
            "Constraint conflict for field '{}' should have at least NoSuggestion",
            conflict.field_name.as_ref().unwrap_or(&"unknown".to_string())
        );
    }
    
    println!("✓ Constraint fixtures: 100% coverage ({} conflicts)", fixtures.len());
}

#[test]
fn test_all_fixtures_combined_coverage() {
    let engine = SuggestionEngine::new();
    
    let all_fixtures = [
        comprehensive_conflict_fixtures(),
        type_cast_fixtures(),
        naming_collision_fixtures(),
        nullability_fixtures(),
        constraint_fixtures(),
    ]
    .concat();
    
    let mut total = 0;
    let mut with_suggestions = 0;
    let mut with_actionable = 0;
    
    for conflict in &all_fixtures {
        total += 1;
        let suggestions = engine.suggest(conflict);
        
        if !suggestions.is_empty() {
            with_suggestions += 1;
        }
        
        let has_actionable = suggestions.iter().any(|s| {
            !matches!(s.kind, audd_resolution::SuggestionKind::NoSuggestion { .. })
        });
        
        if has_actionable {
            with_actionable += 1;
        }
    }
    
    let coverage = (with_suggestions as f64 / total as f64) * 100.0;
    let actionable_rate = (with_actionable as f64 / total as f64) * 100.0;
    
    assert!(
        coverage >= 90.0,
        "Overall coverage is {:.1}%, expected ≥90%",
        coverage
    );
    
    println!("✓ Combined fixtures coverage: {:.1}% ({}/{})", coverage, with_suggestions, total);
    println!("  - Actionable suggestions: {:.1}% ({}/{})", actionable_rate, with_actionable, total);
}

#[test]
fn test_suggestion_quality_metrics() {
    let engine = SuggestionEngine::new();
    let fixtures = comprehensive_conflict_fixtures();
    
    let mut total_suggestions = 0;
    let mut high_confidence = 0;
    let mut low_impact = 0;
    
    for conflict in &fixtures {
        let suggestions = engine.suggest(conflict);
        
        for suggestion in suggestions {
            total_suggestions += 1;
            
            if suggestion.confidence.value() >= 0.7 {
                high_confidence += 1;
            }
            
            if matches!(
                suggestion.impact,
                audd_resolution::Impact::Minimal | audd_resolution::Impact::Low
            ) {
                low_impact += 1;
            }
        }
    }
    
    let high_confidence_rate = (high_confidence as f64 / total_suggestions as f64) * 100.0;
    let low_impact_rate = (low_impact as f64 / total_suggestions as f64) * 100.0;
    
    println!("✓ Suggestion quality metrics:");
    println!("  - Total suggestions: {}", total_suggestions);
    println!("  - High confidence (≥0.7): {:.1}%", high_confidence_rate);
    println!("  - Low impact: {:.1}%", low_impact_rate);
    
    // Quality assertions
    assert!(total_suggestions > 0, "Should generate suggestions");
}

#[test]
fn test_evidence_completeness() {
    let engine = SuggestionEngine::new();
    let fixtures = comprehensive_conflict_fixtures();
    
    for conflict in &fixtures {
        let suggestions = engine.suggest(conflict);
        
        for suggestion in suggestions {
            // Every suggestion should have evidence
            assert!(
                !suggestion.evidence.is_empty(),
                "Suggestion {} should have evidence",
                suggestion.id
            );
            
            // Every suggestion should have an explanation
            assert!(
                !suggestion.explanation.is_empty(),
                "Suggestion {} should have explanation",
                suggestion.id
            );
        }
    }
    
    println!("✓ All suggestions have complete evidence and explanations");
}
