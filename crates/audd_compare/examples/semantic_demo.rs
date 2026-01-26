//! Example demonstrating semantic matching capabilities

use audd_compare::{
    SemanticMatchConfig, SemanticMatchPipeline,
};

fn main() {
    println!("=== AUDD Semantic Matching Demo ===\n");

    // Create pipeline with default configuration
    let config = SemanticMatchConfig::default();
    let pipeline = SemanticMatchPipeline::new(config);

    // Test cases demonstrating different matching strategies
    let test_cases = vec![
        ("User", "users", "Pluralization"),
        ("UserProfile", "user_profile", "CamelCase normalization"),
        ("  USERS  ", "users", "Case & whitespace normalization"),
        ("firstName", "first_name", "Format normalization"),
        ("user_id", "userId", "Separator normalization"),
        ("sales_order", "order_sales", "Token reordering"),
        ("adress", "address", "Fuzzy matching (typo)"),
        ("user_table", "users_tbl", "Plural + stopwords"),
    ];

    println!("Basic Matching Tests:");
    println!("{:-<80}", "");
    for (a, b, description) in test_cases {
        let result = pipeline.compare(a, b);
        println!("  {} vs {}", a, b);
        println!("    Description: {}", description);
        println!("    Decision: {:?}", result.decision);
        println!("    Score: {:.3}", result.final_score);
        
        if let Some(reasons) = &result.reasons {
            println!("    Strategies:");
            for reason in reasons {
                println!("      - {}: {:.3}", reason.strategy, reason.score);
            }
        }
        println!();
    }

    // Synonym matching example
    println!("\n{:=<80}", "");
    println!("Synonym Matching Test:");
    println!("{:-<80}", "");
    
    let mut synonym_config = SemanticMatchConfig::default();
    synonym_config.synonyms.groups = vec![
        vec!["customer".to_string(), "client".to_string(), "user".to_string()],
        vec!["product".to_string(), "item".to_string(), "sku".to_string()],
        vec!["id".to_string(), "identifier".to_string(), "key".to_string()],
    ];
    
    let synonym_pipeline = SemanticMatchPipeline::new(synonym_config);
    
    let synonym_tests = vec![
        ("customer", "user"),
        ("product", "item"),
        ("user_id", "customer_key"),
    ];
    
    for (a, b) in synonym_tests {
        let result = synonym_pipeline.compare(a, b);
        println!("  {} vs {}", a, b);
        println!("    Decision: {:?}", result.decision);
        println!("    Score: {:.3}", result.final_score);
        if let Some(reasons) = &result.reasons {
            for reason in reasons {
                println!("      - {}: {:.3}", reason.strategy, reason.score);
                if let Some(detail) = &reason.detail {
                    println!("        Detail: {}", detail);
                }
            }
        }
        println!();
    }

    // Configuration demonstration
    println!("\n{:=<80}", "");
    println!("Configuration Options:");
    println!("{:-<80}", "");
    
    let default_config = SemanticMatchConfig::default();
    println!("  Semantic matching: {}", default_config.enabled);
    println!("  Locale: {:?}", default_config.locale);
    println!("  Explanations: {}", default_config.explain);
    println!("\n  Normalization:");
    println!("    Remove diacritics: {}", default_config.normalization.remove_diacritics);
    println!("    Unicode normalization: {:?}", default_config.normalization.unicode_normalization);
    println!("    Split camelCase: {}", default_config.normalization.split_camel_case);
    println!("    Stopwords: {:?}", default_config.normalization.stopwords);
    println!("\n  Pluralization:");
    println!("    Enabled: {}", default_config.pluralization.enabled);
    println!("\n  Stemming:");
    println!("    Enabled: {}", default_config.stemming.enabled);
    println!("\n  Token similarity:");
    println!("    Enabled: {}", default_config.token_similarity.enabled);
    println!("    Metric: {:?}", default_config.token_similarity.metric);
    println!("\n  Fuzzy matching:");
    println!("    Enabled: {}", default_config.fuzzy.enabled);
    println!("    Algorithm: {:?}", default_config.fuzzy.algorithm);
    println!("\n  N-grams:");
    println!("    Enabled: {}", default_config.ngrams.enabled);
    println!("    N: {}", default_config.ngrams.n);
    println!("\n  Weights:");
    println!("    Exact normalized: {}", default_config.weights.exact_normalized);
    println!("    Plural/stem: {}", default_config.weights.plural_stem);
    println!("    Synonyms: {}", default_config.weights.synonyms);
    println!("    Token similarity: {}", default_config.weights.token_similarity);
    println!("    Fuzzy chars: {}", default_config.weights.fuzzy_chars);
    println!("    N-grams: {}", default_config.weights.ngrams);
    println!("\n  Thresholds:");
    println!("    Auto match: {}", default_config.thresholds.auto_match);
    println!("    Probable match: {}", default_config.thresholds.probable_match);
    println!("    Allow probable as match: {}", default_config.allow_probable_as_match);

    println!("\n{:=<80}", "");
    println!("Demo complete!");
}
