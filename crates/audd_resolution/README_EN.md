# audd_resolution

Resolution suggestions and decision tracking for AUDD schema comparison conflicts.

## Overview

This crate provides the foundation for assisted resolution and auditable decision-making in AUDD:

- **Suggestions**: Explainable recommendations for resolving schema conflicts
- **Decisions**: Auditable tracking of resolution choices (human or automated)
- **Decision Logs**: Complete audit trail of conflict → suggestion → decision

## Features

- **Multiple Suggestion Types**:
  - `CastSafe`: Safe type casts (e.g., Int32 → Int64)
  - `CastRisky`: Risky casts with warnings (e.g., Int64 → Int32)
  - `RenameField`: Field renaming to avoid collisions
  - `PreferType`: Choose preferred type based on policy
  - `SplitField` / `MergeFields`: Structural changes (suggestions only)
  - `NoSuggestion`: Explicit marker when no auto-resolution available

- **Confidence & Impact Tracking**:
  - Confidence levels: High (0.9-1.0), Medium (0.6-0.89), Low (0.0-0.59)
  - Impact levels: Minimal, Low, Medium, High, Critical

- **Auditable Decisions**:
  - Source tracking (User vs System)
  - Timestamps
  - Rationale and metadata
  - Status tracking (Pending, Applied, Rejected, Superseded)

- **Configurable Policies**:
  - Type preference (prefer larger, higher precision, schema A/B)
  - Nullability policy (always nullable, always not null, prefer schema A/B)
  - Length policy (use max, use min, prefer schema A/B)
  - Risky suggestion control
  - Confidence thresholds

- **Export Formats**:
  - JSON: Structured data export
  - Markdown: Human-readable summaries

## Quick Start

```rust
use audd_compare::Conflict;
use audd_resolution::{
    DecisionLog, Decision, DecisionSource, 
    Suggestion, SuggestionEngine,
};

// Generate suggestions from a conflict
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

// Make a decision
let decision = Decision::accept(
    "dec1".to_string(),
    suggestions[0].clone(),
    "Approved for production".to_string(),
    DecisionSource::User {
        username: "admin".to_string(),
    },
);

// Track in decision log
let mut log = DecisionLog::new();
log.add_decision(decision);

// Export
let json = log.to_json().unwrap();
let markdown = log.to_markdown();
```

## Examples

Run the comprehensive example:

```bash
cargo run --package audd_resolution --example resolution_workflow
```

## Configuration

Create custom resolution policies:

```rust
use audd_resolution::{ResolutionConfig, SuggestionEngine};

// Conservative mode - only safe suggestions
let config = ResolutionConfig::conservative();
let engine = SuggestionEngine::with_config(config);

// Prefer schema A for all conflicts
let config = ResolutionConfig::prefer_schema_a();
let engine = SuggestionEngine::with_config(config);

// Custom configuration
let config = ResolutionConfig {
    type_preference: TypePreferencePolicy::PreferLarger,
    nullability_policy: NullabilityPolicy::AlwaysNullable,
    length_policy: LengthPolicy::UseMaximum,
    allow_risky_suggestions: false,
    min_confidence: 0.8,
    generate_alternatives: true,
};
let engine = SuggestionEngine::with_config(config);
```

## Testing

The crate includes comprehensive test coverage:

```bash
# Run all tests
cargo test --package audd_resolution

# Run integration tests
cargo test --package audd_resolution --test integration_test

# Run coverage tests
cargo test --package audd_resolution --test coverage_test
```

**Test Statistics:**
- Total tests: 62
- Unit tests: 38
- Coverage tests: 13
- Fixture tests: 5
- Integration tests: 5
- Doc tests: 1

**Coverage Metrics:**
- Comprehensive fixtures: ≥90% coverage achieved
- Type cast fixtures: 100% coverage
- Naming collision fixtures: 100% coverage
- All conflict types produce at least one suggestion

## Architecture

### Core Components

1. **Suggestion** (`suggestion.rs`)
   - Data structure for resolution recommendations
   - Includes confidence, impact, evidence, and explanation
   - Multiple suggestion kinds for different conflict types

2. **Decision** (`decision.rs`)
   - Tracks acceptance/rejection of suggestions
   - Records source (user or system), rationale, and timestamp
   - Status tracking for lifecycle management

3. **DecisionLog** (`decision_log.rs`)
   - Collection of decisions with metadata
   - Export to JSON and Markdown
   - Query interface for filtering decisions

4. **SuggestionEngine** (`engine.rs`)
   - Generates suggestions from conflicts
   - Configurable behavior via ResolutionConfig
   - Type-specific suggestion strategies

5. **ResolutionConfig** (`config.rs`)
   - Policies for type preference, nullability, length
   - Control over risky suggestions and confidence thresholds
   - Preset configurations (default, conservative, prefer A/B)

### Suggestion Generation Flow

```
Conflict → SuggestionEngine → [Suggestion, ...]
                ↓
           Configuration
                ↓
         Type Analysis → CastSafe/CastRisky
         Name Analysis → RenameField
    Nullability Check → PreferType (nullable)
       Constraint Check → NoSuggestion
```

### Decision Tracking Flow

```
Suggestions → User/System Review → Decision
                                      ↓
                              DecisionLog → Export
                                            - JSON
                                            - Markdown
```

## Status

**Implementation Status: COMPLETE** ✅

All EPIC 04 issues have been implemented:

- ✅ Issue 04.1: Core data models (Suggestion/Decision/DecisionLog)
- ✅ Issue 04.2: Type incompatibility suggestions (casts)
- ✅ Issue 04.3: Naming collision suggestions (rename)
- ✅ Issue 04.4: Configurable type preferences
- ✅ Issue 04.5: Integration with comparison flow
- ✅ Issue 04.6: Fixtures and coverage tests

## Future Enhancements

Potential future improvements (out of scope for MVP):

- UI for interactive decision making
- Machine learning for suggestion ranking
- Custom suggestion strategies via plugins
- Integration with version control systems
- Automatic application of approved decisions
- Rollback capabilities for applied decisions

## License

Licensed under the MIT License. See [LICENSE](../../LICENSE) for details.
