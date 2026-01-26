//! Integration tests for the CLI

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn get_audd_bin() -> PathBuf {
    // Find the compiled binary
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // Go up from audd-cli
    path.pop(); // Go up from crates
    path.push("target");
    path.push("debug");
    path.push("audd");
    path
}

fn get_fixtures_dir() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // Go up from audd-cli
    path.pop(); // Go up from crates
    path.push("fixtures");
    path.push("adapters");
    path
}

#[test]
fn test_help_command() {
    let output = Command::new(get_audd_bin())
        .arg("--help")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("AUDD - Dynamic Data Unification Algorithm"));
    assert!(stdout.contains("compare"));
    assert!(stdout.contains("inspect"));
    assert!(stdout.contains("load"));
}

#[test]
fn test_compare_help() {
    let output = Command::new(get_audd_bin())
        .args(&["compare", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Compare and unify"));
    assert!(stdout.contains("--source-a"));
    assert!(stdout.contains("--source-b"));
    assert!(stdout.contains("--out"));
}

#[test]
fn test_inspect_help() {
    let output = Command::new(get_audd_bin())
        .args(&["inspect", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Inspect and export"));
    assert!(stdout.contains("--source"));
    assert!(stdout.contains("--out"));
}

#[test]
fn test_inspect_csv_to_stdout() {
    let fixtures = get_fixtures_dir();
    let csv_file = fixtures.join("users.csv");

    let output = Command::new(get_audd_bin())
        .args(&["inspect", "--source", csv_file.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"source_name\": \"users\""));
    assert!(stdout.contains("\"source_type\": \"csv\""));
}

#[test]
fn test_inspect_csv_to_file() {
    let fixtures = get_fixtures_dir();
    let csv_file = fixtures.join("users.csv");
    
    let temp_dir = tempfile::tempdir().unwrap();
    let out_file = temp_dir.path().join("ir.json");

    let output = Command::new(get_audd_bin())
        .args(&[
            "inspect",
            "--source",
            csv_file.to_str().unwrap(),
            "--out",
            out_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(out_file.exists());

    let content = fs::read_to_string(&out_file).unwrap();
    assert!(content.contains("\"source_name\": \"users\""));
    assert!(content.contains("\"source_type\": \"csv\""));
}

#[test]
fn test_compare_csv_and_json() {
    let fixtures = get_fixtures_dir();
    let csv_file = fixtures.join("users.csv");
    let json_file = fixtures.join("users.json");
    
    let temp_dir = tempfile::tempdir().unwrap();
    let out_dir = temp_dir.path().join("compare_output");

    let output = Command::new(get_audd_bin())
        .args(&[
            "compare",
            "--source-a",
            csv_file.to_str().unwrap(),
            "--source-b",
            json_file.to_str().unwrap(),
            "--out",
            out_dir.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    // Check all expected output files exist
    assert!(out_dir.join("unified_schema.json").exists());
    assert!(out_dir.join("diff.json").exists());
    assert!(out_dir.join("decision_log.json").exists());
    assert!(out_dir.join("report.md").exists());

    // Verify content of unified schema
    let unified = fs::read_to_string(out_dir.join("unified_schema.json")).unwrap();
    assert!(unified.contains("\"schema_name\""));
    assert!(unified.contains("\"entities\""));

    // Verify content of report
    let report = fs::read_to_string(out_dir.join("report.md")).unwrap();
    assert!(report.contains("# AUDD Comparison Report"));
    assert!(report.contains("## Summary"));
    assert!(report.contains("Matches"));
    assert!(report.contains("Conflicts"));
}

#[test]
fn test_compare_invalid_source() {
    let output = Command::new(get_audd_bin())
        .args(&[
            "compare",
            "--source-a",
            "nonexistent.csv",
            "--source-b",
            "also_nonexistent.json",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_load_csv() {
    let fixtures = get_fixtures_dir();
    let csv_file = fixtures.join("users.csv");

    let output = Command::new(get_audd_bin())
        .args(&["load", "--source", csv_file.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Schema loaded successfully"));
    assert!(stdout.contains("Source Name: users"));
    assert!(stdout.contains("Source Type: csv"));
}
