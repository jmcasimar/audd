//! Acceptance tests for RF6, K08: one test per rule of the default catalog
//! (`catalog/resolutions.toml`, docs/VALIDACION_TESIS_AUDD.md section 6.4).
//!
//! Each test builds the conflict the rule is meant for and checks that the catalog
//! selects exactly that rule and turns it into the expected suggestion. R-CONS-002
//! (CHECK) and R-CONS-003 (primary key) are reachable from the catalog, but the
//! comparison engine does not emit those conflicts yet (E4 is not implemented).

use std::path::PathBuf;

use audd_compare::Conflict;
use audd_resolution::{ResolutionCatalog, SuggestionEngine, SuggestionKind};

fn catalog() -> ResolutionCatalog {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../catalog");
    ResolutionCatalog::load_from_dir(dir).expect("default catalog must load")
}

fn types(a: &str, b: &str) -> Conflict {
    Conflict::type_incompatible("t".into(), "f".into(), a.into(), b.into(), 0, 0)
}

fn constraint(a: &str, b: &str) -> Conflict {
    Conflict::constraint_mismatch("t".into(), "f".into(), a.into(), b.into(), 0, 0)
}

/// The catalog selects `rule` for `conflict` and the engine returns `kind`.
fn assert_rule(conflict: Conflict, rule: &str, kind: &str) {
    let catalog = catalog();
    let found = catalog.find_rule(&conflict).map(|r| r.id.as_str());
    assert_eq!(found, Some(rule), "conflict: {:?}", conflict.evidence);

    let suggestions = SuggestionEngine::new()
        .with_catalog(catalog)
        .suggest(&conflict);
    let s = &suggestions[0];
    let got = match &s.kind {
        SuggestionKind::CastSafe { .. } => "CastSafe",
        SuggestionKind::CastRisky { .. } => "CastRisky",
        SuggestionKind::PreferType { .. } => "PreferType",
        SuggestionKind::RenameField { .. } => "RenameField",
        SuggestionKind::NoSuggestion { .. } => "NoSuggestion",
        _ => "other",
    };
    assert_eq!(got, kind);
    assert!(s.evidence.contains(&format!("catalog:{}", rule)));
}

#[test]
fn acceptance_rf6_rule_r_type_001() {
    assert_rule(types("Int32", "Int64"), "R-TYPE-001", "CastSafe");
    // "Int32 vs Int64" applies in both directions
    assert_rule(types("Int64", "Int32"), "R-TYPE-001", "CastSafe");
}

#[test]
fn acceptance_rf6_rule_r_type_002() {
    assert_rule(types("Int32", "Decimal(10, 2)"), "R-TYPE-002", "CastSafe");
    assert_rule(types("Decimal(12, 2)", "Int64"), "R-TYPE-002", "CastSafe");
}

#[test]
fn acceptance_rf6_rule_r_type_003() {
    assert_rule(
        types("Float64", "Decimal(10, 2)"),
        "R-TYPE-003",
        "CastRisky",
    );
}

#[test]
fn acceptance_rf6_rule_r_type_004() {
    assert_rule(types("Int32", "String"), "R-TYPE-004", "CastSafe");
}

#[test]
fn acceptance_rf6_rule_r_type_005() {
    assert_rule(types("String", "Int32"), "R-TYPE-005", "CastRisky");
}

#[test]
fn acceptance_rf6_rule_r_type_006() {
    assert_rule(types("Boolean", "Int32"), "R-TYPE-006", "CastSafe");
}

#[test]
fn acceptance_rf6_rule_r_type_007() {
    assert_rule(types("Date", "DateTime"), "R-TYPE-007", "CastSafe");
    assert_rule(types("Date", "Timestamp"), "R-TYPE-007", "CastSafe");
}

#[test]
fn acceptance_rf6_rule_r_type_008() {
    assert_rule(types("DateTime", "Date"), "R-TYPE-008", "CastRisky");
}

#[test]
fn acceptance_rf6_rule_r_len_001() {
    let c = Conflict::length_mismatch(
        "t".into(),
        "f".into(),
        "MaxLength: 20".into(),
        "MaxLength: 40".into(),
        0,
        0,
    );
    assert_rule(c, "R-LEN-001", "PreferType");
}

#[test]
fn acceptance_rf6_rule_r_null_001() {
    let c = Conflict::nullability_mismatch("t".into(), "f".into(), true, false, 0, 0);
    assert_rule(c, "R-NULL-001", "PreferType");
}

#[test]
fn acceptance_rf6_rule_r_cons_001() {
    assert_rule(
        constraint("Unique: true", "Unique: false"),
        "R-CONS-001",
        "PreferType",
    );
}

#[test]
fn acceptance_rf6_rule_r_cons_002() {
    assert_rule(
        constraint("Check: qty > 0", "Check: none"),
        "R-CONS-002",
        "PreferType",
    );
}

#[test]
fn acceptance_rf6_rule_r_cons_003() {
    assert_rule(
        constraint("PrimaryKey: true", "PrimaryKey: false"),
        "R-CONS-003",
        "NoSuggestion",
    );
}

#[test]
fn acceptance_rf6_rule_r_norm_001() {
    let c = Conflict::normalization_collision(
        "t".into(),
        "user_id".into(),
        "UserId".into(),
        "user_id".into(),
        0,
        1,
    );
    assert_rule(c, "R-NORM-001", "RenameField");
}

#[test]
fn acceptance_rf6_every_catalog_rule_has_a_test() {
    let tested = [
        "R-TYPE-001",
        "R-TYPE-002",
        "R-TYPE-003",
        "R-TYPE-004",
        "R-TYPE-005",
        "R-TYPE-006",
        "R-TYPE-007",
        "R-TYPE-008",
        "R-LEN-001",
        "R-NULL-001",
        "R-CONS-001",
        "R-CONS-002",
        "R-CONS-003",
        "R-NORM-001",
    ];
    let mut ids: Vec<_> = catalog().rules.iter().map(|r| r.id.clone()).collect();
    ids.sort();
    let mut expected: Vec<_> = tested.iter().map(|s| s.to_string()).collect();
    expected.sort();
    assert_eq!(
        ids, expected,
        "a catalog rule was added or removed without a test"
    );
}
