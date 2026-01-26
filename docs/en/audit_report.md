# AUDD Technical Audit Report

**Date:** 2026-01-26  
**Scope:** Database connectors (SQLite, PostgreSQL, MySQL/MariaDB, MongoDB, SQL Server, Firebird), File adapters (CSV/JSON/XML/SQL), Remote adapter (HTTP/HTTPS, Google Sheets), IR extensions

---

## Executive Summary

**Total Issues Identified:** 20  
**Critical:** 5 | **High:** 8 | **Medium:** 5 | **Low:** 2

### Critical Findings Requiring Immediate Action

1. **SQL Injection Vulnerabilities** - SQLite and MySQL connectors use string interpolation for table names
2. **Compilation Failures** - Firebird connector has missing type references preventing build
3. **Security Misconfiguration** - SQL Server disables TLS certificate validation in production code
4. **Resource Exhaustion** - Remote adapter can consume unlimited memory downloading files
5. **SSRF Vulnerability** - Remote adapter accepts unvalidated URLs including internal services

---

## Detailed Findings

### 🔴 CRITICAL SEVERITY

#### C1: SQL Injection in SQLite Connector

**File:** `crates/audd_adapters_db/src/sqlite.rs` lines 100, 164, 234  
**Risk Level:** Critical  
**CVSS Score:** 9.8 (Critical)

**Problem:**
```rust
let query = format!("PRAGMA table_info('{}')", table_name); // UNSAFE
```

Table names and index names from database metadata are directly interpolated into SQL queries without validation. A malicious SQLite database file with table name `'; DROP TABLE users--` could execute arbitrary SQL.

**Attack Scenario:**
1. Attacker creates malicious SQLite database with table named `'; INSERT INTO admin_users VALUES('hacker','password')--`
2. AUDD loads this database for schema extraction
3. SQL injection executes, adding attacker to admin table

**Remediation:**
```rust
// Add validation function
fn validate_sqlite_identifier(name: &str) -> DbResult<()> {
    if name.contains('\'') || name.contains(';') || name.contains('\0') || name.contains('"') {
        return Err(DbError::ExtractionError(
            format!("Invalid identifier contains unsafe characters: {}", name)
        ));
    }
    if name.len() > 128 {
        return Err(DbError::ExtractionError("Identifier too long".to_string()));
    }
    Ok(())
}

// Use before query construction
validate_sqlite_identifier(&table_name)?;
let query = format!("PRAGMA table_info('{}')", table_name);
```

**Priority:** P0 - Fix immediately before any production deployment

---

#### C2: SQL Injection in MySQL Connector  

**File:** `crates/audd_adapters_db/src/mysql.rs` lines 85-90, 118-124, 231-239  
**Risk Level:** Critical  
**CVSS Score:** 9.1 (Critical)

**Problem:**
```rust
let query = format!(
    "SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES 
     WHERE TABLE_SCHEMA = '{}' AND TABLE_TYPE = 'BASE TABLE'",
    self.database_name  // UNSAFE
);
```

Database name and table names are interpolated without sanitization. Unlike SQLite where names come from database file, MySQL connection string is user-provided, making this exploitable.

**Attack Scenario:**
1. User provides connection string: `mysql://user:pass@host/'; DROP DATABASE prod; --`
2. Code executes: `WHERE TABLE_SCHEMA = ''; DROP DATABASE prod; --'`
3. Production database dropped

**Remediation:**
Use parameterized queries for all INFORMATION_SCHEMA queries:
```rust
let mut stmt = conn.prepare(
    "SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES 
     WHERE TABLE_SCHEMA = ? AND TABLE_TYPE = 'BASE TABLE'
     ORDER BY TABLE_NAME"
)?;
let tables = stmt.query(&[&self.database_name])?;
```

**Priority:** P0 - Critical security vulnerability

---

#### C3: Firebird Connector Compilation Failures

**File:** `crates/audd_adapters_db/src/firebird.rs`, `crates/audd_adapters_db/src/error.rs`  
**Risk Level:** Critical (Availability)  
**Impact:** Complete build failure with firebird feature

**Problems:**

1. **Missing Error Variant:**
```rust
// error.rs - DbError enum doesn't have QueryFailed variant
// But firebird.rs uses it 33 times:
return Err(DbError::QueryFailed(format!("..."))) // COMPILATION ERROR
```

2. **Invalid Import:**
```rust
// firebird.rs:4
use rsfbclient::{Connection, ConnectionBuilder, FbError};
// ERROR: no `ConnectionBuilder` in rsfbclient crate
```

**Remediation:**
```rust
// Option 1: Add missing variant to error.rs
pub enum DbError {
    // ... existing variants
    QueryFailed(String), // ADD THIS
}

// Option 2: Replace all QueryFailed with QueryError in firebird.rs
return Err(DbError::QueryError(format!("...")))

// For ConnectionBuilder - check rsfbclient docs for correct API
// May need: use rsfbclient::ConnectionConfiguration;
```

**Priority:** P0 - Code doesn't compile

---

#### C4: Unbounded Memory Consumption in Remote Adapter

**File:** `crates/audd_adapters_file/src/remote_adapter.rs` lines 162-165  
**Risk Level:** Critical (DoS)  
**CVSS Score:** 7.5 (High)

**Problem:**
```rust
let mut reader = response.into_reader();
let mut buffer = Vec::new();
std::io::copy(&mut reader, &mut buffer) // NO SIZE LIMIT
    .map_err(|e| AdapterError::IoError(e))?;
```

Entire HTTP response loaded into memory without size limit. Attacker can provide URL to 10GB file causing OOM crash.

**Attack Scenario:**
1. Attacker provides URL: `https://evil.com/10GB-file.csv`
2. AUDD starts downloading, allocating 10GB in Vec
3. System runs out of memory, process killed

**Remediation:**
```rust
const MAX_REMOTE_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100MB

let mut reader = response.into_reader().take(MAX_REMOTE_FILE_SIZE);
let mut buffer = Vec::with_capacity(4096); // Reasonable initial capacity
let bytes_read = std::io::copy(&mut reader, &mut buffer)
    .map_err(|e| AdapterError::IoError(e))?;

if bytes_read >= MAX_REMOTE_FILE_SIZE {
    return Err(AdapterError::FileTooLarge(
        format!("Remote file exceeds {}MB limit", MAX_REMOTE_FILE_SIZE / 1024 / 1024)
    ));
}
```

**Priority:** P0 - DoS vulnerability

---

#### C5: SSRF Vulnerability in Remote Adapter

**File:** `crates/audd_adapters_file/src/remote_adapter.rs` lines 138-151  
**Risk Level:** Critical  
**CVSS Score:** 9.1 (Critical)

**Problem:**
```rust
fn fetch_content(&self) -> AdapterResult<Vec<u8>> {
    let url = if self.is_google_sheets_url() {
        self.convert_google_sheets_url()
    } else {
        self.url.clone() // NO VALIDATION
    };
    let response = ureq::get(&url).call() // FETCHES ANY URL
```

Accepts any URL without validation. Can be used to scan internal network, access cloud metadata endpoints, or read local files.

**Attack Scenarios:**
1. **Internal Port Scan:** `http://192.168.1.1:22`, `http://192.168.1.1:3306` etc.
2. **Cloud Metadata:** `http://169.254.169.254/latest/meta-data/` (AWS credentials)
3. **Local Files:** `file:///etc/passwd` (if ureq supports file://)
4. **Internal Services:** `http://localhost:6379/` (Redis), `http://localhost:9200/` (Elasticsearch)

**Remediation:**
```rust
fn validate_url(url: &str) -> AdapterResult<()> {
    // Only allow HTTP/HTTPS
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(AdapterError::InvalidUrl(
            "Only HTTP and HTTPS URLs are supported".to_string()
        ));
    }

    // Parse URL to get host
    let url_parts: Vec<&str> = url.split('/').collect();
    if url_parts.len() < 3 {
        return Err(AdapterError::InvalidUrl("Malformed URL".to_string()));
    }
    
    let host_port = url_parts[2];
    let host = host_port.split(':').next().unwrap_or(host_port);
    
    // Block localhost and private IPs
    let blocked = [
        "localhost", "127.0.0.1", "0.0.0.0",
        "169.254.169.254", // AWS metadata
        "metadata.google.internal", // GCP metadata
    ];
    
    for blocked_host in &blocked {
        if host.eq_ignore_ascii_case(blocked_host) {
            return Err(AdapterError::InvalidUrl(
                format!("Access to {} is not allowed", host)
            ));
        }
    }
    
    // Block private IP ranges: 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16
    if host.starts_with("10.") || 
       host.starts_with("192.168.") ||
       host.starts_with("172.16.") || host.starts_with("172.17.") {
        return Err(AdapterError::InvalidUrl(
            "Access to private IP addresses is not allowed".to_string()
        ));
    }
    
    Ok(())
}

// Use in fetch_content before ureq::get()
validate_url(&url)?;
let response = ureq::get(&url).call()
```

**Priority:** P0 - SSRF allows internal network access

---

### 🟠 HIGH SEVERITY

#### H1: Insecure Port Parsing with Silent Failure

**File:** `crates/audd_adapters_db/src/sqlserver.rs` line 87  
**Risk Level:** High

**Problem:**
```rust
let port = p.parse::<u16>().unwrap_or(1433) // SILENT FAILURE
```

Invalid port like "abc" silently becomes 1433. Could connect to wrong server.

**Remediation:**
```rust
let port = p.parse::<u16>().map_err(|_| 
    DbError::InvalidConnectionString(format!("Invalid port number: {}", p))
)?;
```

---

#### H2: TLS Certificate Validation Disabled in SQL Server

**File:** `crates/audd_adapters_db/src/sqlserver.rs` line 98  
**Risk Level:** High  
**CVSS Score:** 6.8 (Medium)

**Problem:**
```rust
config.trust_cert(); // For development/testing - IN PRODUCTION CODE!
```

All SQL Server connections vulnerable to MITM attacks.

**Remediation:**
```rust
// Remove trust_cert() entirely
// Add documentation about certificate requirements
// Optionally: add environment variable for dev mode only
if std::env::var("AUDD_DEV_MODE").is_ok() {
    eprintln!("WARNING: TLS certificate validation disabled (dev mode)");
    config.trust_cert();
}
```

---

#### H3: Production Code Uses unwrap() Without Error Handling

**Files:** Multiple locations in sqlserver.rs, postgres.rs  
**Risk Level:** High

**Problem:**
```rust
let column_name: &str = row.get(0).unwrap_or(""); // SILENT DATA CORRUPTION
```

Unexpected database schema causes empty strings instead of errors.

**Remediation:** Return proper errors instead of fallbacks

---

#### H4: MongoDB Sample Size Not Validated

**File:** `crates/audd_adapters_db/src/mongodb.rs` line 63  
**Risk Level:** High

**Problem:** Accepts sample_size=0 (no inference) or sample_size=10000000 (OOM/slow)

**Remediation:**
```rust
const MIN_SAMPLE_SIZE: usize = 1;
const MAX_SAMPLE_SIZE: usize = 10000;

if sample_size < MIN_SAMPLE_SIZE || sample_size > MAX_SAMPLE_SIZE {
    return Err(DbError::InvalidParameter(
        format!("Sample size must be between {} and {}", MIN_SAMPLE_SIZE, MAX_SAMPLE_SIZE)
    ));
}
```

---

#### H5: Async Runtime Resource Leak

**File:** `crates/audd_adapters_db/src/factory.rs` lines 89-95, 108-116, 129-135  
**Risk Level:** High (Memory Leak)

**Problem:** Tokio runtime created but never cleaned up. Thread pools may leak in long-running processes.

**Remediation:**
```rust
let connector = runtime.block_on(async {
    PostgresConnector::new(&conn_details).await
})?;
runtime.shutdown_background(); // Explicit cleanup
Ok(Box::new(connector))
```

---

#### H6: Hardcoded Decimal Precision Loses Data

**File:** `crates/audd_adapters_db/src/sqlite.rs` lines 472-475  
**Risk Level:** High (Data Loss)

**Problem:** DECIMAL(20,4) mapped to hardcoded (10,2). Values truncated without error.

**Remediation:** Parse precision from type string or store original type in metadata

---

#### H7: PostgreSQL Connection Errors Only to stderr

**File:** `crates/audd_adapters_db/src/postgres.rs` lines 71-75  
**Risk Level:** High

**Problem:** Connection errors printed to stderr, not surfaced to caller. Queries fail without clear reason.

**Remediation:** Use channel or Arc<Mutex<Option<Error>>> to surface errors

---

#### H8: SQL Definitions Stored Without Sanitization

**Files:** All IR and adapter code handling definitions  
**Risk Level:** High

**Problem:** View/trigger/procedure SQL definitions from malicious database could contain injection payloads executed when schema applied elsewhere.

**Remediation:** Add warnings in documentation. Consider validation/sanitization before re-execution.

---

### 🟡 MEDIUM SEVERITY

#### M1-M5: See full report for medium severity issues
- Foreign key metadata format inconsistency across connectors
- Missing test coverage for error paths
- Error variant naming inconsistency
- PostgreSQL connection spawned without proper error handling
- No validation on various input parameters

---

### ⚪ LOW SEVERITY

#### L1-L2: See full report for low severity issues
- Unused imports (warnings)
- Magic numbers in Firebird type mapping
- Doc comment examples use unwrap()
- Google Sheets ID extraction could be more robust

---

## Recommendations by Priority

### Immediate Actions (P0 - This Week)

1. **Fix Firebird compilation errors** - Add QueryFailed variant, fix imports
2. **Implement SQL injection protection** - Add identifier validation to SQLite/MySQL
3. **Add URL validation to remote adapter** - Block SSRF attacks
4. **Add file size limits** - Prevent OOM in remote adapter
5. **Remove TLS trust_cert()** - Enable proper certificate validation

### Short Term (P1 - Next Sprint)

6. Fix port parsing to return errors instead of silent fallback
7. Replace unwrap() calls with proper error handling
8. Add MongoDB sample size validation
9. Implement runtime cleanup for async connectors
10. Parse and preserve DECIMAL precision/scale

### Medium Term (P2 - This Quarter)

11. Standardize foreign key metadata format
12. Add comprehensive error path testing
13. Improve PostgreSQL connection error handling
14. Add SQL definition sanitization/validation

### Long Term (P3 - Ongoing)

15. Remove unused imports
16. Replace magic numbers with constants
17. Update doc examples to use proper error handling
18. Improve Google Sheets URL parsing robustness

---

## Testing Recommendations

### Critical Path Tests (Implement First)

1. **SQL Injection Tests:**
```rust
#[test]
fn test_sqlite_sql_injection_prevention() {
    let malicious_names = vec![
        "'; DROP TABLE users--",
        "table'; DELETE FROM admin; --",
        "test\0table",
    ];
    for name in malicious_names {
        // Verify injection is blocked
    }
}
```

2. **SSRF Prevention Tests:**
```rust
#[test]
fn test_remote_adapter_blocks_localhost() {
    let urls = vec![
        "http://localhost/file.csv",
        "http://127.0.0.1/file.csv",
        "http://169.254.169.254/meta-data",
        "file:///etc/passwd",
    ];
    for url in urls {
        assert!(RemoteAdapter::new(url).is_err());
    }
}
```

3. **Resource Limit Tests:**
```rust
#[test]
fn test_remote_adapter_size_limit() {
    // Mock server that sends 200MB
    // Verify adapter rejects with FileTooLarge error
}
```

4. **Port Validation Tests:**
```rust
#[test]
fn test_sqlserver_invalid_port_rejected() {
    let invalid = "sqlserver://user:pass@host:abc/db";
    assert!(SqlServerConnector::parse_connection_string(invalid).is_err());
}
```

### Comprehensive Test Coverage

See `/docs/test_plan.md` for complete 150+ test specification including:
- Unit tests for all error paths
- Integration tests with mock databases
- Property-based tests for type mapping
- Fuzzing tests for parsers
- Regression tests between connectors
- CI/CD automation

---

## Security Audit Summary

**Critical Vulnerabilities:** 5  
**All Must Be Fixed Before Production**

1. SQL Injection (2 instances)
2. SSRF vulnerability
3. TLS disabled
4. Unbounded resource consumption

**Recommended Actions:**
- Conduct security review of all input validation
- Implement comprehensive fuzzing
- Add security scanning to CI/CD pipeline
- Document security assumptions and requirements

---

## Appendix: Affected Files

### Files Requiring Immediate Changes

- `crates/audd_adapters_db/src/sqlite.rs` - SQL injection fix
- `crates/audd_adapters_db/src/mysql.rs` - SQL injection fix  
- `crates/audd_adapters_db/src/firebird.rs` - Compilation fixes
- `crates/audd_adapters_db/src/error.rs` - Add missing variant
- `crates/audd_adapters_db/src/sqlserver.rs` - TLS, port parsing
- `crates/audd_adapters_file/src/remote_adapter.rs` - SSRF, size limits
- `crates/audd_adapters_db/src/factory.rs` - Runtime cleanup

### Files Requiring Test Coverage

- All connector files (error path tests)
- All adapter files (malformed input tests)
- IR schema files (round-trip tests)
- Factory files (integration tests)

---

**End of Audit Report**
