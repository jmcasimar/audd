//! Output file generation utilities

use crate::error::{CliError, CliResult};
use audd_compare::{ComparisonResult, Conflict, UnifiedSchema};
use audd_resolution::{DecisionLog, Suggestion};
use std::fs;
use std::path::{Path, PathBuf};

/// Entry for `suggestions.json`, capturing every suggestion produced for a conflict
/// regardless of whether it was auto-accepted.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SuggestionEntry {
    pub conflict_index: usize,
    pub entity_name: String,
    pub field_name: Option<String>,
    pub conflict_type: String,
    pub suggestion_id: String,
    pub suggestion_kind: String,
    pub confidence: f64,
    pub accepted_auto: bool,
    pub explanation: String,
    /// `catalog:<rule id>` or `engine:default` (K03)
    pub origin: String,
}

impl SuggestionEntry {
    pub fn new(
        conflict_index: usize,
        conflict: &Conflict,
        suggestion: &Suggestion,
        accepted_auto: bool,
    ) -> Self {
        Self {
            conflict_index,
            entity_name: conflict.entity_name.clone(),
            field_name: conflict.field_name.clone(),
            conflict_type: format!("{:?}", conflict.conflict_type),
            suggestion_id: suggestion.id.clone(),
            suggestion_kind: suggestion_kind_name(&suggestion.kind),
            confidence: suggestion.confidence.value(),
            accepted_auto,
            explanation: suggestion.explanation.clone(),
            origin: audd_resolution::suggestion_origin(suggestion),
        }
    }
}

fn suggestion_kind_name(kind: &audd_resolution::SuggestionKind) -> String {
    match kind {
        audd_resolution::SuggestionKind::CastSafe { .. } => "CastSafe".to_string(),
        audd_resolution::SuggestionKind::CastRisky { .. } => "CastRisky".to_string(),
        audd_resolution::SuggestionKind::RenameField { .. } => "RenameField".to_string(),
        audd_resolution::SuggestionKind::PreferType { .. } => "PreferType".to_string(),
        audd_resolution::SuggestionKind::SplitField { .. } => "SplitField".to_string(),
        audd_resolution::SuggestionKind::MergeFields { .. } => "MergeFields".to_string(),
        audd_resolution::SuggestionKind::NoSuggestion { .. } => "NoSuggestion".to_string(),
    }
}

/// Ensure output directory exists
pub fn ensure_output_dir(out_dir: &Path) -> CliResult<()> {
    if !out_dir.exists() {
        fs::create_dir_all(out_dir).map_err(|e| CliError::OutputDirCreation {
            path: out_dir.display().to_string(),
            details: e,
        })?;
    }
    Ok(())
}

/// Write unified schema to JSON file
pub fn write_unified_schema(out_dir: &Path, schema: &UnifiedSchema) -> CliResult<PathBuf> {
    let path = out_dir.join("unified_schema.json");
    let json = serde_json::to_string_pretty(schema)?;
    fs::write(&path, json).map_err(|e| CliError::OutputWrite {
        path: path.display().to_string(),
        details: e,
    })?;
    Ok(path)
}

/// Write comparison diff to JSON file
pub fn write_diff(out_dir: &Path, result: &ComparisonResult) -> CliResult<PathBuf> {
    let path = out_dir.join("diff.json");
    let json = serde_json::to_string_pretty(result)?;
    fs::write(&path, json).map_err(|e| CliError::OutputWrite {
        path: path.display().to_string(),
        details: e,
    })?;
    Ok(path)
}

/// Write decision log to JSON file
pub fn write_decision_log(out_dir: &Path, log: &DecisionLog) -> CliResult<PathBuf> {
    let path = out_dir.join("decision_log.json");
    let json = log.to_json()?;
    fs::write(&path, json).map_err(|e| CliError::OutputWrite {
        path: path.display().to_string(),
        details: e,
    })?;
    Ok(path)
}

/// Write markdown report
pub fn write_report(
    out_dir: &Path,
    log: &DecisionLog,
    result: &ComparisonResult,
) -> CliResult<PathBuf> {
    let path = out_dir.join("report.md");

    // Extract schema names from decision log metadata or use defaults
    let schema_a_name = log.metadata.schema_a_id.as_deref().unwrap_or("Schema A");
    let schema_b_name = log.metadata.schema_b_id.as_deref().unwrap_or("Schema B");

    // Generate comprehensive report using the report module
    let report_content =
        audd_cli::report::generate_report(schema_a_name, schema_b_name, result, Some(log));

    fs::write(&path, report_content).map_err(|e| CliError::OutputWrite {
        path: path.display().to_string(),
        details: e,
    })?;
    Ok(path)
}

/// Write JSON report (optional structured export)
pub fn write_json_report(
    out_dir: &Path,
    log: &DecisionLog,
    result: &ComparisonResult,
) -> CliResult<PathBuf> {
    let path = out_dir.join("report.json");

    // Extract schema names from decision log metadata or use defaults
    let schema_a_name = log.metadata.schema_a_id.as_deref().unwrap_or("Schema A");
    let schema_b_name = log.metadata.schema_b_id.as_deref().unwrap_or("Schema B");

    // Generate JSON report
    let json_report =
        audd_cli::report::generate_json_report(schema_a_name, schema_b_name, result, Some(log));

    let json = serde_json::to_string_pretty(&json_report)?;
    fs::write(&path, json).map_err(|e| CliError::OutputWrite {
        path: path.display().to_string(),
        details: e,
    })?;
    Ok(path)
}

/// Write IR schema to JSON (for inspect command)
#[allow(dead_code)]
pub fn write_ir_schema(out_dir: &Path, schema: &audd_ir::SourceSchema) -> CliResult<PathBuf> {
    let path = out_dir.join("ir.json");
    let json = schema.to_json()?;
    fs::write(&path, json).map_err(|e| CliError::OutputWrite {
        path: path.display().to_string(),
        details: e,
    })?;
    Ok(path)
}

/// Write all generated suggestions to `suggestions.json`.
///
/// This includes suggestions that were not auto-accepted, closing gap B01.
pub fn write_suggestions(out_dir: &Path, suggestions: &[SuggestionEntry]) -> CliResult<PathBuf> {
    let path = out_dir.join("suggestions.json");
    let json = serde_json::to_string_pretty(suggestions)?;
    fs::write(&path, json).map_err(|e| CliError::OutputWrite {
        path: path.display().to_string(),
        details: e,
    })?;
    Ok(path)
}
