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

- **Export Formats**:
  - JSON: Structured data export
  - Markdown: Human-readable summaries

## Usage

```rust
use audd_resolution::{
    DecisionLog, Decision, DecisionSource, 
    Suggestion, Confidence, Impact
};

// Create a suggestion
let suggestion = Suggestion::cast_safe(
    "sug1".to_string(),
    "users".to_string(),
    "id".to_string(),
    "Int32".to_string(),
    "Int64".to_string(),
    "Safe widening cast from Int32 to Int64".to_string(),
);

// Create a decision
let decision = Decision::accept(
    "dec1".to_string(),
    suggestion,
    "Approved for production".to_string(),
    DecisionSource::User {
        username: "admin".to_string(),
    },
);

// Track in decision log
let mut log = DecisionLog::new();
log.add_decision(decision);

// Export to JSON
let json = log.to_json().unwrap();
std::fs::write("decision_log.json", json).unwrap();

// Or export to Markdown
let markdown = log.to_markdown();
std::fs::write("decision_log.md", markdown).unwrap();
```

## Status

This module implements **EPIC 04 - Issue 04.1**: Core data model definitions.

Future enhancements:
- Suggestion engine (Issue 04.2-04.4)
- Integration with comparison engine (Issue 04.5)
- Comprehensive fixtures and tests (Issue 04.6)
