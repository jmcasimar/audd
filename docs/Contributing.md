# Contribuir a AUDD

**🌐 Idioma / Language:**  
📘 **Español** | 📗 [English](en/Contributing.md)

---

¡Gracias por tu interés en contribuir a AUDD! Este documento proporciona lineamientos y mejores prácticas para contribuir al proyecto.

## Tabla de Contenidos

- [Código de Conducta](#código-de-conducta)
- [Primeros Pasos](#primeros-pasos)
- [Configuración del Entorno de Desarrollo](#configuración-del-entorno-de-desarrollo)
- [Flujo de Trabajo de Desarrollo](#flujo-de-trabajo-de-desarrollo)
- [Ejecutar Tests](#ejecutar-tests)
- [Añadir Nuevos Adaptadores](#añadir-nuevos-adaptadores)
- [Estándares de Documentación](#estándares-de-documentación)
- [Proceso de Code Review](#proceso-de-code-review)
- [Lineamientos de Commits](#lineamientos-de-commits)
- [Pull Requests](#pull-requests)
- [Estilo de Código](#estilo-de-código)

---

## Código de Conducta

Este proyecto y todos los participantes están gobernados por el [Código de Conducta de AUDD](../CODE_OF_CONDUCT.md). Al participar, se espera que respetes este código. Por favor, reporta comportamiento inaceptable a través de GitHub Issues.

---

## Primeros Pasos

### 1. Fork y Clone del Repositorio

```bash
# 1. Hacer fork del repositorio en GitHub
# (Haz clic en el botón "Fork" en https://github.com/jmcasimar/AUDD)

# 2. Clonar tu fork
git clone https://github.com/TU_USUARIO/AUDD.git
cd AUDD

# 3. Añadir upstream remoto
git remote add upstream https://github.com/jmcasimar/AUDD.git

# 4. Verificar remotos
git remote -v
# origin    https://github.com/TU_USUARIO/AUDD.git (fetch)
# origin    https://github.com/TU_USUARIO/AUDD.git (push)
# upstream  https://github.com/jmcasimar/AUDD.git (fetch)
# upstream  https://github.com/jmcasimar/AUDD.git (push)
```

### 2. Crear una Rama de Feature

```bash
# Actualizar main
git checkout main
git pull upstream main

# Crear rama de feature
git checkout -b feature/mi-nueva-feature

# O para bugfixes
git checkout -b fix/descripcion-del-bug
```

---

## Configuración del Entorno de Desarrollo

### Prerequisitos

**Requeridos:**
- **Rust 1.70+** - Instalar con [rustup](https://rustup.rs/)
- **Cargo** - Incluido con Rust
- **Git** - Control de versiones

**Opcionales (para adaptadores de BD):**
- **SQLite** - Incluido por defecto
- **MySQL client libraries** - Para soporte MySQL
  ```bash
  # Ubuntu/Debian
  sudo apt-get install libmysqlclient-dev
  
  # macOS
  brew install mysql-client
  ```
- **PostgreSQL client libraries** - Para soporte PostgreSQL
  ```bash
  # Ubuntu/Debian
  sudo apt-get install libpq-dev
  
  # macOS
  brew install postgresql
  ```

### Instalación de Rust

```bash
# Instalar Rust usando rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Activar en la sesión actual
source $HOME/.cargo/env

# Verificar instalación
rustc --version
cargo --version
```

### Instalar Componentes Adicionales

```bash
# Rustfmt (formateador de código)
rustup component add rustfmt

# Clippy (linter)
rustup component add clippy

# Rust Analyzer (opcional, para IDEs)
rustup component add rust-analyzer
```

### Compilar el Proyecto

```bash
# Compilar en modo desarrollo
cargo build

# Compilar en modo release (optimizado)
cargo build --release

# El binario estará en:
# - Desarrollo: ./target/debug/audd
# - Release: ./target/release/audd
```

### Verificar la Configuración

```bash
# Ejecutar tests
cargo test

# Verificar formato
cargo fmt --all -- --check

# Ejecutar linter
cargo clippy --all-targets --all-features -- -D warnings

# Si todo pasa, tu entorno está listo ✅
```

---

## Flujo de Trabajo de Desarrollo

### Ciclo de Desarrollo Típico

```bash
# 1. Actualizar tu rama con los últimos cambios
git checkout main
git pull upstream main
git checkout feature/mi-feature
git rebase main

# 2. Hacer cambios en el código
# ... editar archivos ...

# 3. Ejecutar tests mientras desarrollas
cargo test

# 4. Formatear código
cargo fmt

# 5. Verificar con clippy
cargo clippy --all-targets --all-features

# 6. Commit de cambios
git add .
git commit -m "feat: Añadir nueva funcionalidad X"

# 7. Push a tu fork
git push origin feature/mi-feature

# 8. Crear Pull Request en GitHub
```

### Comandos Útiles Durante el Desarrollo

```bash
# Compilar y ejecutar en un comando
cargo run -- --help
cargo run -- inspect --source fixtures/adapters/users.csv

# Ver warnings detallados
cargo build --verbose

# Limpiar artifacts de compilación
cargo clean

# Actualizar dependencias
cargo update

# Verificar que el proyecto compila sin warnings
cargo build --all-targets --all-features 2>&1 | tee build.log
```

---

## Ejecutar Tests

AUDD tiene diferentes niveles de testing para asegurar la calidad del código.

### Tests Unitarios

```bash
# Ejecutar todos los tests
cargo test

# Tests de un crate específico
cargo test -p audd_ir
cargo test -p audd_compare
cargo test -p audd_adapters_file
cargo test -p audd_adapters_db
cargo test -p audd_resolution
cargo test -p audd-cli

# Test específico por nombre
cargo test test_csv_adapter
cargo test test_comparison_engine

# Ejecutar tests con output detallado
cargo test -- --nocapture

# Ejecutar tests con logging
RUST_LOG=debug cargo test -- --nocapture
```

### Tests de Integración

```bash
# Los tests de integración están en cada crate en tests/
# Ejecutar tests de integración de adapters
cargo test -p audd_adapters_file --test integration_test

# Tests de CLI
cargo test -p audd-cli --test cli_tests
cargo test -p audd-cli --test report_tests
```

### Tests de Cobertura (Opcional)

```bash
# Instalar tarpaulin (herramienta de cobertura)
cargo install cargo-tarpaulin

# Generar reporte de cobertura
cargo tarpaulin --all-features --workspace --timeout 120 --out Html

# Ver reporte en tarpaulin-report.html
```

### Ejecutar Tests en Paralelo

```bash
# Por defecto cargo ejecuta tests en paralelo
cargo test

# Ejecutar tests secuencialmente (útil para debugging)
cargo test -- --test-threads=1
```

### Tests Específicos por Categoría

```bash
# Solo tests que contengan "csv" en el nombre
cargo test csv

# Solo tests que contengan "compare"
cargo test compare

# Excluir tests lentos (si están marcados con #[ignore])
cargo test -- --ignored
```

### Benchmarks (Futuro)

```bash
# Cuando se implementen benchmarks
cargo bench
```

---

## Añadir Nuevos Adaptadores

AUDD está diseñado para ser extensible. Aquí está cómo añadir soporte para un nuevo formato o base de datos.

### Añadir un Adaptador de Archivo

**Ejemplo: Añadir soporte para formato YAML**

#### Paso 1: Crear el Archivo del Adaptador

```bash
# Navegar al crate de file adapters
cd crates/audd_adapters_file/src/

# Crear nuevo archivo para el adaptador
touch yaml_adapter.rs
```

#### Paso 2: Implementar el Trait `SchemaAdapter`

```rust
// crates/audd_adapters_file/src/yaml_adapter.rs

use audd_ir::{SourceSchema, EntitySchema, FieldSchema, CanonicalType};
use crate::adapter::SchemaAdapter;
use crate::error::{AdapterResult, AdapterError};
use std::path::Path;
use std::fs;

pub struct YamlAdapter;

impl SchemaAdapter for YamlAdapter {
    fn load(&self, path: &Path) -> AdapterResult<SourceSchema> {
        // Leer archivo
        let content = fs::read_to_string(path)
            .map_err(|e| AdapterError::IoError(e))?;
        
        // Parsear YAML (necesitarás añadir serde_yaml a dependencies)
        let yaml_value: serde_yaml::Value = serde_yaml::from_str(&content)
            .map_err(|e| AdapterError::ParseError(e.to_string()))?;
        
        // Convertir a IR
        let entities = self.parse_yaml_to_entities(&yaml_value)?;
        
        // Construir SourceSchema
        Ok(SourceSchema::builder()
            .source_name(path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown"))
            .source_type("yaml")
            .entities(entities)
            .build())
    }
}

impl YamlAdapter {
    fn parse_yaml_to_entities(&self, yaml: &serde_yaml::Value) 
        -> AdapterResult<Vec<EntitySchema>> {
        // Implementar lógica de parsing
        // Esto depende de la estructura del YAML
        todo!("Implementar parsing de YAML a EntitySchema")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_yaml_adapter_load() {
        let adapter = YamlAdapter;
        let path = Path::new("fixtures/test.yaml");
        // Añadir assertions
    }
}
```

#### Paso 3: Registrar en el Factory

```rust
// crates/audd_adapters_file/src/factory.rs

use crate::yaml_adapter::YamlAdapter;

impl AdapterFactory {
    pub fn from_extension(ext: &str) -> AdapterResult<Box<dyn SchemaAdapter>> {
        match ext.to_lowercase().as_str() {
            "csv" => Ok(Box::new(CsvAdapter)),
            "json" => Ok(Box::new(JsonAdapter)),
            "xml" => Ok(Box::new(XmlAdapter)),
            "sql" => Ok(Box::new(SqlAdapter)),
            "yaml" | "yml" => Ok(Box::new(YamlAdapter)), // ← Añadir esto
            _ => Err(AdapterError::UnsupportedFormat(ext.to_string())),
        }
    }
}
```

#### Paso 4: Exportar el Módulo

```rust
// crates/audd_adapters_file/src/lib.rs

pub mod yaml_adapter;
```

#### Paso 5: Añadir Dependencia (si es necesaria)

```toml
# crates/audd_adapters_file/Cargo.toml

[dependencies]
serde_yaml = "0.9"  # Añadir dependencia YAML
```

#### Paso 6: Crear Tests

```bash
# Crear archivo fixture de prueba
mkdir -p fixtures/adapters
cat > fixtures/adapters/test.yaml << EOF
users:
  - id: 1
    name: "Alice"
    email: "alice@example.com"
  - id: 2
    name: "Bob"
    email: "bob@example.com"
EOF
```

```rust
// crates/audd_adapters_file/tests/yaml_test.rs

#[test]
fn test_yaml_adapter_integration() {
    use audd_adapters_file::yaml_adapter::YamlAdapter;
    use audd_adapters_file::adapter::SchemaAdapter;
    use std::path::Path;
    
    let adapter = YamlAdapter;
    let path = Path::new("../../fixtures/adapters/test.yaml");
    
    let result = adapter.load(path);
    assert!(result.is_ok());
    
    let schema = result.unwrap();
    assert_eq!(schema.source_type, "yaml");
    assert!(!schema.entities.is_empty());
}
```

#### Paso 7: Actualizar Documentación

```markdown
# docs/adapters_files.md

## Formatos Soportados

| Formato | Extensión | Auto-detección | Inferencia de Tipos |
|---------|-----------|----------------|---------------------|
| CSV     | `.csv`    | ✓             | Básica              |
| JSON    | `.json`   | ✓             | ✓                   |
| XML     | `.xml`    | ✓             | Básica              |
| SQL DDL | `.sql`    | ✓             | ✓                   |
| YAML    | `.yaml`, `.yml` | ✓        | ✓                   |  ← Añadir

### YAML Adapter

El adaptador YAML soporta archivos YAML estructurados...
```

### Añadir un Adaptador de Base de Datos

El proceso es similar pero en el crate `audd_adapters_db`:

```bash
cd crates/audd_adapters_db/src/
touch oracle_adapter.rs  # Ejemplo
```

Implementar el trait `DatabaseAdapter`:

```rust
pub trait DatabaseAdapter {
    fn connect(&self, connection_string: &str) -> AdapterResult<()>;
    fn load_schema(&self) -> AdapterResult<SourceSchema>;
}
```

---

## Estándares de Documentación

### Documentación de Código

Todos los elementos públicos deben tener documentación:

```rust
/// Carga un esquema desde un archivo CSV.
///
/// # Argumentos
///
/// * `path` - Ruta al archivo CSV
///
/// # Retorna
///
/// Un `SourceSchema` representando la estructura del CSV
///
/// # Errores
///
/// Retorna un error si:
/// - El archivo no existe
/// - El archivo no puede ser parseado
/// - El CSV no tiene encabezados
///
/// # Ejemplos
///
/// ```
/// use audd_adapters_file::csv_adapter::CsvAdapter;
/// use std::path::Path;
///
/// let adapter = CsvAdapter;
/// let schema = adapter.load(Path::new("data.csv"))?;
/// ```
pub fn load(&self, path: &Path) -> AdapterResult<SourceSchema> {
    // implementación
}
```

### Documentación Markdown

- Mantener español como idioma base
- Proporcionar traducción al inglés en `docs/en/`
- Incluir header de idioma en cada archivo:

```markdown
**🌐 Idioma / Language:**  
📘 **Español** | 📗 [English](en/FILENAME.md)
```

### Estructura de Documentación

```
docs/
├── README.md                    # Índice general (español)
├── Getting-Started.md           # Guía de inicio (español)
├── FAQ.md                       # Preguntas frecuentes (español)
├── Usage-Examples.md            # Ejemplos de uso (español)
├── Contributing.md              # Guía de contribución (español)
├── Architecture.md              # Arquitectura (español)
├── CONFIG.md                    # Configuración (español)
└── en/                          # Traducciones al inglés
    ├── README.md
    ├── Getting-Started.md
    ├── FAQ.md
    ├── Usage-Examples.md
    ├── Contributing.md
    ├── Architecture.md
    └── CONFIG.md
```

### Actualizar CHANGELOG (cuando se implemente)

```markdown
# Changelog

## [Unreleased]

### Added
- Soporte para archivos YAML (#123)
- Nuevo comando `audd validate` (#124)

### Changed
- Mejorado rendimiento de comparación en 40% (#125)

### Fixed
- Corregido bug en detección de tipos NULL (#126)
```

---

## Proceso de Code Review

### Antes de Solicitar Review

**Checklist:**

- [ ] El código compila sin warnings: `cargo build --all-targets --all-features`
- [ ] Todos los tests pasan: `cargo test`
- [ ] Código formateado: `cargo fmt`
- [ ] Sin warnings de clippy: `cargo clippy --all-targets --all-features`
- [ ] Documentación actualizada (si aplica)
- [ ] Tests añadidos para nueva funcionalidad
- [ ] Commits siguen convenciones
- [ ] PR tiene descripción clara

### Durante el Review

1. **Responde a comentarios** de manera constructiva
2. **Haz commits adicionales** para cambios solicitados (no fuerces push)
3. **Marca conversaciones como resueltas** cuando hayas aplicado cambios
4. **Pide clarificación** si un comentario no está claro

### Después del Review

```bash
# Una vez aprobado, el PR será merged por un maintainer
# Actualiza tu fork después del merge
git checkout main
git pull upstream main
git push origin main
```

---

## Lineamientos de Commits

Usamos [Conventional Commits](https://www.conventionalcommits.org/) para mensajes de commit claros y consistentes.

### Formato

```
<tipo>(<scope>): <descripción>

[cuerpo opcional]

[footer opcional]
```

### Tipos

- `feat`: Nueva funcionalidad
- `fix`: Corrección de bug
- `docs`: Cambios en documentación
- `style`: Cambios de formato (sin cambios de código)
- `refactor`: Refactorización de código
- `perf`: Mejoras de rendimiento
- `test`: Añadir o modificar tests
- `chore`: Cambios en build, CI, o herramientas
- `revert`: Revertir un commit previo

### Scopes Comunes

- `cli`: Cambios en el CLI
- `ir`: Cambios en Intermediate Representation
- `adapters`: Cambios en adaptadores (file o db)
- `compare`: Cambios en motor de comparación
- `resolution`: Cambios en motor de resolución
- `docs`: Documentación
- `ci`: Configuración de CI/CD

### Ejemplos

```bash
# Nueva funcionalidad
git commit -m "feat(adapters): Añadir soporte para archivos YAML"

# Corrección de bug
git commit -m "fix(compare): Corregir detección de tipos NULL en comparación"

# Documentación
git commit -m "docs: Actualizar guía de contribución con sección de adapters"

# Refactorización
git commit -m "refactor(ir): Simplificar lógica de builder de SourceSchema"

# Tests
git commit -m "test(adapters): Añadir tests de integración para YamlAdapter"

# Con cuerpo y footer
git commit -m "feat(cli): Añadir comando validate

Añade nuevo comando para validar archivos de configuración.

Closes #123"
```

### Commits Atómicos

- Un commit = un cambio lógico
- Si haces múltiples cosas, separa en múltiples commits

```bash
# Mal ✗
git commit -m "feat: añadir YAML, corregir bug CSV, actualizar docs"

# Bien ✓
git commit -m "feat(adapters): Añadir soporte YAML"
git commit -m "fix(adapters): Corregir parsing de CSV con comillas"
git commit -m "docs(adapters): Actualizar lista de formatos soportados"
```

---

## Pull Requests

### Crear un Pull Request

1. **Push tu rama** a tu fork
   ```bash
   git push origin feature/mi-feature
   ```

2. **Ir a GitHub** y crear Pull Request

3. **Completar el template** del PR:

```markdown
## Descripción

Breve descripción de los cambios.

## Tipo de cambio

- [ ] Bug fix (cambio que corrige un issue)
- [ ] Nueva funcionalidad (cambio que añade funcionalidad)
- [ ] Breaking change (cambio que rompe compatibilidad)
- [ ] Documentación

## ¿Cómo se ha testeado?

Describir los tests realizados.

## Checklist

- [ ] Mi código sigue el estilo del proyecto
- [ ] He realizado una auto-revisión de mi código
- [ ] He comentado mi código en áreas difíciles de entender
- [ ] He actualizado la documentación
- [ ] Mis cambios no generan nuevos warnings
- [ ] He añadido tests que prueban que mi fix funciona o que mi feature funciona
- [ ] Tests unitarios nuevos y existentes pasan localmente
- [ ] He ejecutado `cargo fmt` y `cargo clippy`
```

### Tamaño del PR

- **Ideal**: < 400 líneas de cambio
- **Máximo recomendado**: < 1000 líneas
- Si es más grande, considera dividir en múltiples PRs

### Title del PR

Seguir el formato de conventional commits:

```
feat(adapters): Añadir soporte para archivos YAML
fix(compare): Corregir detección de conflictos en tipos NULL
docs: Actualizar guía de contribución
```

---

## Estilo de Código

### Rust Style Guide

Seguimos la [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) oficial.

### Configuración de Rustfmt

El proyecto usa esta configuración en `rustfmt.toml`:

```toml
edition = "2021"
max_width = 100
tab_spaces = 4
newline_style = "Unix"
use_small_heuristics = "Default"
```

### Aplicar Formato

```bash
# Formatear todos los archivos
cargo fmt

# Verificar sin modificar
cargo fmt --all -- --check
```

### Clippy Lints

```bash
# Ejecutar clippy
cargo clippy --all-targets --all-features -- -D warnings

# Esto falla si hay warnings, perfecto para CI
```

### Naming Conventions

```rust
// Tipos: PascalCase
struct SourceSchema { }
enum CanonicalType { }

// Funciones y variables: snake_case
fn load_schema() { }
let field_name = "id";

// Constantes: SCREAMING_SNAKE_CASE
const MAX_FIELDS: usize = 1000;

// Lifetimes: single lowercase letter
fn compare<'a>(schema_a: &'a Schema) { }
```

### Error Handling

```rust
// Preferir Result<T, E> sobre panic!
pub fn load(&self, path: &Path) -> AdapterResult<SourceSchema> {
    // No usar unwrap() en código de producción
    let content = fs::read_to_string(path)?; // ✓ Usar ?
    
    // En tests, unwrap() está bien
    #[cfg(test)]
    let schema = adapter.load(path).unwrap(); // ✓ Ok en tests
}
```

### Comentarios

```rust
// Comentarios en español o inglés, ambos son aceptables
// Preferir documentación sobre comentarios

// Mal ✗ - comentario obvio
// Incrementa contador
counter += 1;

// Bien ✓ - explica el "por qué"
// Usamos Jaro-Winkler porque maneja mejor nombres con diferentes convenciones
let similarity = jaro_winkler(&name_a, &name_b);
```

---

## CI/CD

El proyecto usa GitHub Actions para CI/CD. Los workflows se ejecutan automáticamente en:

- Push a `main` o `develop`
- Pull Requests a `main` o `develop`

### Workflows

```yaml
# .github/workflows/ci.yml

jobs:
  fmt:      # Verifica formato
  clippy:   # Ejecuta linter
  test:     # Ejecuta tests en Linux, Windows, macOS
  build:    # Verifica que compila en todas las plataformas
```

### Pasar CI Localmente

```bash
# Ejecutar todas las verificaciones de CI localmente
./scripts/check-ci.sh  # Si existe

# O manualmente:
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo build --release --all-features
```

---

## Preguntas y Soporte

### ¿Dónde Pedir Ayuda?

- **GitHub Discussions**: Para preguntas generales sobre contribuir
- **GitHub Issues**: Para reportar bugs o solicitar features
- **Pull Request comments**: Para preguntas sobre código específico

### Recursos Útiles

- [Rust Book](https://doc.rust-lang.org/book/) - Aprender Rust
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Ejemplos prácticos
- [Cargo Book](https://doc.rust-lang.org/cargo/) - Documentación de Cargo
- [AUDD Architecture](Architecture.md) - Arquitectura del proyecto

---

## Reconocimiento

¡Todos los contribuidores son valorados y reconocidos!

- Los contribuidores son listados automáticamente en GitHub
- Contribuciones significativas son mencionadas en release notes

---

**¡Gracias por contribuir a AUDD!** 🎉

Tu tiempo y esfuerzo ayudan a hacer que AUDD sea mejor para todos.

---

**Última actualización:** 2026-01-26
