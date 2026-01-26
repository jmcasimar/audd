# Semantic Matching Feature

## Overview

The Semantic Matching feature provides intelligent, configurable comparison of entity and field names in AUDD, going beyond simple string equality to recognize equivalent names despite differences in formatting, pluralization, typos, and terminology.

## Key Features

### 1. **Advanced Normalization**
- Unicode normalization (NFC/NFKC)
- Diacritics removal (e.g., `canción` → `cancion`)
- CamelCase/PascalCase splitting (`UserProfile` → `user profile`)
- Separator normalization (`user-name`, `user_name`, `userName` all become equivalent)
- Stopword removal (e.g., `tbl`, `table`, `tmp`)

### 2. **Pluralization & Stemming**
- English plural/singular normalization (`users` ↔ `user`)
- Irregular plural handling (`children` ↔ `child`, `people` ↔ `person`)
- Simple stemming for entity names
- Spanish language support (basic rules included)
- Configurable exceptions list

### 3. **Synonym Dictionary**
- Controlled equivalences (`customer` ↔ `user` ↔ `client`)
- Bidirectional matching
- Multiple synonym groups
- Domain-specific terminology support

### 4. **Token Similarity**
- Jaccard or Dice coefficient
- Handles token reordering (`sales_order` ↔ `order_sales`)
- Detailed overlap information for debugging

### 5. **Fuzzy Character Matching**
- Jaro-Winkler (recommended for short names)
- Levenshtein distance
- Damerau-Levenshtein distance
- Typo-tolerant matching (`adress` ↔ `address`)

### 6. **N-gram Similarity**
- Configurable n-gram size (2-5, default 3)
- Jaccard or Dice metric
- Handles prefix/suffix variations

### 7. **Configurable Scoring Pipeline**
- Weighted combination of strategies
- Adjustable thresholds for match decisions
- Three-level classification: `match`, `probable_match`, `no_match`
- Detailed explanations for debugging

## Usage

### Basic Usage

```rust
use audd_compare::{SemanticMatchConfig, SemanticMatchPipeline};

// Create pipeline with default configuration
let config = SemanticMatchConfig::default();
let pipeline = SemanticMatchPipeline::new(config);

// Compare two names
let result = pipeline.compare("User", "users");

println!("Decision: {:?}", result.decision);  // Match
println!("Score: {:.2}", result.final_score); // 0.96
```

### With Synonyms

```rust
let mut config = SemanticMatchConfig::default();
config.synonyms.groups = vec![
    vec!["customer".to_string(), "client".to_string(), "user".to_string()],
    vec!["product".to_string(), "item".to_string(), "sku".to_string()],
];

let pipeline = SemanticMatchPipeline::new(config);
let result = pipeline.compare("customer", "user");
// Result: Match (via synonyms)
```

### Custom Configuration

```rust
let mut config = SemanticMatchConfig::default();

// Adjust weights
config.weights.synonyms = 0.80;
config.weights.fuzzy_chars = 0.10;

// Adjust thresholds
config.thresholds.auto_match = 0.90;
config.thresholds.probable_match = 0.70;

// Configure normalization
config.normalization.remove_diacritics = true;
config.normalization.stopwords.push("temp".to_string());

// Select fuzzy algorithm
config.fuzzy.algorithm = FuzzyAlgorithm::DamerauLevenshtein;

let pipeline = SemanticMatchPipeline::new(config);
```

### Disabling Semantic Matching

```rust
let mut config = CompareConfig::default();
config.semantic_match.enabled = false;

// Will fall back to exact/normalized/similarity matching
```

## Configuration Reference

### Default Weights

| Strategy | Weight | Description |
|----------|--------|-------------|
| `exact_normalized` | 0.60 | Exact match after normalization |
| `plural_stem` | 0.55 | Match after plural/stem reduction |
| `synonyms` | 0.70 | Synonym dictionary match |
| `token_similarity` | 0.25 | Token overlap (Jaccard/Dice) |
| `fuzzy_chars` | 0.15 | Character-level fuzzy matching |
| `ngrams` | 0.10 | N-gram similarity |

### Default Thresholds

- `auto_match`: 0.80 - Scores ≥ this are automatic matches
- `probable_match`: 0.55 - Scores ≥ this are probable matches
- Scores < `probable_match` are no matches

### Supported Locales

- `en` - English (default)
- `es` - Spanish
- `mixed` - Universal rules

## Examples

### Normalization

```rust
// All of these match:
"UserProfile" == "user_profile"
"firstName" == "first_name"
"  USERS  " == "users"
"user-name" == "user_name"
```

### Pluralization

```rust
// All of these match:
"users" == "user"
"categories" == "category"
"children" == "child"
```

### Token Reordering

```rust
// These match due to token overlap:
"sales_order" ~= "order_sales"  // ProbableMatch
"user_profile" == "profile_user"  // ProbableMatch
```

### Fuzzy Matching

```rust
// Typos are recognized:
"adress" ~= "address"
"recieve" ~= "receive"
```

## Integration with AUDD

Semantic matching is automatically used in the comparison engine when enabled:

```rust
use audd_compare::{compare, CompareConfig};

let mut config = CompareConfig::default();
// Semantic matching is enabled by default

let result = compare(&schema_a, &schema_b, &config);
// Matches will use semantic comparison
```

Matches are reported with a `Semantic` reason:

```rust
match &match_result.reason {
    MatchReason::Semantic { score, decision, details } => {
        println!("Semantic match: {}, score: {}", decision, score);
    }
    _ => {}
}
```

## Best Practices

1. **Start with defaults**: The default configuration works well for most use cases
2. **Use synonyms for domain terms**: Define synonym groups for your specific domain
3. **Adjust weights carefully**: Small changes can have large effects
4. **Enable explanations during development**: Set `explain: true` to understand matching decisions
5. **Monitor probable matches**: Review `probable_match` decisions to tune thresholds
6. **Disable stemming for strict matching**: Stemming is disabled by default; enable only if needed

## Performance Considerations

- Semantic matching adds computational overhead compared to exact matching
- Most expensive: fuzzy character matching and n-grams
- Least expensive: exact normalized and plural/stem matching
- Disable strategies you don't need to improve performance
- The pipeline short-circuits on exact matches

## Troubleshooting

### Names not matching when they should

1. Enable `explain: true` to see which strategies fired
2. Check if normalization is removing needed information
3. Verify synonym groups contain the expected terms
4. Lower the `auto_match` threshold
5. Check if stemming is being too aggressive

### Names matching when they shouldn't

1. Increase the `auto_match` threshold
2. Reduce weights for fuzzy/ngram strategies
3. Disable strategies that are too permissive
4. Add exceptions to pluralization rules

### Unexpected scores

1. Remember that weights are applied only to active strategies
2. Check if all expected strategies are enabled
3. Verify that normalized text is what you expect
4. Use the `reasons` field to see individual strategy scores

## License

This feature is part of AUDD and is licensed under the same terms.
