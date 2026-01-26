# Reporte de Auditoría Técnica de AUDD

**Fecha:** 2026-01-26  
**Alcance:** Conectores de bases de datos (SQLite, PostgreSQL, MySQL/MariaDB, MongoDB, SQL Server, Firebird), Adaptadores de archivos (CSV/JSON/XML/SQL), Adaptador remoto (HTTP/HTTPS, Google Sheets), extensiones IR

---

## Resumen Ejecutivo

**Total de Problemas Identificados:** 20  
**Críticos:** 5 | **Altos:** 8 | **Medios:** 5 | **Bajos:** 2

### Hallazgos Críticos que Requieren Acción Inmediata

1. **Vulnerabilidades de SQL Injection** - Los conectores SQLite y MySQL utilizan interpolación de cadenas para nombres de tablas
2. **Fallos de Compilación** - El conector Firebird tiene referencias de tipo faltantes que impiden la compilación
3. **Configuración de Seguridad Incorrecta** - SQL Server deshabilita la validación de certificados TLS en código de producción
4. **Agotamiento de Recursos** - El adaptador remoto puede consumir memoria ilimitada al descargar archivos
5. **Vulnerabilidad SSRF** - El adaptador remoto acepta URLs no validadas incluyendo servicios internos

---

## Hallazgos Detallados

### 🔴 SEVERIDAD CRÍTICA

#### C1: SQL Injection en el Conector SQLite

**Archivo:** `crates/audd_adapters_db/src/sqlite.rs` líneas 100, 164, 234  
**Nivel de Riesgo:** Crítico  
**Puntuación CVSS:** 9.8 (Crítico)

**Problema:**
```rust
let query = format!("PRAGMA table_info('{}')", table_name); // INSEGURO
```

Los nombres de tablas y nombres de índices de los metadatos de la base de datos se interpolan directamente en consultas SQL sin validación. Un archivo de base de datos SQLite malicioso con nombre de tabla `'; DROP TABLE users--` podría ejecutar SQL arbitrario.

**Escenario de Ataque:**
1. El atacante crea una base de datos SQLite maliciosa con una tabla llamada `'; INSERT INTO admin_users VALUES('hacker','password')--`
2. AUDD carga esta base de datos para extracción de esquema
3. La inyección SQL se ejecuta, agregando al atacante a la tabla de administradores

**Remediación:**
```rust
// Agregar función de validación
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

// Usar antes de la construcción de la consulta
validate_sqlite_identifier(&table_name)?;
let query = format!("PRAGMA table_info('{}')", table_name);
```

**Prioridad:** P0 - Corregir inmediatamente antes de cualquier despliegue a producción

---

#### C2: SQL Injection en el Conector MySQL  

**Archivo:** `crates/audd_adapters_db/src/mysql.rs` líneas 85-90, 118-124, 231-239  
**Nivel de Riesgo:** Crítico  
**Puntuación CVSS:** 9.1 (Crítico)

**Problema:**
```rust
let query = format!(
    "SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES 
     WHERE TABLE_SCHEMA = '{}' AND TABLE_TYPE = 'BASE TABLE'",
    self.database_name  // INSEGURO
);
```

El nombre de la base de datos y los nombres de tablas se interpolan sin sanitización. A diferencia de SQLite donde los nombres provienen del archivo de base de datos, la cadena de conexión de MySQL es proporcionada por el usuario, lo que hace esto explotable.

**Escenario de Ataque:**
1. El usuario proporciona una cadena de conexión: `mysql://user:pass@host/'; DROP DATABASE prod; --`
2. El código ejecuta: `WHERE TABLE_SCHEMA = ''; DROP DATABASE prod; --'`
3. La base de datos de producción es eliminada

**Remediación:**
Utilice consultas parametrizadas para todas las consultas de INFORMATION_SCHEMA:
```rust
let mut stmt = conn.prepare(
    "SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES 
     WHERE TABLE_SCHEMA = ? AND TABLE_TYPE = 'BASE TABLE'
     ORDER BY TABLE_NAME"
)?;
let tables = stmt.query(&[&self.database_name])?;
```

**Prioridad:** P0 - Vulnerabilidad de seguridad crítica

---

#### C3: Fallos de Compilación en el Conector Firebird

**Archivo:** `crates/audd_adapters_db/src/firebird.rs`, `crates/audd_adapters_db/src/error.rs`  
**Nivel de Riesgo:** Crítico (Disponibilidad)  
**Impacto:** Fallo completo de compilación con la característica firebird

**Problemas:**

1. **Variante de Error Faltante:**
```rust
// error.rs - El enum DbError no tiene la variante QueryFailed
// Pero firebird.rs la usa 33 veces:
return Err(DbError::QueryFailed(format!("..."))) // ERROR DE COMPILACIÓN
```

2. **Importación Inválida:**
```rust
// firebird.rs:4
use rsfbclient::{Connection, ConnectionBuilder, FbError};
// ERROR: no existe `ConnectionBuilder` en el crate rsfbclient
```

**Remediación:**
```rust
// Opción 1: Agregar variante faltante a error.rs
pub enum DbError {
    // ... variantes existentes
    QueryFailed(String), // AGREGAR ESTO
}

// Opción 2: Reemplazar todos los QueryFailed con QueryError en firebird.rs
return Err(DbError::QueryError(format!("...")))

// Para ConnectionBuilder - revisar la documentación de rsfbclient para la API correcta
// Puede necesitar: use rsfbclient::ConnectionConfiguration;
```

**Prioridad:** P0 - El código no compila

---

#### C4: Consumo de Memoria Ilimitado en el Adaptador Remoto

**Archivo:** `crates/audd_adapters_file/src/remote_adapter.rs` líneas 162-165  
**Nivel de Riesgo:** Crítico (DoS)  
**Puntuación CVSS:** 7.5 (Alto)

**Problema:**
```rust
let mut reader = response.into_reader();
let mut buffer = Vec::new();
std::io::copy(&mut reader, &mut buffer) // SIN LÍMITE DE TAMAÑO
    .map_err(|e| AdapterError::IoError(e))?;
```

La respuesta HTTP completa se carga en memoria sin límite de tamaño. Un atacante puede proporcionar una URL a un archivo de 10GB causando un fallo por falta de memoria (OOM).

**Escenario de Ataque:**
1. El atacante proporciona una URL: `https://evil.com/10GB-file.csv`
2. AUDD comienza a descargar, asignando 10GB en Vec
3. El sistema se queda sin memoria, el proceso es terminado

**Remediación:**
```rust
const MAX_REMOTE_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100MB

let mut reader = response.into_reader().take(MAX_REMOTE_FILE_SIZE);
let mut buffer = Vec::with_capacity(4096); // Capacidad inicial razonable
let bytes_read = std::io::copy(&mut reader, &mut buffer)
    .map_err(|e| AdapterError::IoError(e))?;

if bytes_read >= MAX_REMOTE_FILE_SIZE {
    return Err(AdapterError::FileTooLarge(
        format!("Remote file exceeds {}MB limit", MAX_REMOTE_FILE_SIZE / 1024 / 1024)
    ));
}
```

**Prioridad:** P0 - Vulnerabilidad DoS

---

#### C5: Vulnerabilidad SSRF en el Adaptador Remoto

**Archivo:** `crates/audd_adapters_file/src/remote_adapter.rs` líneas 138-151  
**Nivel de Riesgo:** Crítico  
**Puntuación CVSS:** 9.1 (Crítico)

**Problema:**
```rust
fn fetch_content(&self) -> AdapterResult<Vec<u8>> {
    let url = if self.is_google_sheets_url() {
        self.convert_google_sheets_url()
    } else {
        self.url.clone() // SIN VALIDACIÓN
    };
    let response = ureq::get(&url).call() // OBTIENE CUALQUIER URL
```

Acepta cualquier URL sin validación. Puede utilizarse para escanear la red interna, acceder a endpoints de metadatos en la nube, o leer archivos locales.

**Escenarios de Ataque:**
1. **Escaneo de Puerto Interno:** `http://192.168.1.1:22`, `http://192.168.1.1:3306`, etc.
2. **Metadatos en la Nube:** `http://169.254.169.254/latest/meta-data/` (credenciales de AWS)
3. **Archivos Locales:** `file:///etc/passwd` (si ureq soporta file://)
4. **Servicios Internos:** `http://localhost:6379/` (Redis), `http://localhost:9200/` (Elasticsearch)

**Remediación:**
```rust
fn validate_url(url: &str) -> AdapterResult<()> {
    // Solo permitir HTTP/HTTPS
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(AdapterError::InvalidUrl(
            "Only HTTP and HTTPS URLs are supported".to_string()
        ));
    }

    // Analizar URL para obtener el host
    let url_parts: Vec<&str> = url.split('/').collect();
    if url_parts.len() < 3 {
        return Err(AdapterError::InvalidUrl("Malformed URL".to_string()));
    }
    
    let host_port = url_parts[2];
    let host = host_port.split(':').next().unwrap_or(host_port);
    
    // Bloquear localhost e IPs privadas
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
    
    // Bloquear rangos de IP privadas: 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16
    if host.starts_with("10.") || 
       host.starts_with("192.168.") ||
       host.starts_with("172.16.") || host.starts_with("172.17.") {
        return Err(AdapterError::InvalidUrl(
            "Access to private IP addresses is not allowed".to_string()
        ));
    }
    
    Ok(())
}

// Usar en fetch_content antes de ureq::get()
validate_url(&url)?;
let response = ureq::get(&url).call()
```

**Prioridad:** P0 - SSRF permite acceso a la red interna

---

### 🟠 SEVERIDAD ALTA

#### H1: Análisis de Puerto Inseguro con Fallo Silencioso

**Archivo:** `crates/audd_adapters_db/src/sqlserver.rs` línea 87  
**Nivel de Riesgo:** Alto

**Problema:**
```rust
let port = p.parse::<u16>().unwrap_or(1433) // FALLO SILENCIOSO
```

Un puerto inválido como "abc" se convierte silenciosamente en 1433. Podría conectarse al servidor incorrecto.

**Remediación:**
```rust
let port = p.parse::<u16>().map_err(|_| 
    DbError::InvalidConnectionString(format!("Invalid port number: {}", p))
)?;
```

---

#### H2: Validación de Certificados TLS Deshabilitada en SQL Server

**Archivo:** `crates/audd_adapters_db/src/sqlserver.rs` línea 98  
**Nivel de Riesgo:** Alto  
**Puntuación CVSS:** 6.8 (Medio)

**Problema:**
```rust
config.trust_cert(); // Para desarrollo/pruebas - ¡EN CÓDIGO DE PRODUCCIÓN!
```

Todas las conexiones de SQL Server son vulnerables a ataques MITM.

**Remediación:**
```rust
// Eliminar trust_cert() completamente
// Agregar documentación sobre requisitos de certificados
// Opcionalmente: agregar variable de entorno solo para modo desarrollo
if std::env::var("AUDD_DEV_MODE").is_ok() {
    eprintln!("WARNING: TLS certificate validation disabled (dev mode)");
    config.trust_cert();
}
```

---

#### H3: El Código de Producción Usa unwrap() Sin Manejo de Errores

**Archivos:** Múltiples ubicaciones en sqlserver.rs, postgres.rs  
**Nivel de Riesgo:** Alto

**Problema:**
```rust
let column_name: &str = row.get(0).unwrap_or(""); // CORRUPCIÓN DE DATOS SILENCIOSA
```

Un esquema de base de datos inesperado causa cadenas vacías en lugar de errores.

**Remediación:** Retornar errores apropiados en lugar de valores predeterminados

---

#### H4: Tamaño de Muestra de MongoDB No Validado

**Archivo:** `crates/audd_adapters_db/src/mongodb.rs` línea 63  
**Nivel de Riesgo:** Alto

**Problema:** Acepta sample_size=0 (sin inferencia) o sample_size=10000000 (OOM/lento)

**Remediación:**
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

#### H5: Fuga de Recursos en Runtime Asíncrono

**Archivo:** `crates/audd_adapters_db/src/factory.rs` líneas 89-95, 108-116, 129-135  
**Nivel de Riesgo:** Alto (Fuga de Memoria)

**Problema:** El runtime Tokio se crea pero nunca se limpia. Los pools de hilos pueden fugarse en procesos de larga duración.

**Remediación:**
```rust
let connector = runtime.block_on(async {
    PostgresConnector::new(&conn_details).await
})?;
runtime.shutdown_background(); // Limpieza explícita
Ok(Box::new(connector))
```

---

#### H6: Precisión Decimal Codificada Pierde Datos

**Archivo:** `crates/audd_adapters_db/src/sqlite.rs` líneas 472-475  
**Nivel de Riesgo:** Alto (Pérdida de Datos)

**Problema:** DECIMAL(20,4) se mapea a (10,2) codificado. Los valores se truncan sin error.

**Remediación:** Analizar la precisión de la cadena de tipo o almacenar el tipo original en los metadatos

---

#### H7: Errores de Conexión PostgreSQL Solo a stderr

**Archivo:** `crates/audd_adapters_db/src/postgres.rs` líneas 71-75  
**Nivel de Riesgo:** Alto

**Problema:** Los errores de conexión se imprimen en stderr, no se exponen al llamador. Las consultas fallan sin razón clara.

**Remediación:** Usar canal o Arc<Mutex<Option<Error>>> para exponer errores

---

#### H8: Definiciones SQL Almacenadas Sin Sanitización

**Archivos:** Todo el código IR y de adaptador que maneja definiciones  
**Nivel de Riesgo:** Alto

**Problema:** Las definiciones SQL de vistas/triggers/procedimientos de una base de datos maliciosa podrían contener payloads de inyección ejecutados cuando el esquema se aplica en otro lugar.

**Remediación:** Agregar advertencias en la documentación. Considerar validación/sanitización antes de la re-ejecución.

---

### 🟡 SEVERIDAD MEDIA

#### M1-M5: Ver reporte completo para problemas de severidad media
- Inconsistencia en el formato de metadatos de claves foráneas entre conectores
- Falta de cobertura de pruebas para rutas de error
- Inconsistencia en nomenclatura de variantes de error
- Conexión PostgreSQL generada sin manejo apropiado de errores
- Sin validación en varios parámetros de entrada

---

### ⚪ SEVERIDAD BAJA

#### L1-L2: Ver reporte completo para problemas de severidad baja
- Importaciones no utilizadas (advertencias)
- Números mágicos en el mapeo de tipos de Firebird
- Los ejemplos en comentarios de documentación usan unwrap()
- La extracción de ID de Google Sheets podría ser más robusta

---

## Recomendaciones por Prioridad

### Acciones Inmediatas (P0 - Esta Semana)

1. **Corregir errores de compilación de Firebird** - Agregar variante QueryFailed, corregir importaciones
2. **Implementar protección contra SQL injection** - Agregar validación de identificadores a SQLite/MySQL
3. **Agregar validación de URL al adaptador remoto** - Bloquear ataques SSRF
4. **Agregar límites de tamaño de archivo** - Prevenir OOM en adaptador remoto
5. **Eliminar TLS trust_cert()** - Habilitar validación apropiada de certificados

### Corto Plazo (P1 - Siguiente Sprint)

6. Corregir análisis de puerto para retornar errores en lugar de valor predeterminado silencioso
7. Reemplazar llamadas unwrap() con manejo apropiado de errores
8. Agregar validación de tamaño de muestra de MongoDB
9. Implementar limpieza de runtime para conectores asíncronos
10. Analizar y preservar precisión/escala DECIMAL

### Mediano Plazo (P2 - Este Trimestre)

11. Estandarizar formato de metadatos de claves foráneas
12. Agregar pruebas exhaustivas de rutas de error
13. Mejorar manejo de errores de conexión PostgreSQL
14. Agregar sanitización/validación de definiciones SQL

### Largo Plazo (P3 - En Curso)

15. Eliminar importaciones no utilizadas
16. Reemplazar números mágicos con constantes
17. Actualizar ejemplos de documentación para usar manejo apropiado de errores
18. Mejorar robustez del análisis de URL de Google Sheets

---

## Recomendaciones de Pruebas

### Pruebas de Ruta Crítica (Implementar Primero)

1. **Pruebas de SQL Injection:**
```rust
#[test]
fn test_sqlite_sql_injection_prevention() {
    let malicious_names = vec![
        "'; DROP TABLE users--",
        "table'; DELETE FROM admin; --",
        "test\0table",
    ];
    for name in malicious_names {
        // Verificar que la inyección es bloqueada
    }
}
```

2. **Pruebas de Prevención SSRF:**
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

3. **Pruebas de Límite de Recursos:**
```rust
#[test]
fn test_remote_adapter_size_limit() {
    // Servidor simulado que envía 200MB
    // Verificar que el adaptador rechaza con error FileTooLarge
}
```

4. **Pruebas de Validación de Puerto:**
```rust
#[test]
fn test_sqlserver_invalid_port_rejected() {
    let invalid = "sqlserver://user:pass@host:abc/db";
    assert!(SqlServerConnector::parse_connection_string(invalid).is_err());
}
```

### Cobertura de Pruebas Exhaustiva

Consulte `/docs/test_plan.md` para especificación completa de más de 150 pruebas incluyendo:
- Pruebas unitarias para todas las rutas de error
- Pruebas de integración con bases de datos simuladas
- Pruebas basadas en propiedades para mapeo de tipos
- Pruebas de fuzzing para analizadores
- Pruebas de regresión entre conectores
- Automatización CI/CD

---

## Resumen de Auditoría de Seguridad

**Vulnerabilidades Críticas:** 5  
**Todas Deben Corregirse Antes de Producción**

1. SQL Injection (2 instancias)
2. Vulnerabilidad SSRF
3. TLS deshabilitado
4. Consumo de recursos ilimitado

**Acciones Recomendadas:**
- Realizar revisión de seguridad de toda la validación de entrada
- Implementar fuzzing exhaustivo
- Agregar escaneo de seguridad al pipeline CI/CD
- Documentar supuestos y requisitos de seguridad

---

## Apéndice: Archivos Afectados

### Archivos que Requieren Cambios Inmediatos

- `crates/audd_adapters_db/src/sqlite.rs` - Corrección de SQL injection
- `crates/audd_adapters_db/src/mysql.rs` - Corrección de SQL injection  
- `crates/audd_adapters_db/src/firebird.rs` - Correcciones de compilación
- `crates/audd_adapters_db/src/error.rs` - Agregar variante faltante
- `crates/audd_adapters_db/src/sqlserver.rs` - TLS, análisis de puerto
- `crates/audd_adapters_file/src/remote_adapter.rs` - SSRF, límites de tamaño
- `crates/audd_adapters_db/src/factory.rs` - Limpieza de runtime

### Archivos que Requieren Cobertura de Pruebas

- Todos los archivos de conectores (pruebas de rutas de error)
- Todos los archivos de adaptadores (pruebas de entrada malformada)
- Archivos de esquema IR (pruebas de ida y vuelta)
- Archivos de factory (pruebas de integración)

---

**Fin del Reporte de Auditoría**
