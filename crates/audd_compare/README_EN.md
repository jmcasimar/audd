# AUDD Compare

Comparison engine for the AUDD (Automatic Unification of Data Definitions) project.

## Overview

`audd_compare` provides a sophisticated comparison engine for analyzing two schema representations and identifying:
- **Matches**: Fields/entities that are compatible between schemas
- **Exclusives**: Fields/entities present in only one schema
- **Conflicts**: Incompatibilities that require resolution

## Features

### Core Capabilities

- **Multiple Matching Strategies**
  - Exact name matching
  - Normalized matching (case-insensitive, snake_case/camelCase)
  - Similarity-based matching (Jaro-Winkler algorithm)

- **Type Compatibility Analysis**
  - Identical types
  - Compatible types (e.g., String ↔ Text)
  - Safe widening (e.g., Int32 → Int64)
  - Cast required (e.g., Int64 → Int32)
  - Incompatible types

- **Conflict Detection**
  - Type incompatibilities
  - Nullability mismatches
  - Constraint conflicts (unique, length, precision)
  - Normalization collisions

- **Unified Schema Generation**
  - Combines both schemas into a single representation
  - Tracks field origins (A, B, or Both)
  - Marks field states (Matched, Exclusive, Conflicted)

### Configuration

The comparison engine is highly configurable:

```rust
use audd_compare::CompareConfig;

// Default configuration (exact + normalized matching, all checks)
let config = CompareConfig::default();

// All features enabled (includes similarity matching)
let config = CompareConfig::all_features()
    .with_similarity_threshold(0.8);

// Minimal configuration (exact matching only)
let config = CompareConfig::minimal();

// Strict configuration (all checks, high threshold)
let config = CompareConfig::strict();
```

## Usage

### Basic Example

```rust
use audd_compare::{compare, CompareConfig};
use audd_ir::{SourceSchema, EntitySchema, FieldSchema, CanonicalType};

// Create schema A
let schema_a = SourceSchema::builder()
    .source_name("db_a")
    .source_type("mysql")
    .add_entity(
        EntitySchema::builder()
            .entity_name("users")
            .add_field(
                FieldSchema::builder()
                    .field_name("id")
                    .canonical_type(CanonicalType::Int32)
                    .nullable(false)
                    .build()
            )
            .build()
    )
    .build();

// Create schema B
let schema_b = SourceSchema::builder()
    .source_name("db_b")
    .source_type("postgresql")
    .add_entity(
        EntitySchema::builder()
            .entity_name("users")
            .add_field(
                FieldSchema::builder()
                    .field_name("id")
                    .canonical_type(CanonicalType::Int64)
                    .nullable(false)
                    .build()
            )
            .build()
    )
    .build();

// Compare
let config = CompareConfig::default();
let result = compare(&schema_a, &schema_b, &config);

// Analyze results
println!("Matches: {}", result.summary.total_matches);
println!("Exclusives: {}", result.summary.total_exclusives);
println!("Conflicts: {}", result.summary.total_conflicts);
```

### Generating Unified Schema

```rust
use audd_compare::UnifiedSchema;

let unified = UnifiedSchema::from_comparison(&schema_a, &schema_b, &result);

// Export to JSON
let json = unified.to_json()?;
```

## Examples

Run the demo example:

```bash
cargo run --example compare_demo
```

## Testing

The crate includes comprehensive test coverage:

```bash
# Run all tests
cargo test -p audd_compare

# Run unit tests only
cargo test -p audd_compare --lib

# Run integration tests only
cargo test -p audd_compare --test integration_test
```

## Architecture

### Modules

- **config**: Configuration options for the comparison engine
- **conflict**: Conflict types and evidence tracking
- **engine**: Main comparison logic
- **matcher**: Entity and field matching algorithms
- **result**: Match, Exclusive, and ComparisonResult structures
- **types**: Type compatibility analysis
- **unified**: Unified schema construction

### Data Flow

```
Schema A + Schema B + Config
         ↓
    Matching Engine
         ↓
    ┌────┴────┬──────────┬──────────┐
    ↓         ↓          ↓          ↓
  Matches  Exclusives Conflicts  Summary
         ↓
   ComparisonResult
         ↓
   UnifiedSchema
```

## Dependencies

- `audd_ir`: Intermediate representation for schemas
- `serde`: Serialization/deserialization
- `strsim`: String similarity algorithms

## License

MIT
