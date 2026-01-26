//! Semantic matching module for intelligent name comparison

pub mod normalization;
pub mod pipeline;
pub mod pluralization;
pub mod similarity;
pub mod synonyms;

pub use pipeline::{SemanticMatchDecision, SemanticMatchPipeline, SemanticMatchResult};
