//! AUDD CLI Library
//!
//! This library exposes core CLI functionality for testing purposes.

pub mod report;

pub use report::{generate_report, generate_json_report, JsonReport, ReportMetrics, RiskLevel};
