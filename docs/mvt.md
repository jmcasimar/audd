# AUDD - Minimum Verification Tests (MVT)

**Version:** 1.0.0  
**Date:** 2026-02-02  
**Status:** Initial Release

---

## Table of Contents

1. [Introduction](#introduction)
2. [Testing Philosophy](#testing-philosophy)
3. [Test Matrix](#test-matrix)
4. [Test Execution](#test-execution)
5. [Test Fixtures and Data](#test-fixtures-and-data)
6. [Pass/Fail Criteria](#passfail-criteria)
7. [Audit Findings](#audit-findings)
8. [Recommendations](#recommendations)

---

## Introduction

### Objective

This document defines the **Minimum Verification Tests (MVT)** for the AUDD project. The MVT provides a repeatable, deterministic test suite that validates the core functionality of AUDD's components:

- **Internal Representation (IR)**: Canonical schema model
- **Adapters**: Database and file connectors
- **Comparison Engine**: Matching and diff algorithms
- **Resolution Engine**: Conflict resolution and decision tracking

### Scope

The MVT covers:
- ✅ Unit tests for core logic and algorithms
- ✅ Integration tests for component interactions
- ✅ End-to-end tests for complete workflows
- ✅ Error handling and edge cases
- ✅ Determinism and consistency validation

Out of scope:
- ❌ UI/CLI testing (covered separately)
- ❌ Performance benchmarking (covered in separate performance suite)
- ❌ Load/stress testing

### Success Criteria

AUDD is considered "test-ready" and "UI-ready" when:
1. All MVT tests pass consistently (100% pass rate across 3+ runs)
2. Core workflows produce deterministic, auditable outputs
3. Edge cases and error paths are handled gracefully
4. Test coverage meets minimum thresholds (see [Coverage Goals](#coverage-goals))

---

## Testing Philosophy

### Principles

1. **Determinism First**: Same inputs → same outputs, every time
2. **Minimal but Sufficient**: Cover critical paths thoroughly, not every possible path
3. **Fast Feedback**: Tests should run quickly to enable rapid iteration
4. **Clear Contracts**: Every test validates a specific contract or invariant
5. **Reproducible**: Tests must work on any machine without manual setup
6. **Auditable**: Test failures must provide clear, actionable diagnostic information

### Coverage Goals

| Component | Target Coverage | Critical Path Coverage |
|-----------|-----------------|------------------------|
| IR | 85% | 95% |
| Adapters (File) | 80% | 90% |
| Adapters (DB) | 75% | 90% |
| Comparison Engine | 90% | 100% |
| Resolution Engine | 90% | 100% |
| **Overall** | **85%** | **95%** |

---

## Test Matrix

### 1. Internal Representation (IR)

#### 1.1 Schema Construction

| Test ID | Description | Input | Expected Output | Priority |
|---------|-------------|-------|-----------------|----------|
| IR-001 | Build IR from minimal schema | Single entity, 2 fields | Valid SourceSchema | P0 |
| IR-002 | Build IR from complex schema | 5 entities, relationships, keys | Valid SourceSchema with FK refs | P0 |
| IR-003 | Preserve field order | Ordered field list | Same order in IR | P1 |
| IR-004 | Handle duplicate entity names | Two entities named "users" | Error or disambiguation | P1 |
| IR-005 | Normalize identifier variants | "User", "user", "USERS" | Consistent normalized form | P0 |

#### 1.2 Type System

| Test ID | Description | Input | Expected Output | Priority |
|---------|-------------|-------|-----------------|----------|
| IR-010 | Map SQL types to canonical | INTEGER, VARCHAR(50), DECIMAL(10,2) | Int32, String, Decimal | P0 |
| IR-011 | Preserve type precision | DECIMAL(20,6) | CanonicalType with precision=20, scale=6 | P0 |
| IR-012 | Handle unknown types | CUSTOM_TYPE | Unknown or error with metadata | P1 |
| IR-013 | Type compatibility check | Int32 vs Int64 | Compatible | P0 |
| IR-014 | Type incompatibility check | String vs Boolean | Incompatible | P0 |

#### 1.3 Invariants

| Test ID | Description | Invariant | Validation Method | Priority |
|---------|-------------|-----------|-------------------|----------|
| IR-020 | No entity loss | Input entities = Output entities | Count comparison | P0 |
| IR-021 | No field loss | Input fields = Output fields | Count comparison | P0 |
| IR-022 | ID consistency | Entity IDs are unique and stable | Hash or ID check | P0 |
| IR-023 | Relationship consistency | FK references valid PK | Graph validation | P1 |

### 2. File Adapters

#### 2.1 CSV Adapter

| Test ID | Description | Input | Expected Output | Priority |
|---------|-------------|-------|-----------------|----------|
| FA-001 | Parse simple CSV | users.csv (3 rows, 4 cols) | SourceSchema with 1 entity, 4 fields | P0 |
| FA-002 | Handle missing headers | CSV without header row | Error or auto-generated headers | P1 |
| FA-003 | Handle different separators | Tab-separated file | Correct parsing | P1 |
| FA-004 | Infer types from data | Mixed int/string columns | Correct CanonicalType per field | P0 |
| FA-005 | Handle empty values | Cells with "" or NULL | nullable=true where applicable | P1 |
| FA-006 | Handle encoding issues | UTF-8, Latin-1 files | Correct parsing or clear error | P2 |

#### 2.2 JSON Adapter

| Test ID | Description | Input | Expected Output | Priority |
|---------|-------------|-------|-----------------|----------|
| FA-010 | Parse JSON schema | users.json (array of objects) | SourceSchema with inferred types | P0 |
| FA-011 | Handle nested objects | {"user": {"name": "...", "address": {...}}} | Flattened or nested representation | P1 |
| FA-012 | Handle arrays | {"tags": ["a", "b", "c"]} | Array type or multiple fields | P1 |
| FA-013 | Handle null values | {"email": null} | nullable=true | P0 |

#### 2.3 XML Adapter

| Test ID | Description | Input | Expected Output | Priority |
|---------|-------------|-------|-----------------|----------|
| FA-020 | Parse XML schema | users.xml with XSD | SourceSchema | P1 |
| FA-021 | Handle attributes vs elements | Mixed XML structure | Clear mapping to fields | P2 |

#### 2.4 SQL/DDL Adapter

| Test ID | Description | Input | Expected Output | Priority |
|---------|-------------|-------|-----------------|----------|
| FA-030 | Parse CREATE TABLE | Standard DDL | SourceSchema with tables | P0 |
| FA-031 | Extract constraints | PK, FK, UNIQUE, CHECK | Corresponding Key/Constraint objects | P0 |
| FA-032 | Handle views | CREATE VIEW statements | View objects in IR | P1 |

#### 2.5 Error Handling

| Test ID | Description | Input | Expected Behavior | Priority |
|---------|-------------|-------|-------------------|----------|
| FA-040 | File not found | Invalid path | Clear FileNotFoundError | P0 |
| FA-041 | Malformed CSV | CSV with inconsistent columns | ParseError with line number | P1 |
| FA-042 | Invalid JSON | JSON with syntax error | ParseError with position | P0 |
| FA-043 | Empty file | 0-byte file | EmptyFileError or warning | P1 |

### 3. Database Adapters

#### 3.1 SQLite Adapter

| Test ID | Description | Setup | Expected Output | Priority |
|---------|-------------|-------|-----------------|----------|
| DA-001 | Connect to SQLite DB | test.db with 2 tables | Successful connection | P0 |
| DA-002 | Extract table schemas | Tables: users, posts | 2 EntitySchema objects | P0 |
| DA-003 | Extract column metadata | Various data types | Correct CanonicalType mapping | P0 |
| DA-004 | Extract primary keys | PK on users.id | Key object with KeyType::Primary | P0 |
| DA-005 | Extract foreign keys | FK posts.user_id → users.id | Key with foreign reference | P0 |
| DA-006 | Extract indexes | Index on posts.title | Index object | P1 |
| DA-007 | Handle views | View: user_posts | View object in IR | P1 |

#### 3.2 MySQL Adapter

| Test ID | Description | Setup | Expected Output | Priority |
|---------|-------------|-------|-----------------|----------|
| DA-010 | Connect to MySQL | Docker MySQL instance | Successful connection | P1 |
| DA-011 | Extract schemas | Multiple tables | Correct SourceSchema | P1 |
| DA-012 | Handle MySQL-specific types | ENUM, SET, TINYINT | Correct mapping | P1 |

#### 3.3 PostgreSQL Adapter

| Test ID | Description | Setup | Expected Output | Priority |
|---------|-------------|-------|-----------------|----------|
| DA-020 | Connect to PostgreSQL | Docker Postgres instance | Successful connection | P1 |
| DA-021 | Extract schemas | Multiple tables with schemas | Correct namespace handling | P1 |
| DA-022 | Handle Postgres-specific types | UUID, JSONB, ARRAY | Correct mapping | P1 |

#### 3.4 MongoDB Adapter

| Test ID | Description | Setup | Expected Output | Priority |
|---------|-------------|-------|-----------------|----------|
| DA-030 | Connect to MongoDB | Docker MongoDB instance | Successful connection | P1 |
| DA-031 | Infer schema from sampling | Collection with 100 documents | Inferred EntitySchema | P1 |
| DA-032 | Handle nested documents | Documents with subdocuments | Nested or flattened fields | P2 |

#### 3.5 Error Handling

| Test ID | Description | Input | Expected Behavior | Priority |
|---------|-------------|-------|-------------------|----------|
| DA-040 | Invalid connection string | Malformed URL | ConnectionStringError | P0 |
| DA-041 | Connection refused | Offline database | ConnectionError | P0 |
| DA-042 | Authentication failure | Wrong credentials | AuthenticationError | P0 |
| DA-043 | Database not found | Non-existent DB name | DatabaseNotFoundError | P1 |
| DA-044 | Query timeout | Slow query | TimeoutError | P2 |

### 4. Comparison Engine

#### 4.1 Matching

| Test ID | Description | Input | Expected Output | Priority |
|---------|-------------|-------|-----------------|----------|
| CE-001 | Exact name match | A.users vs B.users | Match with high confidence | P0 |
| CE-002 | Case-insensitive match | A.Users vs B.users | Match | P0 |
| CE-003 | Pluralization match | A.user vs B.users | Match with lower confidence | P0 |
| CE-004 | Naming convention match | A.user_profile vs B.UserProfile | Match | P0 |
| CE-005 | No match | A.orders vs B.customers | Exclusive in A and B | P0 |

#### 4.2 Conflict Detection

| Test ID | Description | Scenario | Expected Conflict | Priority |
|---------|-------------|----------|-------------------|----------|
| CE-010 | Type conflict | A.age:String vs B.age:Int | TypeConflict | P0 |
| CE-011 | Nullability conflict | A.email:nullable vs B.email:NOT NULL | NullabilityConflict | P0 |
| CE-012 | Length conflict | A.name:VARCHAR(50) vs B.name:VARCHAR(100) | LengthConflict | P1 |
| CE-013 | Constraint conflict | A.id:PK vs B.id:no constraint | ConstraintConflict | P1 |

#### 4.3 Determinism

| Test ID | Description | Method | Validation | Priority |
|---------|-------------|--------|------------|----------|
| CE-020 | Scoring is stable | Run comparison 3 times | Same scores each time | P0 |
| CE-021 | Match order is stable | Run comparison 3 times | Same match order | P0 |
| CE-022 | Conflict detection is stable | Run comparison 3 times | Same conflicts | P0 |

#### 4.4 Output Validation

| Test ID | Description | Input | Expected Files | Priority |
|---------|-------------|-------|----------------|----------|
| CE-030 | Generate comparison report | Two schemas | JSON diff file | P0 |
| CE-031 | Generate unified schema | Two schemas | Merged schema JSON | P0 |
| CE-032 | Report structure validation | Comparison result | Valid JSON schema | P1 |

### 5. Resolution Engine

#### 5.1 Suggestion Generation

| Test ID | Description | Conflict Type | Expected Suggestions | Priority |
|---------|-------------|---------------|----------------------|----------|
| RE-001 | Type conflict suggestions | String vs Int | Cast, rename, manual review | P0 |
| RE-002 | Nullability suggestions | NULL vs NOT NULL | Make nullable, add default | P0 |
| RE-003 | Length suggestions | VARCHAR(50) vs VARCHAR(100) | Use larger length | P1 |

#### 5.2 Decision Tracking

| Test ID | Description | Actions | Expected Log | Priority |
|---------|-------------|---------|--------------|----------|
| RE-010 | Record accepted suggestion | Accept suggestion | Decision in log with timestamp | P0 |
| RE-011 | Record rejected suggestion | Reject suggestion | Decision in log with reason | P0 |
| RE-012 | Export decision log | Multiple decisions | JSON file with complete log | P0 |

#### 5.3 Idempotency

| Test ID | Description | Method | Validation | Priority |
|---------|-------------|--------|------------|----------|
| RE-020 | Apply resolution twice | Run resolution 2x | Same output both times | P0 |
| RE-021 | Decision log is stable | Generate log 2x | Identical content | P0 |

### 6. End-to-End Workflows

#### 6.1 File to File

| Test ID | Description | Workflow | Expected Outcome | Priority |
|---------|-------------|----------|------------------|----------|
| E2E-001 | CSV to JSON comparison | Load CSV, Load JSON, Compare | Complete comparison with diff | P0 |
| E2E-002 | Generate unified schema | Compare → Resolve → Export | Unified schema C | P0 |

#### 6.2 DB to File

| Test ID | Description | Workflow | Expected Outcome | Priority |
|---------|-------------|----------|------------------|----------|
| E2E-010 | SQLite to CSV comparison | Load SQLite, Load CSV, Compare | Complete comparison | P0 |

#### 6.3 DB to DB

| Test ID | Description | Workflow | Expected Outcome | Priority |
|---------|-------------|----------|------------------|----------|
| E2E-020 | SQLite to MySQL comparison | Load both DBs, Compare | Complete comparison | P1 |

---

## Test Execution

### Running Tests Locally

#### Prerequisites

```bash
# Install Rust (1.70+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone repository
git clone https://github.com/jmcasimar/AUDD.git
cd AUDD
```

#### Run All Tests

```bash
# Run complete test suite
cargo test --workspace

# Run with output
cargo test --workspace -- --nocapture

# Run specific crate tests
cargo test -p audd_ir
cargo test -p audd_compare
cargo test -p audd_resolution
cargo test -p audd_adapters_file
cargo test -p audd_adapters_db
```

#### Run Specific Test Categories

```bash
# Run integration tests only
cargo test --workspace --test integration_test

# Run unit tests only
cargo test --workspace --lib

# Run specific test by name
cargo test test_csv_adapter
```

#### Run with Coverage

```bash
# Install coverage tool
cargo install cargo-llvm-cov

# Generate coverage report
cargo llvm-cov --workspace --html

# View report
open target/llvm-cov/html/index.html
```

### Running Tests in CI

Tests are automatically run on:
- Every pull request
- Every push to `main` branch
- Nightly builds

See `.github/workflows/ci.yml` for CI configuration.

### Test Output Interpretation

#### Successful Test Run

```
running 42 tests
test test_ir_construction ... ok
test test_csv_parsing ... ok
...
test result: ok. 42 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

#### Failed Test Run

```
running 42 tests
test test_ir_construction ... FAILED
test test_csv_parsing ... ok
...

failures:

---- test_ir_construction stdout ----
thread 'test_ir_construction' panicked at 'assertion failed:
expected: SourceSchema { entities: 2 }
actual:   SourceSchema { entities: 1 }', tests/ir_test.rs:45:5
```

**Action**: Review the test file at the specified line, fix the code or test, rerun.

---

## Test Fixtures and Data

### Fixture Organization

```
fixtures/
├── ir/                    # IR test fixtures
│   ├── simple_a.json     # Minimal schema A
│   ├── simple_b.json     # Minimal schema B
│   └── complex.json      # Complex schema with all features
├── adapters/              # Adapter test data
│   ├── users.csv         # Sample CSV file
│   ├── users.json        # Sample JSON file
│   ├── users.xml         # Sample XML file
│   └── users.sql         # Sample DDL
├── databases/             # Database test fixtures
│   ├── test.db           # SQLite test database
│   └── schema.sql        # SQL setup script
└── scenarios/             # End-to-end test scenarios
    ├── csv_to_json/      # CSV → JSON comparison
    ├── db_to_file/       # Database → File comparison
    └── db_to_db/         # Database → Database comparison
```

### Minimal Dataset: "Simple Users"

**Purpose**: Validate basic functionality with minimal data

**entities/adapters/users.csv**:
```csv
id,username,email,age
1,alice,alice@example.com,30
2,bob,bob@example.com,25
3,charlie,charlie@example.com,35
```

**Schema**:
- Entity: `users`
- Fields: `id` (Int), `username` (String), `email` (String), `age` (Int)

### Realistic Dataset: "E-Commerce"

**Purpose**: Simulate real-world schema with relationships

**Tables**:
- `users` (id, username, email, created_at)
- `products` (id, name, price, category)
- `orders` (id, user_id, product_id, quantity, order_date)

**Relationships**:
- `orders.user_id` → `users.id`
- `orders.product_id` → `products.id`

**Variations**:
- Schema A: Snake_case naming (user_id, product_id)
- Schema B: CamelCase naming (userId, productId)
- Type differences: A.price: DECIMAL(10,2), B.price: FLOAT

This dataset tests:
- Name normalization
- Type compatibility
- Foreign key detection
- Conflict resolution

### Dataset Documentation

Each dataset includes a `README.md` with:
- Purpose and coverage
- Schema diagram
- Test scenarios it enables
- Expected comparison results

---

## Pass/Fail Criteria

### Test-Level Criteria

A test **passes** if:
1. It completes without panic/crash
2. All assertions succeed
3. Expected output matches actual output
4. No unexpected errors or warnings

A test **fails** if:
1. Panic or unhandled exception occurs
2. Any assertion fails
3. Output differs from expected
4. Unexpected error occurs

### Suite-Level Criteria

The MVT suite **passes** if:
1. 100% of P0 tests pass
2. ≥95% of P1 tests pass
3. ≥90% of P2 tests pass
4. No regressions from previous run
5. All tests are deterministic (same result across 3 runs)

### Coverage Criteria

| Component | Line Coverage | Branch Coverage | Critical Path |
|-----------|---------------|-----------------|---------------|
| IR | ≥85% | ≥80% | 100% |
| File Adapters | ≥80% | ≥75% | 95% |
| DB Adapters | ≥75% | ≥70% | 90% |
| Compare | ≥90% | ≥85% | 100% |
| Resolution | ≥90% | ≥85% | 100% |

### Release Criteria

Before a release, the following must be true:
- ✅ MVT suite passes 100%
- ✅ Coverage goals met
- ✅ No P0 or P1 security issues
- ✅ All critical bugs fixed
- ✅ Documentation updated
- ✅ CI pipeline green

---

## Audit Findings

### Current State (2026-02-02)

**Strengths**:
- ✅ Core IR module is well-defined and documented
- ✅ Comparison and resolution engines have clear contracts
- ✅ Basic integration tests exist for key workflows
- ✅ Code compiles cleanly with minimal warnings
- ✅ Existing tests pass consistently

**Gaps Identified**:

#### P0 - Critical Gaps (Must Fix Before Production)

1. **Limited Error Path Coverage**: Most tests cover happy paths only
   - **Impact**: Unexpected inputs may cause panics or data corruption
   - **Recommendation**: Add comprehensive error tests (see test_plan.md)

2. **No Security Testing**: SQL injection, SSRF, resource exhaustion not tested
   - **Impact**: Critical security vulnerabilities may exist
   - **Recommendation**: Implement security test suite immediately

3. **Missing Edge Case Coverage**: 
   - Empty files, malformed data, encoding issues not tested
   - **Impact**: Production failures on edge cases
   - **Recommendation**: Add edge case tests to each adapter

4. **No Determinism Validation**: Comparison scoring/matching not tested for consistency
   - **Impact**: Non-reproducible results undermine auditability
   - **Recommendation**: Add determinism tests (run 3x, verify identical results)

#### P1 - High Priority Gaps

5. **Database Adapter Coverage**: 
   - SQLite tested, but MySQL/PostgreSQL/MongoDB minimally tested
   - **Recommendation**: Add integration tests with Docker databases

6. **Type Mapping Coverage**: 
   - Basic types tested, but precision/scale, custom types not covered
   - **Recommendation**: Comprehensive type mapping test matrix

7. **No Regression Test Suite**: 
   - Fixed bugs may resurface
   - **Recommendation**: Add regression test for each bug fix

8. **Limited Fixture Diversity**: 
   - Most tests use simple 2-3 field schemas
   - **Recommendation**: Add realistic fixtures with 10+ tables, complex relationships

#### P2 - Medium Priority Gaps

9. **No Performance Baselines**: 
   - Unknown if changes regress performance
   - **Recommendation**: Add basic benchmarks for critical paths

10. **Documentation Examples Not Tested**: 
    - Doc examples may be out of date
    - **Recommendation**: Use doc tests to verify examples

### Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| SQL Injection in adapters | Medium | Critical | Add input validation + tests (P0) |
| Non-deterministic comparison | Low | High | Add determinism tests (P0) |
| Database connection failures | High | Medium | Improve error handling + tests (P1) |
| Type mapping errors | Medium | High | Comprehensive type tests (P1) |
| Performance regression | Low | Medium | Add benchmarks (P2) |

### Test Infrastructure Needs

**Required**:
- ✅ Cargo test framework (already in place)
- ⚠️  Docker setup for database integration tests (partially in place)
- ❌ Mock HTTP server for remote adapter tests (needs implementation)
- ❌ Golden file testing framework (needs implementation)
- ❌ Property-based testing (needs proptest integration)

**Nice to Have**:
- Fuzzing infrastructure (cargo-fuzz)
- Mutation testing
- Visual diff tools for schema comparisons

---

## Recommendations

### Immediate Actions (This Sprint)

1. **Implement Security Test Suite** (see `docs/test_plan.md` sections 1.1-1.3)
   - SQL injection prevention tests
   - SSRF prevention tests
   - Resource limit tests
   - **Estimated Effort**: 2-3 days

2. **Add Determinism Tests** for comparison engine
   - Run comparison 3x with same inputs
   - Verify identical scores, matches, conflicts
   - **Estimated Effort**: 1 day

3. **Create Comprehensive Fixtures**
   - "E-commerce" realistic dataset
   - Edge case datasets (empty, malformed, large)
   - **Estimated Effort**: 1 day

4. **Add Error Path Tests** for all adapters
   - File not found, connection refused, auth failure
   - Malformed input, timeout scenarios
   - **Estimated Effort**: 2 days

**Total Sprint Effort**: 6-7 days

### Short-Term (Next Month)

5. **Database Integration Tests**
   - Docker compose setup for MySQL, PostgreSQL, MongoDB
   - Full CRUD + schema extraction tests
   - **Estimated Effort**: 3-4 days

6. **Type Mapping Test Matrix**
   - All SQL types → CanonicalType
   - Precision/scale preservation
   - Round-trip consistency
   - **Estimated Effort**: 2 days

7. **Mock HTTP Server** for remote adapter tests
   - Simulate large files, timeouts, errors
   - Google Sheets mocking
   - **Estimated Effort**: 2 days

8. **CI/CD Enhancements**
   - Coverage reporting
   - Test result dashboards
   - Nightly regression runs
   - **Estimated Effort**: 1-2 days

### UI-Ready Checklist

Before building a UI/CLI, ensure:
- ✅ All P0 tests implemented and passing
- ✅ Security tests passing (no SQL injection, SSRF vulnerabilities)
- ✅ Determinism validated (comparison results are reproducible)
- ✅ Error handling tested (graceful failures with clear messages)
- ✅ End-to-end workflows validated with realistic data
- ✅ Documentation complete and accurate

**Estimated Timeline to UI-Ready**: 2-3 sprints (4-6 weeks)

### Long-Term Quality Goals

- **Coverage**: Maintain >85% overall, >95% on critical paths
- **Regression**: Zero regression tolerance (all old tests must pass)
- **Performance**: Comparison should complete <1s for schemas with <100 entities
- **Security**: Zero critical/high vulnerabilities in quarterly audits
- **Maintainability**: Every bug fix includes a regression test

---

## Appendix A: Test Execution Log Template

```
=== AUDD MVT Execution Log ===
Date: YYYY-MM-DD
Version: vX.Y.Z
Executor: [Name]
Environment: [Local/CI/Docker]

--- Configuration ---
Rust Version: 
OS: 
Test Scope: [All/Subset]

--- Results ---
Total Tests: 
Passed: 
Failed: 
Skipped: 
Duration: 

--- Coverage ---
Overall: 
IR: 
Adapters: 
Compare: 
Resolution: 

--- Failed Tests ---
[List of failed tests with brief description]

--- Notes ---
[Any observations, blockers, or anomalies]

--- Sign-Off ---
Passed MVT: [Yes/No]
Ready for Release: [Yes/No/Blocked]
Blocker Issues: [List of blocking issues]
```

---

## Appendix B: Quick Reference

### Common Test Commands

```bash
# Run all tests
cargo test --workspace

# Run specific module
cargo test -p audd_compare

# Run with output
cargo test -- --nocapture

# Run single test
cargo test test_csv_parsing -- --exact

# Run ignored tests
cargo test -- --ignored

# Show test list without running
cargo test -- --list
```

### Coverage Commands

```bash
# HTML coverage report
cargo llvm-cov --workspace --html

# Terminal coverage report
cargo llvm-cov --workspace

# Coverage for specific package
cargo llvm-cov -p audd_ir
```

### Debugging Failed Tests

```bash
# Run with backtrace
RUST_BACKTRACE=1 cargo test

# Run with full backtrace
RUST_BACKTRACE=full cargo test

# Run single test with debug output
cargo test test_name -- --exact --nocapture
```

---

## Appendix C: Glossary

- **MVT**: Minimum Verification Tests - the smallest set of tests needed to validate core functionality
- **IR**: Internal Representation - AUDD's canonical schema model
- **Adapter**: Component that loads schemas from external sources (files, databases)
- **Comparison**: Process of matching and diffing two schemas
- **Resolution**: Process of suggesting and applying fixes to schema conflicts
- **Determinism**: Property that same inputs always produce same outputs
- **Golden File**: Expected output file used for comparison in tests
- **Fixture**: Sample data used in tests
- **Regression**: Previously working functionality that breaks

---

**Document Version**: 1.0.0  
**Last Updated**: 2026-02-02  
**Next Review**: 2026-03-02
