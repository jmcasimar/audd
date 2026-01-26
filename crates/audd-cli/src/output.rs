//! Output file generation utilities

use std::fs;
use std::path::{Path, PathBuf};
use audd_compare::{ComparisonResult, UnifiedSchema};
use audd_resolution::DecisionLog;
use crate::error::{CliError, CliResult};

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
pub fn write_report(out_dir: &Path, log: &DecisionLog, result: &ComparisonResult) -> CliResult<PathBuf> {
    let path = out_dir.join("report.md");
    
    // Extract schema names from decision log metadata or use defaults
    let schema_a_name = log.metadata.schema_a_id.as_deref().unwrap_or("Schema A");
    let schema_b_name = log.metadata.schema_b_id.as_deref().unwrap_or("Schema B");
    
    // Generate comprehensive report using the report module
    let report_content = crate::report::generate_report(
        schema_a_name,
        schema_b_name,
        result,
        Some(log),
    );
    
    fs::write(&path, report_content).map_err(|e| CliError::OutputWrite {
        path: path.display().to_string(),
        details: e,
    })?;
    Ok(path)
}

/// Write IR schema to JSON (for inspect command)
pub fn write_ir_schema(out_dir: &Path, schema: &audd_ir::SourceSchema) -> CliResult<PathBuf> {
    let path = out_dir.join("ir.json");
    let json = schema.to_json()?;
    fs::write(&path, json).map_err(|e| CliError::OutputWrite {
        path: path.display().to_string(),
        details: e,
    })?;
    Ok(path)
}
