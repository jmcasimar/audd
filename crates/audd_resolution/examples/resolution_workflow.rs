//! Example demonstrating the resolution workflow
//!
//! This example shows how to:
//! 1. Generate suggestions from conflicts
//! 2. Make decisions on suggestions
//! 3. Track decisions in a decision log
//! 4. Export the decision log

use audd_compare::Conflict;
use audd_resolution::{
    Decision, DecisionLog, DecisionSource, ResolutionConfig, SuggestionEngine,
};

fn main() {
    println!("=== AUDD Resolution Example ===\n");

    // Step 1: Create sample conflicts (normally these come from schema comparison)
    let conflicts = create_sample_conflicts();
    println!("Created {} sample conflicts\n", conflicts.len());

    // Step 2: Create a suggestion engine with default configuration
    let engine = SuggestionEngine::new();
    println!("Suggestion engine created with default configuration\n");

    // Step 3: Generate suggestions for each conflict
    println!("Generating suggestions...\n");
    let all_suggestions: Vec<_> = conflicts
        .iter()
        .enumerate()
        .flat_map(|(idx, conflict)| {
            let mut suggestions = engine.suggest(conflict);
            // Tag suggestions with conflict index
            for suggestion in &mut suggestions {
                *suggestion = suggestion.clone().with_conflict_index(idx);
            }
            suggestions
        })
        .collect();

    println!("Generated {} suggestions for {} conflicts\n", 
             all_suggestions.len(), conflicts.len());

    // Step 4: Display suggestions
    for (idx, suggestion) in all_suggestions.iter().enumerate() {
        println!("Suggestion {}:", idx + 1);
        println!("  Entity: {}", suggestion.entity_name);
        if let Some(ref field) = suggestion.field_name {
            println!("  Field: {}", field);
        }
        println!("  Kind: {:?}", suggestion.kind);
        println!("  Confidence: {:.2}", suggestion.confidence.value());
        println!("  Impact: {:?}", suggestion.impact);
        println!("  Explanation: {}", suggestion.explanation);
        println!();
    }

    // Step 5: Create a decision log
    let mut log = DecisionLog::new()
        .with_schema_ids("schema_a".to_string(), "schema_b".to_string());

    // Step 6: Make decisions on suggestions
    println!("Making decisions...\n");

    // Accept first suggestion (if it's a safe cast)
    if let Some(suggestion) = all_suggestions.first() {
        let decision = Decision::accept(
            format!("dec_{}", 1),
            suggestion.clone(),
            "Automatically approved safe cast".to_string(),
            DecisionSource::System {
                rule: "auto_approve_safe_casts".to_string(),
            },
        )
        .mark_applied();

        log.add_decision(decision);
        println!("✓ Accepted suggestion 1 (automated)");
    }

    // Manually review and accept another suggestion
    if all_suggestions.len() > 1 {
        let decision = Decision::by_user(
            format!("dec_{}", 2),
            all_suggestions[1].clone(),
            true,
            "Reviewed and approved - rename is appropriate".to_string(),
            "admin".to_string(),
        );

        log.add_decision(decision);
        println!("✓ Accepted suggestion 2 (manual review)");
    }

    println!();

    // Step 7: Display decision log summary
    println!("=== Decision Log Summary ===");
    println!("Total decisions: {}", log.metadata.total_decisions);
    println!("Accepted: {}", log.metadata.accepted_decisions);
    println!("Rejected: {}", log.metadata.rejected_decisions);
    println!();

    // Step 8: Export to JSON
    let json = log.to_json().expect("Failed to export to JSON");
    println!("=== JSON Export (first 500 chars) ===");
    println!("{}", &json[..json.len().min(500)]);
    println!("...\n");

    // Step 9: Export to Markdown
    let markdown = log.to_markdown();
    println!("=== Markdown Export ===");
    println!("{}", markdown);

    // Step 10: Demonstrate configuration
    println!("=== Custom Configuration Example ===");
    let conservative_config = ResolutionConfig::conservative();
    let conservative_engine = SuggestionEngine::with_config(conservative_config);
    
    println!("Conservative engine created (safe suggestions only)");
    println!("Risky suggestions allowed: {}", 
             conservative_engine.config().allow_risky_suggestions);
    println!("Min confidence: {}", 
             conservative_engine.config().min_confidence);
}

fn create_sample_conflicts() -> Vec<Conflict> {
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
        // Normalization collision
        Conflict::normalization_collision(
            "customers".to_string(),
            "UserID".to_string(),
            "user_id".to_string(),
            "user_id".to_string(),
            0,
            1,
        ),
        // Nullability mismatch
        Conflict::nullability_mismatch(
            "products".to_string(),
            "description".to_string(),
            false,
            true,
            0,
            1,
        ),
    ]
}
