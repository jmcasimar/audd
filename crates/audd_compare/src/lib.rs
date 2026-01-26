//! AUDD Comparison Engine
//!
//! This crate provides the comparison engine for comparing two schemas (A and B)
//! and generating matches, exclusives, and conflicts.
//!
//! # Overview
//!
//! The comparison engine analyzes two IR schemas and produces:
//! - **Matches**: Fields/entities that are compatible between A and B
//! - **Exclusives**: Fields/entities present in only one schema
//! - **Conflicts**: Incompatible fields/entities that require resolution
//!
//! # Example
//!
//! ```no_run
//! use audd_compare::{compare, CompareConfig};
//! use audd_ir::SourceSchema;
//!
//! # fn example(schema_a: SourceSchema, schema_b: SourceSchema) {
//! let config = CompareConfig::default();
//! let result = compare(&schema_a, &schema_b, &config);
//!
//! println!("Matches: {}", result.matches.len());
//! println!("Exclusives: {}", result.exclusives.len());
//! println!("Conflicts: {}", result.conflicts.len());
//! # }
//! ```

mod config;
mod conflict;
mod engine;
mod matcher;
mod result;
mod types;
mod unified;
mod semantic;

pub use config::CompareConfig;
pub use config::{
    SemanticMatchConfig, MatchLocale, UnicodeNormalization, SimilarityMetric, 
    FuzzyAlgorithm, NormalizationConfig, PluralizationConfig, StemmingConfig,
    SynonymConfig, TokenSimilarityConfig, FuzzyConfig, NgramConfig,
    ScoringWeights, MatchThresholds,
};
pub use conflict::{Conflict, ConflictEvidence, ConflictSeverity, ConflictType};
pub use engine::compare;
pub use result::{ComparisonResult, Exclusive, ExclusiveSide, Match, MatchReason};
pub use types::{compare_types, TypeCompatibility};
pub use unified::{FieldOrigin, FieldState, UnifiedEntity, UnifiedField, UnifiedSchema};
pub use semantic::{SemanticMatchDecision, SemanticMatchPipeline, SemanticMatchResult};
