# Plan de Pruebas Integral para AUDD

**Versión:** 1.0  
**Fecha:** 2026-01-26  
**Alcance:** Conectores de base de datos, Adaptadores de archivos, Adaptador remoto, Extensiones IR

---

## Resumen de Estrategia de Pruebas

### Pirámide de Pruebas

```
         /\
        /  \  10 Pruebas Basadas en Propiedades (Fuzzing, Invariantes)
       /----\
      /      \  30 Pruebas de Integración (BDs Reales, Extremo a Extremo)
     /--------\
    /          \  110 Pruebas Unitarias (Funciones, Rutas de Error)
   /____________\
```

**Total de Pruebas Nuevas:** 150+  
**Cobertura Actual:** ~40% (220 pruebas existentes, principalmente caminos felices)  
**Cobertura Objetivo:** 85%+ (con enfoque en rutas críticas: 95%+)

---

## Categorías de Pruebas

### 1. Pruebas de Seguridad (Prioridad: P0)

#### 1.1 Prevención de Inyección SQL

**Archivo:** `tests/security/sql_injection_test.rs`

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
            // Crear BD SQLite temporal con nombre de tabla maliciosa
            // Intentar cargar schema
            // Verificar: Error retornado O inyección no ejecutada
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
            // Debería fallar durante validación de cadena de conexión
            assert!(result.is_err());
        }
    }
    
    #[test]
    fn test_parameterized_queries_used() {
        // Verificar que las queries SQL reales no usen interpolación de cadenas
        // Esto podría hacerse mediante inspección de código o base de datos mock
        // que registre las queries reales recibidas
    }
}
```

#### 1.2 Prevención de SSRF

**Archivo:** `tests/security/ssrf_test.rs`

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

#### 1.3 Prevención de Agotamiento de Recursos

**Archivo:** `tests/security/resource_limits_test.rs`

```rust
#[test]
fn test_remote_adapter_enforces_size_limit() {
    // Servidor HTTP mock que retorna respuesta de 200MB
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
        (0, true),      // Muy pequeño
        (1, false),     // Mínimo válido
        (10000, false), // Máximo válido
        (10001, true),  // Muy grande
        (usize::MAX, true), // Excesivamente grande
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

### 2. Pruebas de Mapeo de Tipos (Prioridad: P0)

**Archivo:** `tests/type_mapping/comprehensive_type_test.rs`

```rust
#[test]
fn test_decimal_precision_preserved() {
    let test_cases = vec![
        ("DECIMAL(5,2)", CanonicalType::Decimal { precision: 5, scale: 2 }),
        ("NUMERIC(10,4)", CanonicalType::Decimal { precision: 10, scale: 4 }),
        ("DECIMAL(20,6)", CanonicalType::Decimal { precision: 20, scale: 6 }),
    ];
    
    for (db_type, expected) in test_cases {
        // Probar para cada conector
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
        
        // Todas las variantes deberían mapearse al mismo tipo canónico
        assert!(results.windows(2).all(|w| w[0] == w[1]));
    }
}

#[test]
fn test_type_roundtrip_consistency() {
    // Para cada conector:
    // 1. Mapear tipo de base de datos a canónico
    // 2. Mapear canónico de vuelta a tipo de base de datos
    // 3. Verificar que la semántica se preserva
}
```

---

### 3. Pruebas de Manejo de Errores (Prioridad: P1)

**Archivo:** `tests/error_handling/connector_errors_test.rs`

```rust
#[test]
fn test_invalid_connection_strings() {
    let invalid_strings = vec![
        "",
        "not-a-url",
        "db:unknown://host/db",
        "db:sqlite",  // Missing path
        "db:mysql://",  // Missing auth and database
        "db:postgresql://user@/db",  // Missing host
        "db:mongodb://:27017/db",  // Missing host
    ];
    
    for conn_str in invalid_strings {
        let result = create_connector(conn_str);
        assert!(result.is_err(), "Failed to reject: {}", conn_str);
        
        match result.unwrap_err() {
            DbError::InvalidConnectionString(_) => {}, // Esperado
            e => panic!("Wrong error type for '{}': {:?}", conn_str, e),
        }
    }
}

#[test]
fn test_connection_refused_error() {
    // Intentar conectarse a puerto que no está escuchando
    let conn_str = "db:mysql://user:pass@localhost:9999/db";
    let result = create_connector(conn_str);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        DbError::ConnectionError(_) => {}, // Esperado
        e => panic!("Wrong error for refused connection: {:?}", e),
    }
}

#[test]
fn test_authentication_failure() {
    // Asumiendo que BD de prueba está ejecutándose con credenciales conocidas
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
    // Crear archivo que no es una base de datos SQLite válida
    let temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(b"This is not a SQLite database").unwrap();
    
    let conn_str = format!("db:sqlite://{}", temp_file.path().display());
    let connector = SqliteConnector::new(&conn_str);
    let result = connector.load();
    
    assert!(matches!(result, Err(DbError::ConnectionError(_))));
}
```

---

### 4. Pruebas de Integración (Prioridad: P1)

**Archivo:** `tests/integration/end_to_end_test.rs`

```rust
#[test]
fn test_sqlite_full_schema_extraction() {
    // Crear base de datos de prueba con:
    // - Múltiples tablas
    // - Claves foráneas
    // - Índices únicos
    // - Vistas
    // - Triggers
    
    let test_db = create_test_sqlite_db();
    let connector = SqliteConnector::new(test_db.path());
    let schema = connector.load().unwrap();
    
    // Verificar que todos los objetos fueron extraídos
    assert_eq!(schema.entities.len(), 3);
    assert_eq!(schema.views.len(), 1);
    assert_eq!(schema.triggers.len(), 2);
    
    // Verificar claves foráneas
    let users_table = schema.entities.iter()
        .find(|e| e.entity_name == "users")
        .unwrap();
    assert!(users_table.keys.iter().any(|k| k.key_type == KeyType::Foreign));
    
    // Verificar índices
    assert!(!users_table.indexes.is_empty());
}

#[test]
fn test_cross_database_comparison() {
    // Crear mismo schema en SQLite y MySQL
    // Compararlos
    // Verificar que coinciden (salvo diferencias de tipos)
    
    let sqlite_schema = load_from_sqlite();
    let mysql_schema = load_from_mysql();
    
    let differences = compare_schemas(&sqlite_schema, &mysql_schema);
    
    // Solo deberían diferir en representación de tipos
    assert!(differences.iter().all(|d| matches!(d, Difference::TypeDifference(_))));
}

#[test]
fn test_remote_file_to_database_comparison() {
    // Cargar schema desde CSV
    // Cargar mismo schema desde base de datos
    // Compararlos
    
    let csv_url = "https://example.com/schema.csv";
    let db_conn = "db:postgresql://localhost/test";
    
    let csv_schema = load_schema_from_url(csv_url).unwrap();
    let db_schema = load_schema_from_db(db_conn).unwrap();
    
    // Los schemas deberían ser equivalentes
    assert_schemas_equivalent(&csv_schema, &db_schema);
}
```

---

### 5. Pruebas Basadas en Propiedades (Prioridad: P2)

**Archivo:** `tests/property/type_mapping_properties.rs`

Usando `proptest` o `quickcheck`:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_identifier_validation_never_allows_sql_injection(
        name in "[a-zA-Z_][a-zA-Z0-9_]*"
    ) {
        // Identificadores válidos siempre deberían pasar validación
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
        // Misma entrada siempre debería producir misma salida
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

### 6. Pruebas de Fuzzing (Prioridad: P2)

**Archivo:** `fuzz/fuzz_targets/connection_string_fuzzer.rs`

Usando `cargo-fuzz`:

```rust
#![no_main]
use libfuzzer_sys::fuzz_target;
use audd_adapters_db::*;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        // Nunca debería entrar en pánico, solo retornar error
        let _ = parse_connection_string(s);
    }
});
```

**Archivo:** `fuzz/fuzz_targets/type_mapping_fuzzer.rs`

```rust
fuzz_target!(|data: &[u8]| {
    if let Ok(type_str) = std::str::from_utf8(data) {
        // Nunca debería entrar en pánico
        let _ = SqliteConnector::map_type(type_str);
        let _ = MysqlConnector::map_type(type_str);
        let _ = PostgresConnector::map_type(type_str);
    }
});
```

---

### 7. Pruebas de Regresión (Prioridad: P1)

**Archivo:** `tests/regression/known_issues_test.rs`

```rust
#[test]
fn test_issue_001_decimal_precision_lost() {
    // Antes del fix: DECIMAL(20,4) se convertía en DECIMAL(10,2)
    // Después del fix: precisión/escala preservadas
    
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
    // Todos los conectores deberían usar el mismo formato de metadata
    let sqlite_fk = extract_fk_from_sqlite();
    let mysql_fk = extract_fk_from_mysql();
    let postgres_fk = extract_fk_from_postgres();
    
    // Las claves de metadata deberían coincidir
    assert_eq!(
        sqlite_fk.metadata.keys().collect::<Vec<_>>(),
        mysql_fk.metadata.keys().collect::<Vec<_>>()
    );
}
```

---

### 8. Pruebas de Rendimiento (Prioridad: P3)

**Archivo:** `benches/connector_benchmarks.rs`

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_sqlite_extraction(c: &mut Criterion) {
    let db = create_large_test_db(); // 100 tablas, 1000 columnas
    
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

## Datos de Prueba y Fixtures

### Conjuntos de Datos Reproducibles Mínimos

**Archivo:** `tests/fixtures/databases/sqlite_test.sql`

```sql
-- Base de datos SQLite mínima para pruebas
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

**Archivo:** `tests/fixtures/files/schema.csv`

```csv
table_name,column_name,data_type,nullable,primary_key
users,id,INTEGER,false,true
users,username,TEXT,false,false
users,email,TEXT,true,false
posts,id,INTEGER,false,true
posts,user_id,INTEGER,false,false
posts,title,TEXT,false,false
```

**Archivo:** `tests/fixtures/mocks/http_server.rs`

```rust
pub struct MockHttpServer {
    // Implementación para probar adaptador remoto
    // sin solicitudes de red reales
}

impl MockHttpServer {
    pub fn new() -> Self {
        // Crear servidor mock en localhost puerto aleatorio
    }
    
    pub fn mock_file(&self, path: &str, content: &[u8]) {
        // Configurar respuesta para ruta
    }
    
    pub fn mock_google_sheets(&self, sheet_id: &str, csv_content: &str) {
        // Simular endpoint de exportación de Google Sheets
    }
    
    pub fn mock_large_file(&self, path: &str, size_mb: usize) {
        // Retornar archivo que excede límite de tamaño
    }
    
    pub fn mock_error(&self, path: &str, status: u16) {
        // Retornar error HTTP
    }
}
```

---

## Automatización CI/CD

### Workflow de GitHub Actions

**Archivo:** `.github/workflows/comprehensive-tests.yml`

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
      
      postgresql:
        image: postgresql:16
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
          TEST_POSTGRES_URL: postgresql://postgresql:test@localhost/test_db
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

## Criterios de Aceptación

### La Suite de Pruebas Debe Pasar

**Para que el PR sea fusionado:**
- ✅ Todas las pruebas de seguridad pasan (0 fallas)
- ✅ Todas las pruebas unitarias pasan (110+ pruebas)
- ✅ Todas las pruebas de integración pasan (30+ pruebas)
- ✅ Cobertura de código ≥ 85%
- ✅ Cobertura de rutas críticas ≥ 95%
- ✅ Sin nuevas vulnerabilidades de seguridad de `cargo audit`
- ✅ Fuzzing se ejecuta por 1 hora sin crashes
- ✅ Todas las verificaciones de análisis estático pasan

**Para despliegue a producción:**
- ✅ Todos los problemas críticos (C1-C5) resueltos
- ✅ Todos los problemas de alta prioridad (H1-H8) resueltos
- ✅ Revisión de seguridad completada
- ✅ Benchmarks de rendimiento dentro del rango aceptable
- ✅ Documentación actualizada

---

## Mantenimiento de Pruebas

### Monitoreo Continuo

- **Semanal:** Revisar fallas de pruebas y comportamiento inestable
- **Mensual:** Actualizar conjuntos de datos de prueba con nuevos casos límite
- **Trimestral:** Revisar reportes de cobertura e identificar vacíos
- **Por Versión:** Ejecutar suite completa de fuzzing (24 horas)

### Actualizaciones de Datos de Prueba

- Agregar nuevos casos de prueba cuando se encuentren bugs
- Actualizar fixtures cuando cambien los schemas de base de datos
- Expandir pruebas de propiedades con nuevas invariantes
- Agregar pruebas de regresión para cada corrección de bug

---

## Apéndice: Organización de Archivos de Prueba

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

**Fin del Plan de Pruebas**
