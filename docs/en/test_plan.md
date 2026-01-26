# AUDD Comprehensive Test Plan

**Version:** 1.0  
**Date:** 2026-01-26  
**Scope:** Database connectors, File adapters, Remote adapter, IR extensions

---

## Test Strategy Overview

### Testing Pyramid

```
         /\
        /  \  10 Property-Based Tests (Fuzzing, Invariants)
       /----\
      /      \  30 Integration Tests (Real DBs, End-to-End)
     /--------\
    /          \  110 Unit Tests (Functions, Error Paths)
   /____________\
```

**Total New Tests:** 150+  
**Current Coverage:** ~40% (220 existing tests, mostly happy path)  
**Target Coverage:** 85%+ (with focus on critical paths: 95%+)

---

## Test Categories

### 1. Security Tests (Priority: P0)

#### 1.1 SQL Injection Prevention

**File:** `tests/security/sql_injection_test.rs`

```rust
#[cfg(test)]
mod sql_injection_tests {
    use audd_adapters_db::*;
    
    #[test]
    fn test_sqlite_blocks_injection_in_table_names() {
        let malicious_cases = vec![
            "'; DROP TABLE users--",
            "table'; DELETE FROM admin WHERE 1=1; --",
            "test\0table",  // Null byte
            "t'; INSERT INTO users VALUES('hacker'); --",
            "\"; DROP TABLE users; --",  // Double quote variant
        ];
        
        for case in malicious_cases {
            // Create temp SQLite DB with malicious table name
            // Attempt to load schema
            // Verify: Either error returned OR injection not executed
            let result = validate_sqlite_identifier(case);
            assert!(
                result.is_err(),
                "Failed to block SQL injection: {}",
                case
            );
        }
    }
    
    #[test]
    fn test_mysql_blocks_injection_in_db_name() {
        let malicious_dbs = vec![
            "'; DROP DATABASE prod; --",
            "db'; DELETE FROM users; SELECT * FROM tables WHERE name='",
        ];
        
        for db_name in malicious_dbs {
            let conn_str = format!("mysql://user:pass@localhost/{}", db_name);
            let result = MysqlConnector::new(&conn_str);
            // Should fail during connection string validation
            assert!(result.is_err());
        }
    }
    
    #[test]
    fn test_parameterized_queries_used() {
        // Verify that actual SQL queries don't use string interpolation
        // This could be done via code inspection or mock database
        // that logs actual queries received
    }
}
```

#### 1.2 SSRF Prevention

**File:** `tests/security/ssrf_test.rs`

```rust
#[test]
fn test_remote_adapter_blocks_localhost() {
    let blocked_urls = vec![
        "http://localhost/file.csv",
        "http://127.0.0.1/data.json",
        "http://0.0.0.0/schema.xml",
        "http://[::1]/file.csv",  // IPv6 localhost
    ];
    
    for url in blocked_urls {
        let result = RemoteAdapter::new(url);
        assert!(
            result.is_err(),
            "Failed to block localhost URL: {}",
            url
        );
    }
}

#[test]
fn test_remote_adapter_blocks_private_ips() {
    let private_ips = vec![
        "http://10.0.0.1/file.csv",        // 10.0.0.0/8
        "http://172.16.0.1/file.csv",      // 172.16.0.0/12
        "http://192.168.1.1/file.csv",     // 192.168.0.0/16
        "http://169.254.169.254/meta-data", // AWS metadata
    ];
    
    for url in private_ips {
        let result = RemoteAdapter::new(url);
        assert!(result.is_err(), "Failed to block private IP: {}", url);
    }
}

#[test]
fn test_remote_adapter_blocks_file_protocol() {
    let file_urls = vec![
        "file:///etc/passwd",
        "file://C:/Windows/System32/config/SAM",
        "file:////etc/shadow",
    ];
    
    for url in file_urls {
        let result = RemoteAdapter::new(url);
        assert!(result.is_err(), "Failed to block file:// URL: {}", url);
    }
}

#[test]
fn test_remote_adapter_allows_valid_urls() {
    let valid_urls = vec![
        "https://example.com/data.csv",
        "http://public-api.example.com/schema.json",
        "https://docs.google.com/spreadsheets/d/ABC123/edit",
    ];
    
    for url in valid_urls {
        let result = RemoteAdapter::new(url);
        assert!(result.is_ok(), "Incorrectly blocked valid URL: {}", url);
    }
}
```

#### 1.3 Resource Exhaustion Prevention

**File:** `tests/security/resource_limits_test.rs`

```rust
#[test]
fn test_remote_adapter_enforces_size_limit() {
    // Mock HTTP server that returns 200MB response
    let mock_server = MockServer::start();
    mock_server.mock(|when, then| {
        when.path("/huge.csv");
        then.status(200)
            .body(vec![b'A'; 200 * 1024 * 1024]); // 200MB
    });
    
    let adapter = RemoteAdapter::new(&format!("{}/huge.csv", mock_server.url()));
    let result = adapter.load_schema();
    
    assert!(matches!(result, Err(AdapterError::FileTooLarge(_))));
}

#[test]
fn test_mongodb_sample_size_limits() {
    let tests = vec![
        (0, true),      // Too small
        (1, false),     // Min valid
        (10000, false), // Max valid
        (10001, true),  // Too large
        (usize::MAX, true), // Way too large
    ];
    
    for (size, should_error) in tests {
        let result = MongoConnector::new_with_sample_size("mongodb://localhost", size);
        assert_eq!(result.await.is_err(), should_error,
                   "Sample size {} should_error={}", size, should_error);
    }
}

#[test]
fn test_port_number_validation() {
    let invalid_ports = vec![
        "abc",
        "99999",  // > u16::MAX
        "-1",
        "1.5",
        "",
    ];
    
    for port in invalid_ports {
        let conn_str = format!("sqlserver://user:pass@host:{}/db", port);
        let result = SqlServerConnector::new(&conn_str);
        assert!(result.is_err(), "Failed to reject invalid port: {}", port);
    }
}
```

---

### 2. Type Mapping Tests (Priority: P0)

**File:** `tests/type_mapping/comprehensive_type_test.rs`

```rust
#[test]
fn test_decimal_precision_preserved() {
    let test_cases = vec![
        ("DECIMAL(5,2)", CanonicalType::Decimal { precision: 5, scale: 2 }),
        ("NUMERIC(10,4)", CanonicalType::Decimal { precision: 10, scale: 4 }),
        ("DECIMAL(20,6)", CanonicalType::Decimal { precision: 20, scale: 6 }),
    ];
    
    for (db_type, expected) in test_cases {
        // Test for each connector
        assert_eq!(SqliteConnector::map_type(db_type), expected);
        assert_eq!(MysqlConnector::map_type(db_type), expected);
        // etc.
    }
}

#[test]
fn test_all_sqlite_types_mapped() {
    let sqlite_types = vec![
        "INTEGER", "INT", "TINYINT", "SMALLINT", "MEDIUMINT", "BIGINT",
        "TEXT", "CHAR", "VARCHAR", "CLOB",
        "BLOB",
        "REAL", "DOUBLE", "FLOAT",
        "NUMERIC", "DECIMAL",
        "BOOLEAN",
        "DATE", "DATETIME",
    ];
    
    for db_type in sqlite_types {
        let result = SqliteConnector::map_type(db_type);
        assert_ne!(result, CanonicalType::Unknown,
                   "Unmapped type: {}", db_type);
    }
}

#[test]
fn test_type_mapping_case_insensitive() {
    let variants = vec![
        ("INTEGER", "integer", "Integer", "InTeGeR"),
        ("VARCHAR", "varchar", "Varchar", "VarChar"),
    ];
    
    for variant_set in variants {
        let results: Vec<_> = variant_set.iter()
            .map(|v| SqliteConnector::map_type(v))
            .collect();
        
        // All variants should map to same canonical type
        assert!(results.windows(2).all(|w| w[0] == w[1]));
    }
}

#[test]
fn test_type_roundtrip_consistency() {
    // For each connector:
    // 1. Map database type to canonical
    // 2. Map canonical back to database type
    // 3. Verify semantics preserved
}
```

---

### 3. Error Handling Tests (Priority: P1)

**File:** `tests/error_handling/connector_errors_test.rs`

```rust
#[test]
fn test_invalid_connection_strings() {
    let invalid_strings = vec![
        "",
        "not-a-url",
        "db:unknown://host/db",
        "db:sqlite",  // Missing path
        "db:mysql://",  // Missing auth and database
        "db:postgres://user@/db",  // Missing host
        "db:mongodb://:27017/db",  // Missing host
    ];
    
    for conn_str in invalid_strings {
        let result = create_connector(conn_str);
        assert!(result.is_err(), "Failed to reject: {}", conn_str);
        
        match result.unwrap_err() {
            DbError::InvalidConnectionString(_) => {}, // Expected
            e => panic!("Wrong error type for '{}': {:?}", conn_str, e),
        }
    }
}

#[test]
fn test_connection_refused_error() {
    // Try to connect to port that's not listening
    let conn_str = "db:mysql://user:pass@localhost:9999/db";
    let result = create_connector(conn_str);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        DbError::ConnectionError(_) => {}, // Expected
        e => panic!("Wrong error for refused connection: {:?}", e),
    }
}

#[test]
fn test_authentication_failure() {
    // Assuming test DB is running with known credentials
    let conn_str = "db:mysql://wrong:wrong@localhost/test";
    let connector = MysqlConnector::new(conn_str);
    let result = connector.load();
    
    assert!(matches!(result, Err(DbError::ConnectionError(_))));
}

#[test]
fn test_database_not_found() {
    let conn_str = "db:mysql://user:pass@localhost/nonexistent_db_12345";
    let connector = MysqlConnector::new(conn_str);
    let result = connector.load();
    
    assert!(result.is_err());
}

#[test]
fn test_malformed_database_file() {
    // Create file that's not a valid SQLite database
    let temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"This is not a SQLite database").unwrap();
    
    let conn_str = format!("db:sqlite://{}", temp_file.path().display());
    let connector = SqliteConnector::new(&conn_str);
    let result = connector.load();
    
    assert!(matches!(result, Err(DbError::ConnectionError(_))));
}
```

---

### 4. Integration Tests (Priority: P1)

**File:** `tests/integration/end_to_end_test.rs`

```rust
#[test]
fn test_sqlite_full_schema_extraction() {
    // Create test database with:
    // - Multiple tables
    // - Foreign keys
    // - Unique indexes
    // - Views
    // - Triggers
    
    let test_db = create_test_sqlite_db();
    let connector = SqliteConnector::new(test_db.path());
    let schema = connector.load().unwrap();
    
    // Verify all objects extracted
    assert_eq!(schema.entities.len(), 3);
    assert_eq!(schema.views.len(), 1);
    assert_eq!(schema.triggers.len(), 2);
    
    // Verify foreign keys
    let users_table = schema.entities.iter()
        .find(|e| e.entity_name == "users")
        .unwrap();
    assert!(users_table.keys.iter().any(|k| k.key_type == KeyType::Foreign));
    
    // Verify indexes
    assert!(!users_table.indexes.is_empty());
}

#[test]
fn test_cross_database_comparison() {
    // Create same schema in SQLite and MySQL
    // Compare them
    // Verify they match (modulo type differences)
    
    let sqlite_schema = load_from_sqlite();
    let mysql_schema = load_from_mysql();
    
    let differences = compare_schemas(&sqlite_schema, &mysql_schema);
    
    // Should only differ in type representation
    assert!(differences.iter().all(|d| matches!(d, Difference::TypeDifference(_))));
}

#[test]
fn test_remote_file_to_database_comparison() {
    // Load schema from CSV
    // Load same schema from database
    // Compare them
    
    let csv_url = "https://example.com/schema.csv";
    let db_conn = "db:postgres://localhost/test";
    
    let csv_schema = load_schema_from_url(csv_url).unwrap();
    let db_schema = load_schema_from_db(db_conn).unwrap();
    
    // Schemas should match
    assert_schemas_equivalent(&csv_schema, &db_schema);
}
```

---

### 5. Property-Based Tests (Priority: P2)

**File:** `tests/property/type_mapping_properties.rs`

Using `proptest` or `quickcheck`:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_identifier_validation_never_allows_sql_injection(
        name in "[a-zA-Z_][a-zA-Z0-9_]*"
    ) {
        // Valid identifiers should always pass validation
        assert!(validate_sqlite_identifier(&name).is_ok());
    }
    
    #[test]
    fn test_identifier_validation_blocks_special_chars(
        special_char in r"[';\"\\0]"
    ) {
        let name = format!("table{}", special_char);
        assert!(validate_sqlite_identifier(&name).is_err());
    }
    
    #[test]
    fn test_type_mapping_deterministic(
        db_type in "[A-Z]{3,10}"
    ) {
        // Same input should always produce same output
        let result1 = map_type(&db_type);
        let result2 = map_type(&db_type);
        assert_eq!(result1, result2);
    }
    
    #[test]
    fn test_url_validation_rejects_dangerous_protocols(
        protocol in "file|ftp|gopher|telnet"
    ) {
        let url = format!("{}://example.com/file.csv", protocol);
        assert!(validate_url(&url).is_err());
    }
}
```

---

### 6. Fuzzing Tests (Priority: P2)

**File:** `fuzz/fuzz_targets/connection_string_fuzzer.rs`

Using `cargo-fuzz`:

```rust
#![no_main]
use libfuzzer_sys::fuzz_target;
use audd_adapters_db::*;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        // Should never panic, only return error
        let _ = parse_connection_string(s);
    }
});
```

**File:** `fuzz/fuzz_targets/type_mapping_fuzzer.rs`

```rust
fuzz_target!(|data: &[u8]| {
    if let Ok(type_str) = std::str::from_utf8(data) {
        // Should never panic
        let _ = SqliteConnector::map_type(type_str);
        let _ = MysqlConnector::map_type(type_str);
        let _ = PostgresConnector::map_type(type_str);
    }
});
```

---

### 7. Regression Tests (Priority: P1)

**File:** `tests/regression/known_issues_test.rs`

```rust
#[test]
fn test_issue_001_decimal_precision_lost() {
    // Before fix: DECIMAL(20,4) became DECIMAL(10,2)
    // After fix: precision/scale preserved
    
    let schema = extract_schema_from_sqlite_with_decimal_20_4();
    let decimal_field = schema.entities[0].fields[0];
    
    match decimal_field.field_type {
        CanonicalType::Decimal { precision, scale } => {
            assert_eq!(precision, 20);
            assert_eq!(scale, 4);
        },
        _ => panic!("Expected Decimal type"),
    }
}

#[test]
fn test_issue_002_foreign_key_metadata_consistent() {
    // All connectors should use same metadata format
    let sqlite_fk = extract_fk_from_sqlite();
    let mysql_fk = extract_fk_from_mysql();
    let postgres_fk = extract_fk_from_postgres();
    
    // Metadata keys should match
    assert_eq!(
        sqlite_fk.metadata.keys().collect::<Vec<_>>(),
        mysql_fk.metadata.keys().collect::<Vec<_>>()
    );
}
```

---

### 8. Performance Tests (Priority: P3)

**File:** `benches/connector_benchmarks.rs`

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_sqlite_extraction(c: &mut Criterion) {
    let db = create_large_test_db(); // 100 tables, 1000 columns
    
    c.bench_function("sqlite_extraction_large_db", |b| {
        b.iter(|| {
            let connector = SqliteConnector::new(db.path());
            connector.load().unwrap()
        });
    });
}

fn bench_type_mapping(c: &mut Criterion) {
    c.bench_function("type_mapping_common_types", |b| {
        b.iter(|| {
            for type_name in &["INTEGER", "TEXT", "REAL", "BLOB"] {
                black_box(SqliteConnector::map_type(type_name));
            }
        });
    });
}

criterion_group!(benches, bench_sqlite_extraction, bench_type_mapping);
criterion_main!(benches);
```

---

## Test Data & Fixtures

### Minimal Reproducible Datasets

**File:** `tests/fixtures/databases/sqlite_test.sql`

```sql
-- Minimal SQLite database for testing
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT,
    age INTEGER
);

CREATE TABLE posts (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    content TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX idx_posts_user ON posts(user_id);

CREATE VIEW user_posts AS
SELECT u.username, p.title, p.content
FROM users u
JOIN posts p ON u.id = p.user_id;

CREATE TRIGGER update_timestamp 
BEFORE UPDATE ON posts
BEGIN
    -- Trigger logic here
END;
```

**File:** `tests/fixtures/files/schema.csv`

```csv
table_name,column_name,data_type,nullable,primary_key
users,id,INTEGER,false,true
users,username,TEXT,false,false
users,email,TEXT,true,false
posts,id,INTEGER,false,true
posts,user_id,INTEGER,false,false
posts,title,TEXT,false,false
```

**File:** `tests/fixtures/mocks/http_server.rs`

```rust
pub struct MockHttpServer {
    // Implementation for testing remote adapter
    // without real network requests
}

impl MockHttpServer {
    pub fn new() -> Self {
        // Create mock server on localhost random port
    }
    
    pub fn mock_file(&self, path: &str, content: &[u8]) {
        // Configure response for path
    }
    
    pub fn mock_google_sheets(&self, sheet_id: &str, csv_content: &str) {
        // Mock Google Sheets export endpoint
    }
    
    pub fn mock_large_file(&self, path: &str, size_mb: usize) {
        // Return file that exceeds size limit
    }
    
    pub fn mock_error(&self, path: &str, status: u16) {
        // Return HTTP error
    }
}
```

---

## CI/CD Automation

### GitHub Actions Workflow

**File:** `.github/workflows/comprehensive-tests.yml`

```yaml
name: Comprehensive Test Suite

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  security-tests:
    name: Security Tests
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run security tests
        run: cargo test --test security -- --nocapture
      
      - name: Check for SQL injection vulnerabilities
        run: cargo test sql_injection
      
      - name: Check for SSRF vulnerabilities
        run: cargo test ssrf
      
      - name: Check resource limits
        run: cargo test resource_limits

  unit-tests:
    name: Unit Tests
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
      
      - name: Run all unit tests
        run: cargo test --lib
      
      - name: Run with all features
        run: cargo test --all-features

  integration-tests:
    name: Integration Tests with Real Databases
    runs-on: ubuntu-latest
    services:
      mysql:
        image: mysql:8.0
        env:
          MYSQL_ROOT_PASSWORD: test
          MYSQL_DATABASE: test_db
        ports:
          - 3306:3306
      
      postgres:
        image: postgres:16
        env:
          POSTGRES_PASSWORD: test
          POSTGRES_DB: test_db
        ports:
          - 5432:5432
      
      mongodb:
        image: mongo:7
        ports:
          - 27017:27017
    
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
      
      - name: Wait for databases
        run: |
          sleep 10
          # Health checks
      
      - name: Run integration tests
        run: cargo test --test integration
        env:
          TEST_MYSQL_URL: mysql://root:test@localhost/test_db
          TEST_POSTGRES_URL: postgres://postgres:test@localhost/test_db
          TEST_MONGODB_URL: mongodb://localhost:27017/test_db

  coverage:
    name: Code Coverage
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          components: llvm-tools-preview
      
      - name: Install cargo-llvm-cov
        run: cargo install cargo-llvm-cov
      
      - name: Generate coverage
        run: cargo llvm-cov --all-features --lcov --output-path lcov.info
      
      - name: Upload to codecov
        uses: codecov/codecov-action@v3
        with:
          files: lcov.info
          fail_ci_if_error: true
      
      - name: Check coverage threshold
        run: |
          COVERAGE=$(cargo llvm-cov --all-features | grep -oP '\d+\.\d+(?=%)')
          echo "Coverage: $COVERAGE%"
          if (( $(echo "$COVERAGE < 85" | bc -l) )); then
            echo "Coverage below 85% threshold"
            exit 1
          fi

  fuzzing:
    name: Fuzz Testing
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: nightly
      
      - name: Install cargo-fuzz
        run: cargo install cargo-fuzz
      
      - name: Run fuzzers (short run for CI)
        run: |
          cargo fuzz run connection_string_fuzzer -- -max_total_time=60
          cargo fuzz run type_mapping_fuzzer -- -max_total_time=60
          cargo fuzz run url_parser_fuzzer -- -max_total_time=60

  audit:
    name: Security Audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/audit-check@v1
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
```

---

## Acceptance Criteria

### Test Suite Must Pass

**For PR to be merged:**
- ✅ All security tests pass (0 failures)
- ✅ All unit tests pass (110+ tests)
- ✅ All integration tests pass (30+ tests)
- ✅ Code coverage ≥ 85%
- ✅ Critical path coverage ≥ 95%
- ✅ No new security vulnerabilities from `cargo audit`
- ✅ Fuzzing runs for 1 hour without crashes
- ✅ All static analysis checks pass

**For production deployment:**
- ✅ All critical issues (C1-C5) fixed
- ✅ All high priority issues (H1-H8) fixed
- ✅ Security review completed
- ✅ Performance benchmarks within acceptable range
- ✅ Documentation updated

---

## Test Maintenance

### Continuous Monitoring

- **Weekly:** Review test failures and flakiness
- **Monthly:** Update test datasets with new edge cases
- **Quarterly:** Review coverage reports and identify gaps
- **Per Release:** Run full fuzzing suite (24 hours)

### Test Data Updates

- Add new test cases when bugs are found
- Update fixtures when database schemas change
- Expand property tests with new invariants
- Add regression tests for each bug fix

---

## Appendix: Test File Organization

```
tests/
├── security/
│   ├── sql_injection_test.rs
│   ├── ssrf_test.rs
│   └── resource_limits_test.rs
├── type_mapping/
│   ├── comprehensive_type_test.rs
│   ├── sqlite_types_test.rs
│   ├── mysql_types_test.rs
│   ├── postgres_types_test.rs
│   └── mongodb_types_test.rs
├── error_handling/
│   ├── connector_errors_test.rs
│   ├── adapter_errors_test.rs
│   └── ir_errors_test.rs
├── integration/
│   ├── end_to_end_test.rs
│   ├── cross_database_test.rs
│   └── remote_file_test.rs
├── property/
│   ├── type_mapping_properties.rs
│   └── identifier_properties.rs
├── regression/
│   └── known_issues_test.rs
└── fixtures/
    ├── databases/
    │   ├── sqlite_test.sql
    │   ├── mysql_test.sql
    │   └── postgres_test.sql
    ├── files/
    │   ├── schema.csv
    │   ├── schema.json
    │   └── schema.xml
    └── mocks/
        ├── http_server.rs
        └── database_mocks.rs

fuzz/
└── fuzz_targets/
    ├── connection_string_fuzzer.rs
    ├── type_mapping_fuzzer.rs
    └── url_parser_fuzzer.rs

benches/
└── connector_benchmarks.rs
```

---

**End of Test Plan**
