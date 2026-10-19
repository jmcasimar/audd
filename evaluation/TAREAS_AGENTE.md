# Tareas para el agente en el repositorio (antes de la corrida oficial)

Leer primero `evaluation/PROTOCOLO_EVALUACION.md`. Cada tarea se cierra con una prueba y un commit propio. **No** modificar `evaluation/manual/*/verdad.csv`.

## Núcleo (`crates/audd_compare`). Solo si el autor adopta la especificación simétrica (sección 5)

- [x] **E1.** En `engine.rs` (`compare_fields`), emitir `TypeIncompatible` también para `TypeCompatibility::WideningSafe` (severidad `Low`) y `RequiresCast` (severidad `Medium`). `Compatible` e `Identical` no generan conflicto.
- [x] **E2.** `NullabilityMismatch` cuando `field_a.nullable != field_b.nullable`, en cualquier sentido.
- [x] **E3.** UNIQUE: considerar también las claves `KeyType::Unique` de una sola columna en `entity.keys`.
- [ ] **E4 (opcional).** Comparar `Constraint::Check` y la pertenencia a la clave primaria (activa R-CONS-002 y 003).
- [x] **E5.** Pruebas `acceptance_rf3_*` actualizadas y `acceptance_rnf2_symmetry` (`crates/audd_resolution/tests/acceptance_symmetric_spec.rs`): comparar A→B y B→A da los mismos tipos de conflicto.
- [ ] Verificar que las 14 reglas de `catalog/resolutions.toml` sean alcanzables: una prueba por regla (`acceptance_rf6_rule_<id>`).

## Suite

- [x] **S4/S5.** `mutate.py`:
  - generar los pares sobre MySQL (`pair.json` con `source_a` y `source_b`);
  - eliminar el nivel 1;
  - operadores según la sección 7.3 del protocolo;
  - 5 bases × semillas 1000–1009.
- [x] **S7.** `requirements_coverage.py`:
  - agregar RF6;
  - contar como cubierto solo lo que **pasa** (`cargo test acceptance -- --format json` o el código de salida por prueba).
- [x] **S8.** `adoption.py`: IAC-1 acumulado con `https://api.npmjs.org/downloads/range/<fecha_publicación>:<hoy>/audd-node`.
- [ ] **S9.** `datasets.toml`: calcular los sha256 reales de `bases/<id>/schema.sql` (`setup_mysql.py` los imprime; los archivos actuales ya tienen hash real).
- [x] Marcar `evaluation/run.py` como obsoleto (sustituido por `evaluation/suite/`) para que nadie reporte sus cifras.
- [x] `Makefile`: objetivo `eval-suite` que ejecute la sección 10 del protocolo.
- [ ] `git tag v0.9.0-eval` en el commit que se evalúa (pedir confirmación al autor).

## Hallazgos durante la integración (2026-10-19)

- [x] **Emparejamiento muchos a muchos** (`audd_compare/src/matcher.rs`, `match_fields` y `match_entities`): cada par A–B que coincide se registra, sin asignación uno a uno ni prioridad del nombre exacto. Efectos medidos: FCI ≠ 0 en `growventa` contra sí misma (`Product` se empareja también con `Product_Product`), falsos positivos de nulabilidad cuando B tiene `to_date` y `ToDate`, y asimetría de exclusivos en SYM. **Resuelto**: asignación uno a uno voraz (exacto > normalizado > semántico > similitud, desempate por índice) en entidades y campos; pruebas en `audd_compare/tests/matching_acceptance_tests.rs`. Con northwind y growventa: FCI = 0 (antes 32 exclusivos falsos) y SYM_exclusives = 100 % (antes 0 %).
- [ ] **Colisión de normalización**: el motor solo la detecta entre campos *no emparejados* de lados distintos; la especificación (sección 5) la define como dos campos de B que se normalizan igual. Hoy el operador correspondiente del bloque B da siempre FN.
- [ ] **Sugerencias sin catálogo**: `analyze_cast_safety` (`audd_resolution/src/engine.rs`) compara por subcadena (`"string"` contiene `"int"`). No afecta la corrida oficial, que usa `--catalog`.
- [x] Pruebas de aceptación faltantes para PRM: RF4, RF5, RF6, RNF1, RNF3 y RNF4 (PRM: 40 % → 100 %). Para RF6 se implementaron `--catalog-optional` (K01), `origin` en las salidas (K03), `audd catalog learn` (K05) y la versión del catálogo en el reporte (K06).
- [ ] Catálogo frente a la tabla 6.4: faltan R-LEN-002, R-NULL-002 y R-NORM-002. R-CONS-002 y R-CONS-003 solo son alcanzables en el catálogo; el motor no emite esos conflictos (E4).

