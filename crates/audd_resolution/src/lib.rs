//! AUDD Resolution Module
//!
//! This crate provides resolution suggestions and decision tracking for conflicts
//! detected during schema comparison.
//!
//! # Overview
//!
//! The resolution module enables:
//! - **Suggestions**: Explainable recommendations for resolving conflicts
//! - **Decisions**: Auditable tracking of resolution choices
//! - **Decision Logs**: Complete trace of conflict → suggestion → decision
//!
//! # Architecture
//!
//! - `Suggestion`: Individual recommendations for conflict resolution
//! - `Decision`: Record of accepting/rejecting a suggestion
//! - `DecisionLog`: Collection of decisions with metadata
//! - `Engine`: Logic for generating suggestions from conflicts
//!
//! # Example
//!
//! ```no_run
//! use audd_resolution::{DecisionLog, Decision, DecisionSource};
//!
//! let mut log = DecisionLog::new();
//!
//! // Add decisions as they are made
//! // let decision = Decision::accept(...);
//! // log.add_decision(decision);
//!
//! // Export to JSON
//! // let json = log.to_json().unwrap();
//! ```
//!
//! # Features
//!
//! - JSON serialization for all structures
//! - Markdown export for decision logs
//! - Confidence levels and impact tracking
//! - Auditable decision history with timestamps

mod decision;
mod decision_log;
mod suggestion;

pub use decision::{Decision, DecisionSource, DecisionStatus};
pub use decision_log::DecisionLog;
pub use suggestion::{Confidence, Impact, Suggestion, SuggestionKind};
