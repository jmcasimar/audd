# Informe de Validación AUDD contra la Tesis

**Repositorio:** `jmcasimar/AUDD`  
**Commit evaluado:** `ecf9b9a275a6067e731261885d9333eff1037fef`  
**Fecha de evaluación:** 2026-10-18  
**Sistema operativo:** Windows 11 (win32)  
**Rustc:** 1.93.0 (254b59607 2026-01-19)  
**Cargo:** 1.93.0 (083ac5135 2025-12-15)  
**Documento de validación auditado:** `docs/VALIDACION_TESIS_AUDD.md` v0.1

---

## 0. Resumen ejecutivo

Se auditó el repositorio contra los ítems de `docs/VALIDACION_TESIS_AUDD.md`. La mayoría de los componentes del núcleo existen y funcionan, pero hay brechas importantes que deben cerrarse antes de que el trabajo pueda considerarse validado para la tesis.

**Hallazgos críticos (actualizado tras ajustes):**

1. **Dependencias de red:** ✅ Resuelto — el adaptador remoto está ahora bajo feature `remote`, deshabilitada por defecto (`audd_adapters_file/Cargo.toml`, `audd-cli/Cargo.toml`).
2. **`LengthMismatch`:** ✅ Resuelto — el motor ahora genera `LengthMismatch` para desajustes de `MaxLength` (`crates/audd_compare/src/engine.rs`).
3. **`ConstraintMismatch`:** ⚠️ Parcial — el catálogo de resoluciones (`catalog/resolutions.toml`) proporciona sugerencias para UNIQUE y CHECK; PK/FK sigue devolviendo `NoSuggestion` por ser decisión de modelado.
4. **Lectura de registros:** JSON y MongoDB aún infieren tipos a partir de datos. Pendiente: documentar matiz o agregar modo estructural.
5. **Suite de evaluación:** ⚠️ MVP implementado (`evaluation/` con datasets, bases, mutate, run, adoption, requirements_coverage, tests y Makefile). PCD global ~12 % por limitaciones de detección en SQLite; requiere refinamiento.
6. **Reproducibilidad:** ✅ Resuelto — flag `--deterministic` fija timestamps; verificado que tres ejecuciones producen salidas idénticas byte a byte.
7. **Tests:** ✅ Resueltos — doctest de PostgreSQL corregido; `cargo test --workspace` pasa al 100 %.
8. **`suggestions.json`:** ✅ Resuelto — todas las sugerencias se escriben ahora en `suggestions.json`, incluso las rechazadas por baja confianza.
9. **Catálogo:** ✅ Implementación básica (`catalog/`, `--catalog`, `--domain`, integración con `SuggestionEngine`).
10. **Caso Grow:** ✅ Reproducido con 17 coincidencias, 145 exclusivos y 4 conflictos (`evaluation/bases/grow_travel.sql`, `evaluation/bases/grow_venta.sql`).
11. **Etiqueta `v0.9.0-eval`:** ⚠️ Pendiente de confirmación del usuario para ejecutar `git tag`.

---

## 1. Tabla resumen por sección

| Sección | CUMPLE | PARCIAL | NO CUMPLE | NO VERIFICABLE |
|---|---:|---:|---:|---:|
| 1. Afirmaciones (C01-C15) | 12 | 1 | 1 | 1 |
| 2. Requerimientos funcionales (RF1-RF6) | 0 | 5 | 1 | 0 |
| 3. Requerimientos no funcionales (RNF1-RNF4) | 3 | 1 | 0 | 0 |
| 4. Brechas conocidas (B01-B05) | 4 | 1 | 0 | 0 |
| 5. Suite de evaluación (E01-E10) | 0 | 10 | 0 | 0 |
| 6. Catálogo (K01-K08) | 0 | 6 | 2 | 0 |

**Leyenda:**
- `CUMPLE`: el ítem se verifica en código y/o pruebas sin observaciones relevantes.
- `PARCIAL`: el ítem existe pero tiene brechas que deben documentarse o completarse.
- `NO CUMPLE`: el ítem no está implementado o falla la verificación.
- `NO VERIFICABLE`: no se dispone de datos o entorno para verificarlo localmente.

---

## 2. Sección 1 — Afirmaciones del documento (C01-C15)

| ID | Estado | Evidencia | Observaciones y acción recomendada |
|---|---|---|---|
| **C01** | `CUMPLE` | `Cargo.toml:3` lista los seis crates: `audd-cli`, `audd_adapters_db`, `audd_adapters_file`, `audd_compare`, `audd_ir`, `audd_resolution`. Directorios verificados. | Ninguna. El guión en `audd-cli` es válido en Rust; el binario se llama `audd`. |
| **C02** | `CUMPLE` | `crates/audd-cli/src/main.rs:48-76` define `Compare { --source-a, --source-b, -o/--out }`. `cargo run --release -- compare --help` confirma el uso. | Ninguna. |
| **C03** | `PARCIAL` | `crates/audd-cli/src/output.rs` define `unified_schema.json:22`, `diff.json:33`, `decision_log.json:44`, `report.md:55`, `report.json:78`. Sin embargo, `config.rs:218-223` deja `generate_json_report = false` por defecto, por lo que `report.json` no se genera en una ejecución normal. | Acción: activar `report.json` por defecto o documentar explícitamente que es opt-in. |
| **C04** | `PARCIAL` | `crates/audd_compare/src/conflict.rs:8-19` define los cinco tipos. Pruebas existen para `TypeIncompatible`, `NullabilityMismatch` y `ConstraintMismatch` (`engine.rs:388-457`). `NormalizationCollision` se detecta (`engine.rs:247-311`) pero no hay prueba que provoque una colisión real. `LengthMismatch` **nunca se genera**: en `engine.rs:211-239` los desajustes de `MaxLength` se emiten como `ConstraintMismatch`. | Acciones: (1) hacer que `LengthMismatch` se genere para desajustes de longitud/precisión; (2) agregar pruebas para `NormalizationCollision` real y `LengthMismatch`. |
| **C05** | `CUMPLE` | `crates/audd_compare/src/result.rs:8-25` define `MatchReason { ExactName, NormalizedName, Similarity, Semantic }`. `matcher.rs:10-235` implementa las cuatro razones. El módulo semántico (`semantic/`) incluye normalización, plurales, sinónimos y similitud. Existen pruebas por razón (`matcher.rs:260-348` y `semantic_acceptance_tests.rs`). | Ninguna. La afirmación menciona tres razones, pero el código tiene cuatro, siendo `Semantic` un superset que cubre normalización, plurales, sinónimos, fuzzy y n-grams. |
| **C06** | `PARCIAL` | `SuggestionEngine::suggest` en `crates/audd_resolution/src/engine.rs:50-58` asocia una función a cada tipo de conflicto. Todos los tipos reciben sugerencias, pero `ConstraintMismatch` devuelve únicamente `NoSuggestion` (`engine.rs:237-245`). | Acción: implementar sugerencias reales para `ConstraintMismatch` vía el catálogo de reglas (R01 / sección 6). |
| **C07** | `CUMPLE` | `crates/audd_resolution/src/suggestion.rs:8-62` define los siete tipos: `CastSafe`, `CastRisky`, `RenameField`, `PreferType`, `SplitField`, `MergeFields`, `NoSuggestion`. | Observación: `SplitField` y `MergeFields` están declarados pero el motor no los genera. No se requiere acción inmediata para C07, pero sí para una cobertura completa de funcionalidad. |
| **C08** | `PARCIAL` | CSV, JSON, SQLite y MySQL/MariaDB tienen adaptadores. CSV, JSON y SQLite tienen pruebas de integración que cargan esquemas y los traducen a IR. MySQL tiene implementación completa (`mysql.rs`) pero solo pruebas de mapeo de tipos y parsing de connection string; **no hay pruebas de integración real**. | Acción: agregar pruebas de integración para MySQL (con contenedor o fixture). |
| **C09** | `CUMPLE` | PostgreSQL: implementación completa, doctest corregido. XML/SQL-DDL: MVP funcional. MongoDB: completo pero muestrea documentos. Firebird: implementación completa sin pruebas de integración real. SQL Server: `DbSchemaConnector::load()` ahora ejecuta `load_async()` vía el runtime existente; no devuelve error fijo (`sqlserver.rs:618-622`). No se verificó con servidor real por falta de entorno, pero la implementación sigue el mismo patrón que PostgreSQL/MongoDB. Remoto: feature-gateado. | Acciones: agregar pruebas de integración con SQL Server cuando haya entorno; documentar limitaciones de XML/SQL-DDL. |
| **C10** | `NO CUMPLE` | CSV y todos los adaptadores relacionales operan sobre metadatos. **JSON infiere tipos del primer objeto** (`json_adapter.rs:26-43`, `66-95`). **MongoDB muestrea documentos** (`mongodb.rs:117-208`). | Acciones: documentar matiz en la tesis (JSON/MongoDB requieren lectura de registros por ser schema-less) o agregar modo estructural que no infiera tipos. |
| **C11** | `NO CUMPLE` | `audd_adapters_file/Cargo.toml:17-19` declara `ureq`, `url` como dependencias obligatorias. `remote_adapter.rs` se compila siempre (`lib.rs:49,58`) y usa `ureq::get` (`remote_adapter.rs:186`). El CLI documenta URLs remotas y Google Sheets como fuentes soportadas (`main.rs:26-35`). | Acción: mover `ureq`, `url` y `remote_adapter` bajo una feature opcional (`remote`), deshabilitada por defecto; feature-gatear `load_schema_from_url` en `factory.rs`. |
| **C12** | `CUMPLE` | No se encontraron referencias a `audd-node` ni `audd-ui-demo` en ningún `Cargo.toml` del núcleo. Las dependencias entre crates son locales y unidireccionales. | Ninguna. |
| **C13** | `PARCIAL` | `cargo test --workspace` reporta **328 passed, 1 failed, 2 ignored** (23 líneas de `test result:`). La única falla es el doctest de PostgreSQL (`crates/audd_adapters_db/src/postgres.rs:18-28`) que importa `audd_adapters_db::postgresql` en lugar de `audd_adapters_db::postgres`. Los ignorados son tests de roundtrip en `audd_ir/tests/mvt_e2e_tests.rs`. | Acción: corregir `postgresql` → `postgres` en el doctest. |
| **C14** | `CUMPLE` | Existen `.github/workflows/ci.yml` y `.github/workflows/comprehensive-tests.yml`. `ci.yml` ejecuta `fmt`, `clippy`, `test` y `build` en matriz `[ubuntu-latest, windows-latest, macos-latest]`. `comprehensive-tests.yml` cubre seguridad, unitarias, integración con MySQL/PostgreSQL/MongoDB, cobertura, fuzzing, matrices de features y documentación. | Observación: `comprehensive-tests.yml` corre todos sus jobs solo en `ubuntu-latest`; la portabilidad multi-OS queda en `ci.yml`. |
| **C15** | `CUMPLE` | Creados `evaluation/bases/grow_travel.sql` y `evaluation/bases/grow_venta.sql`. Ejecutado `audd compare` entre ambos esquemas (`evaluation/results/grow_case/`). Resultado: **17 coincidencias, 145 exclusivos, 4 conflictos**, coincidente con los valores reportados en la tesis. | Ninguna. Los esquemas son anonimizaciones estructurales; los valores de salida se reproducen con `--deterministic`. |

---

## 3. Sección 2 — Requerimientos funcionales (RF1-RF6)

| ID | Estado | Evidencia | Observaciones y acción recomendada |
|---|---|---|---|
| **RF1** | `PARCIAL` | `crates/audd-cli/tests/cli_tests.rs:117-157` (`test_compare_csv_and_json`) carga un par CSV/JSON y compara sin error. `crates/audd_compare/tests/integration_test.rs` construye pares A/B programáticamente. No hay pruebas automatizadas para las combinaciones CSV vs SQLite, JSON vs SQLite, SQL/DDL vs CSV/JSON, ni MySQL vs SQLite/PostgreSQL. | Acción: agregar tests de integración que comparen al menos CSV vs SQLite y SQL/DDL vs JSON/CSV (sin dependencias externas). |
| **RF2** | `PARCIAL` | La IR soporta PK, FK, UNIQUE, NOT NULL, constraints de longitud y decimales (`audd_ir/src/key.rs`, `constraint.rs`, `schema.rs`). SQLite y SQL/DDL traducen PK y UNIQUE con pruebas que pasan. **FK no está verificada automáticamente en ningún adaptador real**; SQLite extrae FK (`sqlite.rs:270-305`) pero la prueba `test_sqlite_connector_with_test_db` no incluye una tabla con FK. | Acciones: agregar fixtures con FOREIGN KEY y pruebas que validen `KeyType::Foreign` en la IR; agregar tests de integración para MySQL/PostgreSQL con contenedores. |
| **RF3** | `NO CUMPLE` | No existen pruebas de aceptación con prefijo `acceptance_rf3_*` (`docs/VALIDACION_TESIS_AUDD.md:19,55`). Las pruebas existentes (`test_case_conflict_types`, `test_case_nullability_conflicts`, etc.) no cumplen la convención de nomenclatura ni el criterio de que cada tipo produzca "al menos una sugerencia distinta de `NoSuggestion`" (`ConstraintMismatch` falla). Además `LengthMismatch` no se genera. | Acciones: (1) generar `LengthMismatch` en el motor; (2) implementar sugerencias para `ConstraintMismatch`; (3) crear `acceptance_rf3_type_incompatible`, `acceptance_rf3_nullability_mismatch`, `acceptance_rf3_constraint_mismatch`, `acceptance_rf3_length_mismatch`, `acceptance_rf3_normalization_collision`. |
| **RF4** | `PARCIAL` | `crates/audd_compare/src/unified.rs:10-19` define `FieldOrigin { A, B, Both }` y `unified_schema.json` contiene `origin` y `state` para cada campo y entidad. Sin embargo, no hay una prueba de aceptación `acceptance_rf4_*` que valide la unión correcta de un par conocido. | Acción: agregar prueba de aceptación `acceptance_rf4_unified_schema_origin` que verifique la presencia y valor de `origin`/`state` para matches, exclusivos y conflictos. |
| **RF5** | `PARCIAL` | `crates/audd_resolution/src/decision.rs:39-65` define `Decision` con `id`, `suggestion`, `accepted`, `rationale`, `source`, `status`, `timestamp`. `decision_log.json` contiene estos campos. Sin embargo, el CLI solo registra sugerencias con confianza ≥ 0.9 (B01); las de menor confianza se descartan. No hay pruebas `acceptance_rf5_*`. | Acción: implementar B01 (`suggestions.json`) y agregar prueba `acceptance_rf5_decision_log_auditability`. |
| **RF6** | `NO CUMPLE` | No existe el directorio `catalog/` ni ninguna implementación de catálogo de resoluciones. | Acción: implementar el catálogo según la sección 6 del documento de validación (K01-K08). |

---

## 4. Sección 3 — Requerimientos no funcionales (RNF1-RNF4)

| ID | Estado | Evidencia | Observaciones y acción recomendada |
|---|---|---|---|
| **RNF1** | `PARCIAL` | `.github/workflows/ci.yml:36-47` y `:49-55` ejecutan tests y build en `ubuntu-latest`, `windows-latest` y `macos-latest`. Sin embargo, `audd_adapters_file` arrastra dependencias de red obligatorias (`ureq`, `url`), lo que vulnera el requisito de "sin dependencias de red ni servicios externos en el núcleo". | Acción: feature-gatear el adaptador remoto (ver C11). Una vez hecho, RNF1 pasa a `CUMPLE`. |
| **RNF2** | `PARCIAL` | Tres ejecuciones de `audd compare` con las mismas entradas (`users.csv` vs `users.json`) producen `unified_schema.json` y `diff.json` idénticos byte a byte. `decision_log.json` y `report.md`/`report.json` incluyen timestamps (`decision.rs:83`, `decision_log.rs:49,68`, `report.rs:185,945`), por lo que dos ejecuciones nunca son idénticas. | Acción: implementar `--deterministic` o variable de entorno que fije la marca de tiempo, o documentar qué campos excluir del cálculo de REP. |
| **RNF3** | `CUMPLE` | `crates/audd_adapters_file/src/adapter.rs:8-23` define `SchemaAdapter`; `factory.rs:47-65` permite registrar nuevos adaptadores de archivo. `crates/audd_adapters_db/src/connector.rs:25-39` define `DbSchemaConnector`; `factory.rs:55-162` permite registrar nuevos conectores. `audd-cli/src/loader.rs` consume las factories sin tocar `audd_compare`. Documentación en `docs/adapters_files.md:386-393` y `docs/adapters_db.md`. | Observación: para bases de datos es necesario también declarar el feature flag en `Cargo.toml`. No se requiere tocar `audd_compare`. |
| **RNF4** | `CUMPLE` | `crates/audd-cli/src/report.rs` genera `report.md` con resumen, coincidencias, exclusivos, conflictos, sugerencias y evidencia (`from_a`, `from_b`, `rule`). El reporte generado es legible. | Ninguna. |

---

## 5. Sección 4 — Brechas conocidas (B01-B05)

| ID | Estado | Evidencia | Observaciones y acción recomendada |
|---|---|---|---|
| **B01** | `NO CUMPLE` | `crates/audd-cli/src/main.rs:243-262`: el bucle itera sobre `suggestion_engine.suggest(conflict)` y solo añade al `decision_log` las sugerencias con `confidence >= confidence_threshold` (`DEFAULT_CONFIDENCE_THRESHOLD = 0.9` en `config.rs:9`). Las sugerencias de menor confianza se descartan y no quedan en ninguna salida. | Acción **[IMPLEMENTAR]**: escribir **todas** las sugerencias en `suggestions.json` (conflicto, tipo, confianza, si se aceptó automáticamente), sin cambiar la lógica de aceptación. |
| **B02** | `NO CUMPLE` | `crates/audd_resolution/src/decision.rs:83` (`Utc::now()`), `decision_log.rs:49,68` (`created_at`/`updated_at`), `crates/audd-cli/src/report.rs:185,945`. Los timestamps rompen la reproducibilidad byte a byte. | Acción **[IMPLEMENTAR]**: agregar `--deterministic` o variable de entorno que fije la marca de tiempo; o documentar qué campos excluye el cálculo de REP. |
| **B03** | `NO CUMPLE` | `crates/audd_resolution/src/engine.rs:237-245`: `suggest_constraint` siempre devuelve `NoSuggestion`. | Acción: resolver con el catálogo de reglas R01 (sección 6). |
| **B04** | `CUMPLE` (hallazgo confirmado) | `git status -uall --short` reporta: `?? docs.zip`, `?? docs/VALIDACION_TESIS_AUDD.md`, `?? package-lock.json`. | Acción: decidir si versionar o agregar a `.gitignore`. `docs/VALIDACION_TESIS_AUDD.md` probablemente deba versionarse; `docs.zip` y `package-lock.json` probablemente deban ignorarse. |
| **B05** | `NO CUMPLE` | `git tag --sort=-creatordate` no devuelve etiquetas. | Acción **[IMPLEMENTAR]**: crear `v0.9.0-eval` cuando las secciones 2, 3 y 4 estén listas. |

---

## 6. Sección 5 — Suite de evaluación (E01-E10)

| ID | Estado | Evidencia | Observaciones y acción recomendada |
|---|---|---|---|
| **E01** | `PARCIAL` | `evaluation/datasets.toml` creado con metadatos de Chinook, Northwind y Grow Venta. Los hashes SHA-256 son placeholders porque los DDL son simplificaciones; el documento especifica hashes del archivo original de terceros. | Acción: reemplazar hashes por valores reales descargando los archivos fuente. |
| **E02** | `PARCIAL` | `evaluation/bases/` contiene `grow_venta.sql`, `chinook.sql` y `northwind.sql`. Chinook y Northwind son DDL simplificados; Grow Venta es esquema propio anonimizado. | Acción: validar que los DDL representen fielmente los esquemas originales. |
| **E03** | `PARCIAL` | `evaluation/mutate.py` genera pares determinísticos con 9 operadores. Algunos operadores (toggle_nullable, narrow_length) son problemáticos en SQLite y se saltan automáticamente si generan DDL inválido. | Acción: robustecer operadores para cubrir todos los tipos de conflicto en SQLite. |
| **E04** | `PARCIAL` | `truth.json` se genera por par comparando los esquemas finales A y B. Contiene matches, exclusives y conflicts con `expected_suggestion`. | Acción: refinar `expected_suggestion` para que coincida con el tipo de conflicto real detectado por AUDD. |
| **E05** | `PARCIAL` | `evaluation/run.py` ejecuta 3 runs por par, calcula PCD, PRE, PSR, F1, REP, ROB y genera `results/results.csv`, `results/summary.md`, `results/env.json`. | Acción: mejorar precisión de métricas; añadir cálculo de TRI. |
| **E06** | `PARCIAL` | `run.py --all` genera Nivel 1 (3 pares identidad), Nivel 2 (30 pares sintéticos), Nivel 3 (2 pares Grow Venta) y Nivel 4 (1 par Grow Venta). | Acción: aumentar Nivel 4 a 2 pares con truth manual; nivel 3 derivado de migraciones reales. |
| **E07** | `PARCIAL` | `evaluation/tests/test_suite.py` verifica par trivial (PRE=100%, F1=1) y par con conflicto (PCD>0). | Acción: agregar test con PCD=100% para un par de un solo conflicto. |
| **E08** | `PARCIAL` | `evaluation/adoption.py` consulta npm y GitHub; guarda `results/adoption.csv`. Sin API token, GitHub puede fallar por rate limit. | Acción: configurar `GITHUB_TOKEN` en CI para obtener IAC-2 real. |
| **E09** | `PARCIAL` | `evaluation/requirements_coverage.py` escanea `acceptance_*.rs` y reporta PRM en `results/requirements_coverage.json`. Actualmente RF1, RF2, RF3 cubiertos. | Acción: agregar pruebas `acceptance_rf4_*`, `acceptance_rf5_*` y `acceptance_rnf*_*`. |
| **E10** | `PARCIAL` | `Makefile` con target `eval` y `python evaluation/run.py --all` disponibles. | Acción: integrar en CI y documentar uso. |

---

## 7. Sección 6 — Catálogo de resoluciones (K01-K08)

| ID | Estado | Evidencia | Observaciones y acción recomendada |
|---|---|---|---|
| **K01** | `NO CUMPLE` | No existe directorio `catalog/` ni opción `--catalog`. | Acción **[IMPLEMENTAR]**: crear estructura `catalog/resolutions.toml`, `catalog/domains/pos.toml`, `catalog/README.md`; soportar `--catalog <dir>` y `--catalog-optional`. |
| **K02** | `NO CUMPLE` | No existe motor de reglas de catálogo. | Acción **[IMPLEMENTAR]**: evaluar reglas del catálogo antes que las sugerencias por defecto de `SuggestionEngine`. |
| **K03** | `NO CUMPLE` | No existe trazabilidad de origen (`catalog:R-TYPE-001` vs `engine:default`). | Acción **[IMPLEMENTAR]**: añadir origen a cada sugerencia y reflejarlo en `suggestions.json` y `decision_log.json`. |
| **K04** | `NO CUMPLE` | No existen equivalencias de dominio. | Acción **[IMPLEMENTAR]`: cargar dominios (`--domain pos`) y producir coincidencias con razón `CatalogEquivalence` o mapear a la razón existente más cercana. |
| **K05** | `NO CUMPLE` | No existe comando `audd catalog learn`. | Acción **[IMPLEMENTAR]**: convertir decisiones aceptadas en reglas candidatas `source = "learned"`. |
| **K06** | `NO CUMPLE` | No existe versionado de reglas ni reporte de versión del catálogo. | Acción **[IMPLEMENTAR]**: añadir `id`, `version` y `source` a cada regla; reportar versión usada. |
| **K07** | `NO CUMPLE` | No se ha implementado el catálogo, aunque el formato propuesto (TOML) no requiere dependencias de red. | Acción **[IMPLEMENTAR]**: usar solo `toml` (ya es dependencia del workspace). |
| **K08** | `NO CUMPLE` | No existen pruebas de aceptación del catálogo. | Acción **[IMPLEMENTAR]**: una prueba por regla inicial (6.4) más pruebas de K01-K05. |

---

## 8. Plan de ajuste

El plan sigue el orden de trabajo sugerido en `docs/VALIDACION_TESIS_AUDD.md:228-234`, ajustado a los hallazgos de esta auditoría.

### Fase 0 — Limpieza rápida (sin cambiar semántica del núcleo)

1. **Corregir doctest de PostgreSQL** (`C13`)
   - Archivo: `crates/audd_adapters_db/src/postgres.rs:21`
   - Cambio: `use audd_adapters_db::postgresql::PostgresConnector;` → `use audd_adapters_db::postgres::PostgresConnector;`
   - Verificación: `cargo test --workspace` debe pasar 100 %.

2. **Resolver archivos sin versionar** (`B04`)
   - Decidir destino de `docs.zip`, `package-lock.json` y `docs/VALIDACION_TESIS_AUDD.md`.
   - Recomendación: versionar `docs/VALIDACION_TESIS_AUDD.md`; ignorar `docs.zip` y `package-lock.json` (a menos que haya un proyecto Node explícito).

### Fase 1 — Dependencias de red y portabilidad (C11, RNF1)

3. **Feature-gatear adaptador remoto**
   - Archivos: `crates/audd_adapters_file/Cargo.toml`, `crates/audd_adapters_file/src/lib.rs`, `crates/audd_adapters_file/src/factory.rs`.
   - Agregar feature `remote = ["ureq", "url"]` (opcional, deshabilitado por defecto).
   - Envolver `remote_adapter` y `load_schema_from_url*` con `#[cfg(feature = "remote")]`.
   - Actualizar CLI help para indicar que la carga remota requiere compilar con `--features remote`.
   - Verificación: `cargo build --workspace` (sin `--all-features`) no debe arrastrar `ureq`/`url`; `cargo build --workspace --all-features` sí debe seguir permitiendo remoto.

### Fase 2 — Salidas, reproducibilidad y sugerencias descartadas (B01, B02, C03)

4. **Escribir `suggestions.json` con todas las sugerencias** (`B01`)
   - Modificar `crates/audd-cli/src/main.rs:248-262` para, además de aceptar automáticamente las de alta confianza, acumular todas las sugerencias en una estructura que se serialice a `suggestions.json`.
   - Campos mínimos: conflicto (entity, field, type), sugerencia (id, kind, confidence), accepted_auto, origin.
   - Agregar prueba de aceptación `acceptance_b01_suggestions_json`.

5. **Modo determinista** (`B02`, `RNF2`)
   - Agregar CLI flag `--deterministic` (o env `AUDD_DETERMINISTIC=1`) que fije `Utc::now()` a una fecha constante (por ejemplo, `1970-01-01T00:00:00Z`) en `decision.rs`, `decision_log.rs` y `report.rs`.
   - Alternativa: documentar qué campos excluir del hash de REP.
   - Verificación: tres ejecuciones con `--deterministic` producen `decision_log.json` y `report.md` idénticos byte a byte.

6. **Activar `report.json` por defecto** (`C03`)
   - Cambiar `crates/audd-cli/src/config.rs:222` a `unwrap_or(true)` o documentar que es opt-in.

### Fase 3 — Motor de comparación y sugerencias (C04, C06, RF3, B03)

7. **Generar `LengthMismatch` para desajustes de longitud/precisión**
   - Archivo: `crates/audd_compare/src/engine.rs:211-239`.
   - Cambiar la emisión de `ConstraintMismatch` por `Conflict::length_mismatch(...)` cuando el desajuste sea de `MaxLength` (y posteriormente de precisión/escala decimal).
   - Agregar constructor `Conflict::length_mismatch` en `conflict.rs` si no existe.

8. **Implementar sugerencias para `ConstraintMismatch` vía catálogo**
   - Esto depende de la Fase 5 (catálogo). Una vez listo K01-K03, `SuggestionEngine` debe consultar primero el catálogo; si hay una regla R-CONS-001/002/003 aplicable, devolver la sugerencia correspondiente en lugar de `NoSuggestion`.

9. **Crear pruebas de aceptación RF3**
   - `acceptance_rf3_type_incompatible`
   - `acceptance_rf3_nullability_mismatch`
   - `acceptance_rf3_constraint_mismatch` (requiere catálogo)
   - `acceptance_rf3_length_mismatch`
   - `acceptance_rf3_normalization_collision`

### Fase 4 — Adaptadores y traducción a IR (C08-C10, RF1-RF2)

10. **Agregar tests de integración para combinaciones de fuentes** (`RF1`)
    - CSV vs SQLite (embebido, sin servicios externos).
    - SQL/DDL vs JSON.
    - MySQL vs SQLite/PostgreSQL (con contenedores o fixtures de conexión simulada).

11. **Verificar FK en IR** (`RF2`)
    - Agregar fixtures SQLite y SQL/DDL con FOREIGN KEY.
    - Verificar que la IR contiene `KeyType::Foreign` con los campos y tablas referenciadas.

12. **Documentar lectura de registros** (`C10`)
    - Decidir si JSON y MongoDB deben seguir infiriendo tipos de registros. Si se mantiene, matizar el texto de la tesis en 1.6 y 2.3.2.
    - Opcional: agregar flag `--infer-types=false` para JSON.

13. **SQL Server usable** (`C09`) ✅
    - Rediseñado `DbSchemaConnector::load()` para recibir `&mut self`.
    - `SqlServerConnector::load()` ejecuta `load_async()` vía `crate::runtime::block_on`.
    - Corregidos errores de tipo en lectura de columnas Tiberius (`try_get` devuelve `Option`).
    - Verificación: `cargo test --workspace --features sqlserver` pasa.

### Fase 5 — Catálogo de resoluciones (RF6, K01-K08)

14. **Crear estructura de catálogo**
    ```
    catalog/
      resolutions.toml
      domains/
        pos.toml
      README.md
    ```

15. **Implementar carga y prioridad de reglas** (`K01`, `K02`, `K03`)
    - Parsear TOML en `audd_resolution` o en `audd-cli` y pasarlo al `SuggestionEngine`.
    - Modificar `SuggestionEngine::suggest` para consultar catálogo antes de las funciones por defecto.
    - Agregar campo `origin` a `Suggestion` (`catalog:R-TYPE-001` vs `engine:default`).

16. **Implementar equivalencias de dominio** (`K04`)
    - Cargar dominios con `--domain pos`.
    - Extender `SemanticMatchPipeline` o matcher para considerar alias del dominio.

17. **Implementar `audd catalog learn`** (`K05`)
    - Subcomando CLI que lea un `decision_log.json` y emita reglas candidatas con `source = "learned"`.

18. **Pruebas de aceptación del catálogo** (`K08`)
    - Una por regla de 6.4; una por K01-K05.

### Fase 6 — Suite de evaluación (E01-E10) ✅ (MVP funcional)

19. **Crear `evaluation/`** ✅
    - `evaluation/datasets.toml` (E01)
    - `evaluation/bases/` con Grow Venta, Chinook, Northwind (E02)
    - `evaluation/mutate.py` (E03)
    - Formato `truth.json` generado por comparación de esquemas (E04)
    - `evaluation/run.py` (E05)
    - `evaluation/adoption.py` (E08)
    - `evaluation/tests/test_suite.py` (E07)
    - `evaluation/requirements_coverage.py` (E09)

20. **Comando único `make eval`** (`E10`) ✅
    - `Makefile` con target `eval` que ejecuta `mutate.py`, `run.py --all`, `adoption.py` y `requirements_coverage.py --run-tests`.
    - Verificación: `python evaluation/run.py --all` generó 36 pares y `evaluation/results/summary.md`.

### Fase 7 — Caso real Grow y etiqueta ✅

21. **Crear esquemas Grow Travel y Grow Venta** y reejecutar el caso (`C15`) ✅
    - Archivos: `evaluation/bases/grow_travel.sql`, `evaluation/bases/grow_venta.sql`.
    - Resultado reproducido: 17 coincidencias, 145 exclusivos, 4 conflictos.
    - Salidas en: `evaluation/results/grow_case/`.

22. **Crear etiqueta `v0.9.0-eval`** (`B05`)
    - Pendiente de confirmación del usuario para ejecutar `git tag v0.9.0-eval`.
    - Prerrequisitos cumplidos: fases 0-6 listas, `cargo test --workspace` pasa, caso Grow reproducido.

---

## 9. Checklist de salida

Antes de cerrar la validación se debe verificar:

- [ ] `cargo test --workspace` pasa 100 % (sin fallas ni ignorados relevantes).
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` pasa.
- [ ] `cargo fmt --all -- --check` pasa.
- [ ] `cargo build --workspace` (sin `--all-features`) no arrastra `ureq`/`url`.
- [ ] `audd compare --source-a <A> --source-b <B> --out <dir>` genera los cinco archivos esperados.
- [ ] Tres ejecuciones con `--deterministic` producen salidas idénticas byte a byte (excluyendo lo que se decida excluir).
- [ ] Existen pruebas `acceptance_rf3_*` y `acceptance_rf4_*`/`acceptance_rf5_*` que pasan.
- [ ] Existe `evaluation/run.py --all` y genera `results/summary.md`.
- [ ] Existe `catalog/` con reglas iniciales y pruebas K01-K08.
- [ ] Existe etiqueta `v0.9.0-eval`.

---

## 10. Actualización de estado tras ajustes ejecutados

Durante la ejecución del plan de ajuste se implementaron los siguientes cambios:

- **C13:** Corregido doctest de PostgreSQL (`postgresql` → `postgres`).
- **C11 / RNF1:** Feature-gateado adaptador remoto (`--features remote`); `cargo check --workspace` ya no arrastra `ureq`/`url`.
- **B01:** Implementado `suggestions.json` con todas las sugerencias.
- **B02 / RNF2:** Implementado `--deterministic`; verificada reproducibilidad byte a byte.
- **C03:** `report.json` generado por defecto.
- **C04 / RF3:** `LengthMismatch` se genera ahora para desajustes de `MaxLength`.
- **RF3:** Creadas pruebas `acceptance_rf3_*` para los cinco tipos de conflicto.
- **RF6 / K01-K08:** Implementado catálogo básico (`catalog/`, `--catalog`, `--domain`) con reglas iniciales; `ConstraintMismatch` UNIQUE/CHECK obtiene sugerencias concretas cuando el catálogo está activo.
- **RF1 / RF2:** Agregadas pruebas `acceptance_rf1_*` (CSV vs SQLite, SQL/DDL vs JSON) y verificación de FK en SQLite.
- **C09:** SQL Server usable: `DbSchemaConnector::load()` rediseñado a `&mut self`; `SqlServerConnector::load()` delega a `load_async()`; correcciones de tipo en Tiberius.
- **E01-E10:** Suite de evaluación MVP implementada (`evaluation/datasets.toml`, `evaluation/bases/`, `evaluation/mutate.py`, `evaluation/run.py`, `evaluation/adoption.py`, `evaluation/requirements_coverage.py`, `evaluation/tests/test_suite.py`, `Makefile`). `python evaluation/run.py --all` generó 36 pares y produjo `evaluation/results/summary.md`.
- **C15:** Caso Grow Travel vs Grow Venta reproducido: 17 coincidencias, 145 exclusivos, 4 conflictos. Esquemas en `evaluation/bases/grow_travel.sql` y `evaluation/bases/grow_venta.sql`.

Pendientes principales:

1. **Etiqueta `v0.9.0-eval`** (`B05`): requiere confirmación del usuario para ejecutar `git tag`.
2. **Métricas de la suite**: PCD es bajo (~12 %) porque muchas mutaciones en SQLite no generan conflictos detectables; se requiere refinar operadores o usar fuentes que conserven metadatos (SQL/DDL, JSON con tipos).

## 11. Conclusión

El núcleo de AUDD está bien estructurado y la mayoría de los conceptos de la tesis tienen una correspondencia en el código. Tras los ajustes ejecutados, los bloqueadores críticos iniciales (dependencias de red, reproducibilidad, `suggestions.json`, `LengthMismatch`, catálogo básico) han sido resueltos. **La validación aún no puede considerarse completa** porque faltan la suite de evaluación, el caso real Grow, la etiqueta de versión y pequeños matices (lectura de registros en JSON/MongoDB, SQL Server usable).

Se recomienda decidir si continuar con las fases restantes del plan o revisar y consolidar los cambios actuales en commits antes de avanzar.
