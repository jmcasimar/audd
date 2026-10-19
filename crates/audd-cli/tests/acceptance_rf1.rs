//! Acceptance tests for RF1 (load two sources and compare without error)
//! and RF2 (translate entities, attributes, types, keys and constraints to IR).

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn get_fixtures_dir() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // crates
    path.pop(); // workspace root
    path.push("fixtures");
    path
}

fn get_audd_bin() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("..");
    path.push("..");
    path.push("target");
    path.push("release");
    path.push("audd");
    #[cfg(windows)]
    path.set_extension("exe");
    path
}

#[test]
fn acceptance_rf1_csv_vs_sqlite() {
    let fixtures = get_fixtures_dir();
    let csv_file = fixtures.join("adapters").join("users.csv");

    let temp_dir = tempfile::tempdir().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let out_dir = temp_dir.path().join("compare_output");

    // Create a SQLite database with matching table
    let conn = rusqlite::Connection::open(&db_path).unwrap();
    conn.execute(
        "CREATE TABLE users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT,
            age INTEGER
        )",
        [],
    )
    .unwrap();
    drop(conn);

    let output = Command::new(get_audd_bin())
        .args([
            "compare",
            "--source-a",
            csv_file.to_str().unwrap(),
            "--source-b",
            &format!("db:sqlite://{}", db_path.to_str().unwrap()),
            "--out",
            out_dir.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "CLI failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(out_dir.join("unified_schema.json").exists());
    assert!(out_dir.join("diff.json").exists());
    assert!(out_dir.join("decision_log.json").exists());
    assert!(out_dir.join("report.md").exists());
}

#[test]
fn acceptance_rf1_sql_ddl_vs_json() {
    let fixtures = get_fixtures_dir();
    let json_file = fixtures.join("adapters").join("users.json");
    let sql_file = fixtures.join("adapters").join("schema.sql");

    let temp_dir = tempfile::tempdir().unwrap();
    let out_dir = temp_dir.path().join("compare_output");

    let output = Command::new(get_audd_bin())
        .args([
            "compare",
            "--source-a",
            sql_file.to_str().unwrap(),
            "--source-b",
            json_file.to_str().unwrap(),
            "--out",
            out_dir.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "CLI failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(out_dir.join("unified_schema.json").exists());
    assert!(out_dir.join("diff.json").exists());
}

#[test]
fn acceptance_rf2_sqlite_foreign_key() {
    let temp_dir = tempfile::tempdir().unwrap();
    let db_path = temp_dir.path().join("fk_test.db");
    let out_dir = temp_dir.path().join("ir_output");

    let conn = rusqlite::Connection::open(&db_path).unwrap();
    conn.execute(
        "CREATE TABLE customers (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        )",
        [],
    )
    .unwrap();
    conn.execute(
        "CREATE TABLE orders (
            id INTEGER PRIMARY KEY,
            customer_id INTEGER NOT NULL,
            total REAL,
            FOREIGN KEY (customer_id) REFERENCES customers(id)
        )",
        [],
    )
    .unwrap();
    drop(conn);

    let output = Command::new(get_audd_bin())
        .args([
            "inspect",
            "--source",
            &format!("db:sqlite://{}", db_path.to_str().unwrap()),
            "--out",
            out_dir.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(
        output.status.success(),
        "CLI failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let ir_content = fs::read_to_string(&out_dir).unwrap();
    assert!(
        ir_content.contains("\"foreign\""),
        "Expected foreign key type in IR output"
    );
    assert!(
        ir_content.contains("referenced_table"),
        "Expected referenced_table metadata in IR output"
    );
    assert!(
        ir_content.contains("customers"),
        "Expected referenced table name in IR output"
    );
    assert!(
        ir_content.contains("customer_id"),
        "Expected foreign key field in IR output"
    );
}
