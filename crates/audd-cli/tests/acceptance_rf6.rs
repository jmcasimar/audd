//! Acceptance tests for RF6: reusable catalog of mappings and rules
//! (docs/VALIDACION_TESIS_AUDD.md, section 6, K01-K07). K08 (one test per
//! rule) lives in `crates/audd_resolution/tests/acceptance_rf6_rules.rs`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;

const SCHEMA_A: &str =
    "CREATE TABLE users (id INT PRIMARY KEY, email VARCHAR(100) NOT NULL, age INT);";
const SCHEMA_B: &str = "CREATE TABLE users (id INT PRIMARY KEY, email VARCHAR(100), age BIGINT);";

fn catalog_dir() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.pop();
    path.join("catalog")
}

fn write_pair(dir: &Path, a: &str, b: &str) -> (PathBuf, PathBuf) {
    fs::create_dir_all(dir).unwrap();
    let pa = dir.join("a.sql");
    let pb = dir.join("b.sql");
    fs::write(&pa, a).unwrap();
    fs::write(&pb, b).unwrap();
    (pa, pb)
}

/// Run `audd compare` from `dir` (so no ./catalog is picked up implicitly).
fn compare(dir: &Path, a: &Path, b: &Path, out: &Path, extra: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_audd"))
        .args(["compare", "--deterministic", "--source-a"])
        .arg(a)
        .arg("--source-b")
        .arg(b)
        .arg("--out")
        .arg(out)
        .args(extra)
        .current_dir(dir)
        .output()
        .expect("failed to run audd")
}

fn read_json(path: &Path) -> Value {
    serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap()
}

fn with_catalog(extra: &[&str]) -> Vec<String> {
    let mut args = vec!["--catalog".to_string(), catalog_dir().display().to_string()];
    args.extend(extra.iter().map(|s| s.to_string()));
    args
}

fn run_ok(dir: &Path, extra: &[String]) -> PathBuf {
    let (a, b) = write_pair(dir, SCHEMA_A, SCHEMA_B);
    let out = dir.join("out");
    let extra: Vec<&str> = extra.iter().map(|s| s.as_str()).collect();
    let o = compare(dir, &a, &b, &out, &extra);
    assert!(
        o.status.success(),
        "audd compare failed: {}",
        String::from_utf8_lossy(&o.stderr)
    );
    out
}

// K01: loading
#[test]
fn acceptance_rf6_k01_catalog_is_loaded_from_option() {
    let tmp = tempfile::tempdir().unwrap();
    let (a, b) = write_pair(tmp.path(), SCHEMA_A, SCHEMA_B);
    let catalog = catalog_dir().display().to_string();
    let o = compare(
        tmp.path(),
        &a,
        &b,
        &tmp.path().join("out"),
        &["--catalog", &catalog],
    );
    assert!(o.status.success());
    assert!(String::from_utf8_lossy(&o.stdout).contains("Loaded resolution catalog (14 rules"));
}

fn broken_catalog(dir: &Path) -> PathBuf {
    let c = dir.join("broken_catalog");
    fs::create_dir_all(&c).unwrap();
    fs::write(
        c.join("resolutions.toml"),
        "[[rule]]\nid = \"R-X\"\nthis is not toml\n",
    )
    .unwrap();
    c
}

#[test]
fn acceptance_rf6_k01_malformed_catalog_gives_clear_error() {
    let tmp = tempfile::tempdir().unwrap();
    let (a, b) = write_pair(tmp.path(), SCHEMA_A, SCHEMA_B);
    let c = broken_catalog(tmp.path()).display().to_string();
    let o = compare(
        tmp.path(),
        &a,
        &b,
        &tmp.path().join("out"),
        &["--catalog", &c],
    );
    assert!(!o.status.success());
    let err = String::from_utf8_lossy(&o.stderr);
    assert!(
        err.contains("resolutions.toml"),
        "error must name the file: {}",
        err
    );
    assert!(
        err.contains("--catalog-optional"),
        "error must suggest the option: {}",
        err
    );
}

#[test]
fn acceptance_rf6_k01_catalog_optional_continues_without_catalog() {
    let tmp = tempfile::tempdir().unwrap();
    let (a, b) = write_pair(tmp.path(), SCHEMA_A, SCHEMA_B);
    let c = broken_catalog(tmp.path()).display().to_string();
    let out = tmp.path().join("out");
    let o = compare(
        tmp.path(),
        &a,
        &b,
        &out,
        &["--catalog", &c, "--catalog-optional"],
    );
    assert!(o.status.success());
    assert!(String::from_utf8_lossy(&o.stderr).contains("continuing without it"));
    assert!(out.join("diff.json").exists());
}

// K02 + K03: catalog rules first, origin traced in every output
#[test]
fn acceptance_rf6_k02_catalog_rules_take_priority_over_engine() {
    let tmp = tempfile::tempdir().unwrap();
    let with = run_ok(&tmp.path().join("with"), &with_catalog(&[]));
    let origins: Vec<_> = read_json(&with.join("suggestions.json"))
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s["origin"].as_str().unwrap().to_string())
        .collect();
    assert!(
        origins.iter().all(|o| o.starts_with("catalog:")),
        "{:?}",
        origins
    );

    // No catalog at all: the engine's default behavior applies
    let without = run_ok(&tmp.path().join("without"), &[]);
    for s in read_json(&without.join("suggestions.json"))
        .as_array()
        .unwrap()
    {
        assert_eq!(s["origin"], "engine:default");
    }
}

#[test]
fn acceptance_rf6_k03_origin_in_suggestions_and_decision_log() {
    let tmp = tempfile::tempdir().unwrap();
    let out = run_ok(tmp.path(), &with_catalog(&[]));
    let suggestions = read_json(&out.join("suggestions.json"));
    assert!(suggestions
        .as_array()
        .unwrap()
        .iter()
        .any(|s| s["origin"] == "catalog:R-NULL-001"));

    let log = read_json(&out.join("decision_log.json"));
    for d in log["decisions"].as_array().unwrap() {
        let origin = d["metadata"]["origin"].as_str().unwrap();
        assert!(origin.starts_with("catalog:") || origin == "engine:default");
        let evidence = d["suggestion"]["evidence"].as_array().unwrap();
        assert!(evidence.iter().any(|e| e == origin));
    }
}

// K04: domain equivalences
#[test]
fn acceptance_rf6_k04_domain_aliases_produce_matches() {
    let a = "CREATE TABLE product (id INT PRIMARY KEY, price DECIMAL(10,2));";
    let b = "CREATE TABLE producto (id INT PRIMARY KEY, precio DECIMAL(10,2));";
    let tmp = tempfile::tempdir().unwrap();
    let (pa, pb) = write_pair(tmp.path(), a, b);
    let catalog = catalog_dir().display().to_string();

    let plain = tmp.path().join("plain");
    assert!(
        compare(tmp.path(), &pa, &pb, &plain, &["--catalog", &catalog])
            .status
            .success()
    );
    let plain = read_json(&plain.join("diff.json"));

    let pos = tmp.path().join("pos");
    assert!(compare(
        tmp.path(),
        &pa,
        &pb,
        &pos,
        &["--catalog", &catalog, "--domain", "pos"]
    )
    .status
    .success());
    let pos = read_json(&pos.join("diff.json"));

    assert_eq!(plain["summary"]["total_matches"], 0);
    // product <-> producto, id, price <-> precio
    assert_eq!(pos["summary"]["total_matches"], 3);
    assert_eq!(pos["summary"]["total_exclusives"], 0);
}

// K05: learning from decisions
#[test]
fn acceptance_rf6_k05_learn_produces_candidate_rules_for_review() {
    let tmp = tempfile::tempdir().unwrap();
    let out = run_ok(tmp.path(), &with_catalog(&[]));
    let catalog_before = fs::read_to_string(catalog_dir().join("resolutions.toml")).unwrap();

    let learned = tmp.path().join("learned.toml");
    let o = Command::new(env!("CARGO_BIN_EXE_audd"))
        .args(["catalog", "learn", "--from"])
        .arg(out.join("decision_log.json"))
        .arg("--out")
        .arg(&learned)
        .output()
        .unwrap();
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));

    let parsed: toml::Value = fs::read_to_string(&learned).unwrap().parse().unwrap();
    let rules = parsed["rule"].as_array().unwrap();
    let accepted = read_json(&out.join("decision_log.json"))["metadata"]["accepted_decisions"]
        .as_u64()
        .unwrap();
    assert!(!rules.is_empty() && rules.len() as u64 <= accepted);
    for r in rules {
        assert_eq!(r["source"].as_str(), Some("learned"));
        assert!(r["conflict_type"].is_str());
        assert!(r["id"].as_str().unwrap().starts_with("L-"));
    }
    // Candidates are never merged into the catalog on their own
    assert_eq!(
        fs::read_to_string(catalog_dir().join("resolutions.toml")).unwrap(),
        catalog_before
    );
}

// K06: versioning
#[test]
fn acceptance_rf6_k06_catalog_version_is_reported() {
    let tmp = tempfile::tempdir().unwrap();
    let out = run_ok(tmp.path(), &with_catalog(&[]));

    let report = fs::read_to_string(out.join("report.md")).unwrap();
    assert!(report.contains("**Catalog:** version 1.0.0 (14 rules)"));

    let catalog = &read_json(&out.join("decision_log.json"))["metadata"]["custom"]["catalog"];
    assert_eq!(catalog["version"], "1.0.0");
    let versions = catalog["rule_versions"].as_object().unwrap();
    assert_eq!(versions.len(), 14);
    assert!(versions.values().all(|v| v.as_u64().is_some()));
}

// K07: no new dependencies beyond TOML; works offline
#[test]
fn acceptance_rf6_k07_catalog_has_no_network_dependencies() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    let manifest: toml::Value = fs::read_to_string(path.join("audd_resolution/Cargo.toml"))
        .unwrap()
        .parse()
        .unwrap();
    let deps = manifest["dependencies"].as_table().unwrap();
    let allowed = [
        "audd_compare",
        "audd_ir",
        "serde",
        "serde_json",
        "toml",
        "chrono",
    ];
    for name in deps.keys() {
        assert!(
            allowed.contains(&name.as_str()),
            "unexpected dependency {}",
            name
        );
    }
}
