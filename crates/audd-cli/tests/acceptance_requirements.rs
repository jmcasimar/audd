//! Acceptance tests for RF4, RF5, RNF1, RNF3 and RNF4
//! (docs/VALIDACION_TESIS_AUDD.md, sections 2 and 3).
//!
//! The comparison tests run the real `audd` binary on a small, known DDL pair:
//!
//! - `users` exists in both schemas: `id` identical, `email` with a nullability
//!   conflict, `age` Int32 vs Int64, plus one exclusive field per side
//! - `logs` only exists in A and `tags` only in B

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

const SCHEMA_A: &str = "CREATE TABLE users (
  id INT PRIMARY KEY,
  email VARCHAR(100) NOT NULL,
  age INT,
  only_a TEXT
);
CREATE TABLE logs (id INT PRIMARY KEY);
";

const SCHEMA_B: &str = "CREATE TABLE users (
  id INT PRIMARY KEY,
  email VARCHAR(100),
  age BIGINT,
  only_b TEXT
);
CREATE TABLE tags (id INT PRIMARY KEY);
";

fn workspace_root() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // crates
    path.pop(); // workspace root
    path
}

/// Run `audd compare` on the known pair and return the output directory.
fn compare_known_pair(dir: &Path, extra: &[&str]) -> PathBuf {
    let a = dir.join("a.sql");
    let b = dir.join("b.sql");
    fs::write(&a, SCHEMA_A).unwrap();
    fs::write(&b, SCHEMA_B).unwrap();
    let out = dir.join("out");

    let output = Command::new(env!("CARGO_BIN_EXE_audd"))
        .args(["compare", "--source-a"])
        .arg(&a)
        .arg("--source-b")
        .arg(&b)
        .arg("--out")
        .arg(&out)
        .args(extra)
        .current_dir(dir)
        .output()
        .expect("failed to run audd");
    assert!(
        output.status.success(),
        "audd compare failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    out
}

fn read_json(path: &Path) -> Value {
    serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap()
}

fn entity<'a>(unified: &'a Value, name: &str) -> &'a Value {
    unified["entities"]
        .as_array()
        .unwrap()
        .iter()
        .find(|e| e["entity_name"] == name)
        .unwrap_or_else(|| panic!("entity {} not in unified schema", name))
}

fn field<'a>(entity: &'a Value, name: &str) -> &'a Value {
    entity["fields"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["field"]["field_name"] == name)
        .unwrap_or_else(|| panic!("field {} not in unified entity", name))
}

// ---------------------------------------------------------------------------
// RF4: unified schema with matches and exclusives, annotated with their origin
// ---------------------------------------------------------------------------

#[test]
fn acceptance_rf4_unified_schema_origin() {
    let tmp = tempfile::tempdir().unwrap();
    let out = compare_known_pair(tmp.path(), &["--deterministic"]);
    let unified = read_json(&out.join("unified_schema.json"));

    // Entities: matched, A only, B only
    assert_eq!(entity(&unified, "users")["origin"], "BOTH");
    assert_eq!(entity(&unified, "logs")["origin"], "A");
    assert_eq!(entity(&unified, "tags")["origin"], "B");

    // Fields: origin and state for a match, a conflict and each exclusive
    let users = entity(&unified, "users");
    for (name, origin, state) in [
        ("id", "BOTH", "matched"),
        ("email", "BOTH", "conflicted"),
        ("age", "BOTH", "conflicted"),
        ("only_a", "A", "exclusive"),
        ("only_b", "B", "exclusive"),
    ] {
        let f = field(users, name);
        assert_eq!(f["origin"], origin, "origin of {}", name);
        assert_eq!(f["state"], state, "state of {}", name);
    }
}

#[test]
fn acceptance_rf4_unified_schema_is_the_union() {
    let tmp = tempfile::tempdir().unwrap();
    let out = compare_known_pair(tmp.path(), &["--deterministic"]);
    let unified = read_json(&out.join("unified_schema.json"));

    let mut names: Vec<String> = unified["entities"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| e["entity_name"].as_str().unwrap().to_string())
        .collect();
    names.sort();
    assert_eq!(names, vec!["logs", "tags", "users"]);

    // Union of the fields of `users`: 3 shared + 1 per side, each exactly once
    let mut fields: Vec<String> = entity(&unified, "users")["fields"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f["field"]["field_name"].as_str().unwrap().to_string())
        .collect();
    fields.sort();
    assert_eq!(fields, vec!["age", "email", "id", "only_a", "only_b"]);
}

// ---------------------------------------------------------------------------
// RF5: execution and decisions recorded in an auditable way
// ---------------------------------------------------------------------------

#[test]
fn acceptance_rf5_decision_log_auditability() {
    let tmp = tempfile::tempdir().unwrap();
    let out = compare_known_pair(tmp.path(), &[]);
    let log = read_json(&out.join("decision_log.json"));

    let decisions = log["decisions"].as_array().unwrap();
    assert!(!decisions.is_empty(), "expected at least one decision");
    for d in decisions {
        // ID, suggestion, who decided, rationale and time
        assert!(d["id"].as_str().is_some_and(|s| !s.is_empty()));
        assert!(d["suggestion"]["id"].is_string());
        assert!(d["accepted"].is_boolean());
        assert!(d["rationale"].as_str().is_some_and(|s| !s.is_empty()));
        assert!(d["source"].is_object(), "decision must say who decided");
        let ts = d["timestamp"].as_str().unwrap();
        assert!(
            chrono::DateTime::parse_from_rfc3339(ts).is_ok(),
            "timestamp must be RFC 3339: {}",
            ts
        );
        assert!(
            !ts.starts_with("1970"),
            "without --deterministic the real time is recorded"
        );
    }

    let meta = &log["metadata"];
    assert_eq!(meta["total_decisions"], decisions.len());
    let accepted = decisions.iter().filter(|d| d["accepted"] == true).count();
    assert_eq!(meta["accepted_decisions"], accepted);
}

#[test]
fn acceptance_rf5_all_suggestions_are_recorded() {
    // B01: every suggestion is written, including those not auto-accepted
    let tmp = tempfile::tempdir().unwrap();
    let out = compare_known_pair(tmp.path(), &["--deterministic"]);
    let diff = read_json(&out.join("diff.json"));
    let suggestions = read_json(&out.join("suggestions.json"));
    let suggestions = suggestions.as_array().unwrap();

    let conflicts = diff["conflicts"].as_array().unwrap().len();
    assert!(conflicts > 0);
    for i in 0..conflicts {
        assert!(
            suggestions.iter().any(|s| s["conflict_index"] == i),
            "conflict {} has no recorded suggestion",
            i
        );
    }
    for s in suggestions {
        assert!(s["accepted_auto"].is_boolean());
        assert!(s["confidence"].is_number());
    }
}

#[test]
fn acceptance_rf5_report_summarizes_execution() {
    let tmp = tempfile::tempdir().unwrap();
    let out = compare_known_pair(tmp.path(), &["--deterministic"]);
    let report = fs::read_to_string(out.join("report.md")).unwrap();

    assert!(report.contains("## Executive Summary"));
    assert!(report.contains("## Decision Log"));
    let log = read_json(&out.join("decision_log.json"));
    for d in log["decisions"].as_array().unwrap() {
        let id = d["id"].as_str().unwrap();
        assert!(report.contains(id), "report does not list decision {}", id);
    }
}

// ---------------------------------------------------------------------------
// RNF1: portability (CI on three operating systems, no network in the core)
// ---------------------------------------------------------------------------

#[test]
fn acceptance_rnf1_ci_runs_tests_on_linux_windows_and_macos() {
    let ci = fs::read_to_string(workspace_root().join(".github/workflows/ci.yml")).unwrap();
    let test_job = ci
        .split("\n  test:")
        .nth(1)
        .expect("ci.yml must have a `test` job");
    let test_job = test_job.split("\n  build:").next().unwrap();
    for os in ["ubuntu-latest", "windows-latest", "macos-latest"] {
        assert!(test_job.contains(os), "test job does not run on {}", os);
    }
}

fn manifest(path: &str) -> toml::Value {
    let text = fs::read_to_string(workspace_root().join(path)).unwrap();
    text.parse().unwrap()
}

#[test]
fn acceptance_rnf1_core_has_no_network_dependencies_by_default() {
    const NETWORK: [&str; 5] = ["ureq", "url", "reqwest", "hyper", "curl"];

    // Comparison, IR and resolution crates: no network crates at all
    for krate in ["audd_ir", "audd_compare", "audd_resolution"] {
        let m = manifest(&format!("crates/{}/Cargo.toml", krate));
        let deps = m.get("dependencies").and_then(|d| d.as_table());
        for dep in NETWORK {
            assert!(
                deps.is_none_or(|d| !d.contains_key(dep)),
                "{} depends on {}",
                krate,
                dep
            );
        }
    }

    // File adapters: networking only behind the optional `remote` feature
    let files = manifest("crates/audd_adapters_file/Cargo.toml");
    for dep in ["ureq", "url"] {
        let d = &files["dependencies"][dep];
        assert_eq!(
            d["optional"].as_bool(),
            Some(true),
            "{} must be optional",
            dep
        );
    }
    let defaults = files["features"]["default"].as_array().unwrap();
    assert!(!defaults.iter().any(|f| f.as_str() == Some("remote")));

    // The CLI does not enable it by default either
    let cli = manifest("crates/audd-cli/Cargo.toml");
    let defaults = cli["features"]["default"].as_array().unwrap();
    assert!(!defaults.iter().any(|f| f.as_str() == Some("remote")));
    assert_eq!(
        cli["dependencies"]["audd_adapters_file"]["default-features"].as_bool(),
        Some(false)
    );
}

// ---------------------------------------------------------------------------
// RNF3: extensibility (a new adapter does not touch audd_compare)
// ---------------------------------------------------------------------------

/// Minimal adapter for a toy format: one `entity: field:type, field:type` per line.
struct ToyAdapter;

impl audd_adapters_file::SchemaAdapter for ToyAdapter {
    fn load(&self, path: &Path) -> audd_adapters_file::AdapterResult<audd_ir::SourceSchema> {
        let text = fs::read_to_string(path)?;
        let entities = text
            .lines()
            .filter_map(|line| line.split_once(':'))
            .map(|(name, fields)| {
                let fields = fields
                    .split(',')
                    .filter_map(|f| f.trim().split_once(' '))
                    .map(|(n, t)| {
                        let ty = match t {
                            "int" => audd_ir::CanonicalType::Int32,
                            _ => audd_ir::CanonicalType::String,
                        };
                        audd_ir::FieldSchema::builder()
                            .field_name(n)
                            .canonical_type(ty)
                            .nullable(true)
                            .build()
                    })
                    .collect();
                audd_ir::EntitySchema::builder()
                    .entity_name(name.trim())
                    .fields(fields)
                    .build()
            })
            .collect();
        Ok(audd_ir::SourceSchema::builder()
            .source_name("toy")
            .source_type("toy")
            .entities(entities)
            .build())
    }
}

#[test]
fn acceptance_rnf3_new_adapter_only_implements_the_trait() {
    let tmp = tempfile::tempdir().unwrap();
    let toy = tmp.path().join("schema.toy");
    fs::write(&toy, "users: id int, email text\n").unwrap();

    // Used through the same trait object the factories return
    let adapter: Box<dyn audd_adapters_file::SchemaAdapter> = Box::new(ToyAdapter);
    let schema_a = adapter.load(&toy).unwrap();

    let sql = tmp.path().join("b.sql");
    fs::write(&sql, "CREATE TABLE users (id INT, email VARCHAR(50));").unwrap();
    let schema_b = audd_adapters_file::load_schema_from_file(&sql).unwrap();

    // audd_compare consumes the IR unchanged
    let result = audd_compare::compare(
        &schema_a,
        &schema_b,
        &audd_compare::CompareConfig::default(),
    );
    assert_eq!(result.summary.total_matches, 3); // entity + 2 fields
    assert_eq!(result.summary.total_conflicts, 0);
}

#[test]
fn acceptance_rnf3_extension_steps_are_documented() {
    let files = fs::read_to_string(workspace_root().join("docs/adapters_files.md")).unwrap();
    assert!(files.contains("Implement the `SchemaAdapter` trait"));
    assert!(files.contains("factory.rs"));
    let db = fs::read_to_string(workspace_root().join("docs/adapters_db.md")).unwrap();
    assert!(db.contains("DbSchemaConnector"));
}

// ---------------------------------------------------------------------------
// RNF4: transparency (readable report with suggestions and evidence)
// ---------------------------------------------------------------------------

#[test]
fn acceptance_rnf4_report_is_readable() {
    let tmp = tempfile::tempdir().unwrap();
    let out = compare_known_pair(tmp.path(), &["--deterministic"]);
    let report = fs::read_to_string(out.join("report.md")).unwrap();

    for section in [
        "## Executive Summary",
        "### Matches",
        "### Exclusives",
        "### Conflicts",
        "## Resolution Suggestions",
    ] {
        assert!(report.contains(section), "report lacks {}", section);
    }

    // Every conflict appears with its evidence (from_a, from_b, rule)
    let diff = read_json(&out.join("diff.json"));
    let conflicts = diff["conflicts"].as_array().unwrap();
    assert_eq!(
        // Evidence lines are nested under each conflict
        report.matches("\n  - **From Schema A**:").count(),
        conflicts.len()
    );
    for c in conflicts {
        let ev = &c["evidence"];
        for key in ["from_a", "from_b", "rule"] {
            let text = ev[key].as_str().unwrap();
            assert!(report.contains(text), "report lacks evidence `{}`", text);
        }
    }

    // And every suggestion is listed
    for s in read_json(&out.join("suggestions.json")).as_array().unwrap() {
        let id = s["suggestion_id"].as_str().unwrap();
        assert!(
            report.contains(id),
            "report does not list suggestion {}",
            id
        );
    }
}
