# Validación de AUDD contra el documento recepcional

**Propósito:** lista de verificación ejecutable por un agente de código dentro del repositorio `jmcasimar/AUDD`. Cada ítem corresponde a una afirmación, un requerimiento o un indicador de la tesis («Algoritmo de Unificación Dinámica de Datos», MDDI, UdeG). El agente debe comprobar cada ítem contra el código, las pruebas y la ejecución real, y producir evidencia.

**Versión:** 0.1, 18 de octubre de 2026. Referencias a la tesis: capítulo 1 (1.6 Alcance), capítulo 4 (4.3 Diseño, 4.3.5 Requerimientos) y capítulo 5 (5.3 Indicadores, 5.6 Validación).

---

## 0. Instrucciones para el agente

1. **No cambies la semántica del núcleo** (`audd_ir`, `audd_compare`, `audd_resolution`) para que un ítem «pase». Si un ítem falla, repórtalo como falla. Solo implementa lo que esta lista marca como **[IMPLEMENTAR]**.
2. **Preserva la portabilidad.** No agregues dependencias de red, servicios externos ni interfaces gráficas al núcleo. Todo lo nuevo debe funcionar sin conexión y en Windows, macOS y Linux.
3. **Evidencia por ítem.** Para cada ID, registra:
   - estado: `CUMPLE`, `PARCIAL`, `NO CUMPLE` o `NO VERIFICABLE`;
   - evidencia: `archivo:línea`, nombre de la prueba o comando ejecutado con su salida resumida;
   - en caso de falla, la acción recomendada.
4. **Entregable:** `evaluation/VALIDATION_REPORT.md`, con una tabla resumen por sección y el detalle por ítem. Incluye al inicio el hash del commit evaluado, la fecha, el sistema operativo y la versión de `rustc`.
5. **Trabaja en una rama** (`validation/tesis`) y haz commits pequeños con mensajes que citen el ID (por ejemplo, `RF3: prueba de aceptación de clasificación de conflictos`).
6. Las pruebas nuevas de aceptación deben nombrarse `acceptance_<id>_<descripcion>` (por ejemplo, `acceptance_rf3_type_incompatible`), para poder contarlas por requerimiento.

---

## 1. Afirmaciones del documento que deben ser verdaderas en el código

Si alguna resulta falsa, se corrige el **documento**, no el código, salvo indicación contraria.

| ID | Afirmación en la tesis | Dónde la dice | Cómo verificar |
|---|---|---|---|
| C01 | El núcleo está organizado en los módulos `audd_ir`, `audd_compare`, `audd_resolution`, `audd_adapters_file`, `audd_adapters_db` y `audd-cli` | 4.3.3 | Revisar `Cargo.toml` del workspace y `crates/` |
| C02 | El CLI se usa como `audd compare --source-a <A> --source-b <B> --out <dir>` | 5.6.1 | Ejecutar `audd compare --help` y una comparación con `fixtures/adapters` |
| C03 | Las salidas son `unified_schema.json`, `diff.json`, `decision_log.json`, `report.md` y `report.json` | 4.3.1 (Tabla 12) | Ejecutar C02 y listar el directorio de salida; documentar la estructura de cada archivo |
| C04 | Se detectan exactamente cinco tipos de conflicto: TypeIncompatible, NullabilityMismatch, ConstraintMismatch, LengthMismatch y NormalizationCollision | 4.3.5 (RF3), 5.3 | `crates/audd_compare/src/conflict.rs` y una prueba que provoque cada tipo |
| C05 | Las coincidencias se identifican por nombre exacto, nombre normalizado o similitud (con normalización, plurales y sinónimos) | 4.3.1, 4.3.3 | `audd_compare/src/result.rs` (`MatchReason`) y `semantic/`; una prueba por razón |
| C06 | A cada conflicto se le asocia al menos una sugerencia de resolución | 4.3.5 (RF3) | Revisar `SuggestionEngine::suggest`. **Conocido:** ConstraintMismatch devuelve `NoSuggestion`. Reportar `PARCIAL` y ver R01 |
| C07 | Tipos de sugerencia: CastSafe, CastRisky, RenameField, PreferType, SplitField, MergeFields y NoSuggestion | 5.6.1 (Tabla 21) | `audd_resolution/src/suggestion.rs` |
| C08 | Fuentes disponibles para la evaluación: CSV, JSON, SQLite y MySQL/MariaDB | 1.6, 4.3.4 (Tabla 13) | Una prueba de integración por fuente que cargue un esquema y lo traduzca a IR |
| C09 | Estado de PostgreSQL, XML, SQL/DDL, MongoDB, SQL Server, Firebird y el adaptador remoto | 4.3.4 (Tabla 13) | Existen archivos en `audd_adapters_db/src` y `audd_adapters_file/src`. Clasificar cada uno como funcional (con prueba que pase), parcial o no funcional |
| C10 | AUDD trabaja sobre la estructura y no lee los registros de las fuentes | 1.6, 2.3.2, 3.4.4 | Revisar cada adaptador: ¿consulta solo metadatos (por ejemplo, `sqlite_master`, `information_schema`) o lee filas para inferir tipos? El adaptador CSV, en particular, ¿infiere tipos a partir de los datos? Si alguno lee registros, reportarlo con exactitud para matizar el texto |
| C11 | El núcleo se ejecuta localmente, sin servicios externos | 3.4.4, RNF1 | Revisar dependencias de red; el adaptador remoto debe ser opcional y estar fuera del flujo por defecto |
| C12 | audd-node y audd-ui-demo usan el núcleo sin modificarlo | 4.3.3, 5.2 | Revisar que el núcleo no dependa de esos repositorios (dependencia solo en un sentido) |
| C13 | El núcleo cuenta con N pruebas automatizadas | 5.4 | Ejecutar `cargo test --workspace` y reportar el número de pruebas que pasan, fallan e ignoradas (hoy hay 305 anotaciones `#[test]`) |
| C14 | Existe integración continua | 4.2 | Revisar `.github/workflows/ci.yml` y `comprehensive-tests.yml`; ¿pasan en el último commit? |
| C15 | El primer caso real (Grow Travel en MySQL contra Grow Venta en SQLite) dio 17 coincidencias, 145 exclusivos y 4 conflictos | 5.4 | Si se dispone de ambos esquemas, reejecutar y comparar. Si cambia, reportar los valores actuales |

---

## 2. Requerimientos funcionales (capítulo 4.3.5)

Cada requerimiento necesita al menos una prueba de aceptación `acceptance_rfX_*` que pase.

| ID | Requerimiento | Criterio de aceptación | Estado esperado |
|---|---|---|---|
| RF1 | Cargar dos fuentes en los formatos disponibles | Para cada formato de C08, un par A/B se carga y se compara sin error | Verificar |
| RF2 | Traducir entidades, atributos, tipos, claves y restricciones a la IR | Para un esquema de prueba con PK, FK, UNIQUE, NOT NULL, longitudes y decimales, la IR (`ir.json`) contiene todos esos elementos con el tipo canónico correcto | Verificar |
| RF3 | Detectar y clasificar los cinco tipos de conflicto y asociar sugerencias | Un par sintético por tipo produce exactamente ese conflicto, con entidad y campo correctos y al menos una sugerencia distinta de `NoSuggestion` | Esperado `PARCIAL` por ConstraintMismatch; se resuelve con R01 |
| RF4 | Generar un esquema unificado con coincidencias y exclusivos, anotado | `unified_schema.json` contiene la unión correcta de un par conocido; cada elemento indica su origen (A, B o ambos) | Verificar |
| RF5 | Registrar ejecución y decisiones de forma auditable | `decision_log.json` registra cada decisión con su ID, la sugerencia, quién decidió, la justificación y la hora; el reporte resume la ejecución | Verificar; ver brecha B01 |
| RF6 | Catálogo reutilizable de mapeos y reglas | Ver sección 6 | `NO CUMPLE` hasta implementar R02 |

## 3. Requerimientos no funcionales

| ID | Requerimiento | Criterio de aceptación |
|---|---|---|
| RNF1 | Portabilidad | Compilación y pruebas exitosas en Linux, Windows y macOS (usar la matriz de CI; si no existe, agregarla **[IMPLEMENTAR]**). Sin dependencias propietarias en el núcleo |
| RNF2 | Reproducibilidad | Tres ejecuciones de `audd compare` con las mismas entradas producen `diff.json` y `unified_schema.json` idénticos byte a byte. Ver B02 para `decision_log.json` y `report.*` |
| RNF3 | Extensibilidad | Agregar un adaptador nuevo requiere solo implementar el trait del adaptador y registrarlo en la fábrica, sin tocar `audd_compare`. Documentar los pasos en `docs/` y verificarlos con un adaptador de prueba mínimo |
| RNF4 | Transparencia | `report.md` es legible: incluye resumen, coincidencias, exclusivos y conflictos con su sugerencia y su evidencia (`from_a`, `from_b`, `rule`) |

---

## 4. Brechas conocidas que afectan la evaluación

| ID | Brecha | Evidencia | Acción |
|---|---|---|---|
| B01 | El CLI solo escribe en `decision_log.json` las sugerencias con confianza ≥ 0.9, que se aceptan automáticamente. Las sugerencias de menor confianza se descartan y no quedan en ninguna salida | `audd-cli/src/main.rs`, bucle sobre `suggestion_engine.suggest(conflict)`; `DEFAULT_CONFIDENCE_THRESHOLD = 0.9` | **[IMPLEMENTAR]** Escribir todas las sugerencias en `suggestions.json` (conflicto, tipo, confianza, si se aceptó automáticamente), sin cambiar la lógica de aceptación. Es necesario para el indicador PSR |
| B02 | `decision_log.json` y `report.md` incluyen marcas de tiempo (`created_at`, `timestamp`), por lo que dos ejecuciones nunca son idénticas | `audd_resolution/src/decision_log.rs` y `decision.rs` (`Utc::now()`) | **[IMPLEMENTAR]** Agregar una opción `--deterministic` (o una variable de entorno) que fije la marca de tiempo, o documentar qué campos excluye el cálculo de REP |
| B03 | ConstraintMismatch no genera sugerencias | `audd_resolution/src/engine.rs`, `suggest_constraint` | Se resuelve con las reglas del catálogo R01 |
| B04 | Hay archivos sin versionar (`docs.zip`, `package-lock.json` en la raíz) | `git status` | Decidir si se versionan o se agregan a `.gitignore` |
| B05 | No hay una etiqueta de versión para la evaluación | `git tag` | **[IMPLEMENTAR]** Crear `v0.9.0-eval` cuando las secciones 2, 3 y 4 estén listas |

---

## 5. Suite de evaluación (capítulo 5.6)

Todo lo de esta sección es **[IMPLEMENTAR]**, en `evaluation/`, sin modificar el núcleo.

| ID | Componente | Criterio de aceptación |
|---|---|---|
| E01 | `evaluation/datasets.toml` y script de descarga | Cada fuente externa tiene URL fija, versión y SHA-256. Los datos de terceros no se versionan; solo su DDL extraído, con licencia y atribución |
| E02 | `evaluation/bases/` | DDL de al menos 3 esquemas base: Grow Venta (anonimizado, solo estructura), Chinook y Northwind en SQLite |
| E03 | `evaluation/mutate.py` | Aplica entre 3 y 6 operadores con semilla fija y produce `a.sqlite`, `b.sqlite` y `truth.json`. Operadores mínimos: cambio de estilo de nombre, plural o sinónimo, renombrado sin relación, tipo incompatible, ampliación de tipo, longitud, nulabilidad, restricción UNIQUE o CHECK, colisión de normalización, agregar o quitar campo, agregar tabla. La misma semilla genera exactamente los mismos archivos |
| E04 | Formato `truth.json` | `{pair_id, level, seed, matches[{a,b,reason}], exclusives[{side,entity,field}], conflicts[{entity,field,type,expected_suggestion}]}` |
| E05 | `evaluation/run.py` | Para cada par: 3 ejecuciones de `audd compare`, código de salida, tiempo y hash de las salidas; emparejamiento de conflictos por (entidad, campo, tipo) con nombres normalizados; cálculo de PCD, PRE, PSR, F1 de coincidencias, REP y ROB; resultados en `results/results.csv` (por par), `results/summary.md` (por nivel) y `results/env.json` |
| E06 | Niveles del corpus | Nivel 1: 2 pares de identidad entre motores. Nivel 2: 30 pares sintéticos. Nivel 3: ≥ 2 pares de versiones *n* contra *n+1* de Grow Venta, con verdad de referencia derivada de sus migraciones. Nivel 4: 2 pares de caso de aplicación con verdad de referencia manual |
| E07 | Pruebas de la propia suite | Un par trivial (A igual a B) da PCD indefinido, PRE = 100 % y F1 = 1. Un par con un solo conflicto conocido da PCD = 100 % |
| E08 | Indicadores de transferencia | `evaluation/adoption.py`: descargas de `audd-node` desde la API de descargas de npm e interacciones externas con `gh` (issues y PR de cuentas distintas del autor), guardadas con fecha en `results/adoption.csv` |
| E09 | PRM | `cargo test acceptance` y un script que cuente los requerimientos de las secciones 2 y 3 con todas sus pruebas aprobadas (RF6 cuenta solo si se cumple la sección 6) |
| E10 | Un solo comando | `make eval` o `python evaluation/run.py --all` ejecuta E03, E05, E08 y E09 y regenera los resultados |

**Metas que se reportan (Tabla 19 de la tesis):**

| Indicador | Meta |
|---|---|
| PCD | ≥ 70 % |
| PRE | ≥ 90 % |
| PSR | ≥ 50 % |
| F1 | Sin meta, línea base |
| TRI | ≥ 50 % (semiautomático) |
| REP | 100 % |
| ROB | ≥ 90 % |
| PRM | ≥ 80 % |
| IAC-1 | ≥ 20 descargas |
| IAC-2 | Se reporta sin meta |

---

## 6. Catálogo de resoluciones, versión básica (RF6 v0)

### 6.1 Objetivo y límites

Cumplir RF6 en una versión mínima, sin cambiar la arquitectura. El catálogo es un conjunto de **archivos versionados** que el núcleo lee al iniciar. Tiene dos partes:

1. **Reglas de resolución:** qué sugerencia proponer ante un conflicto, con su confianza y justificación.
2. **Equivalencias de dominio:** nombres de campos y entidades que deben considerarse la misma cosa en un dominio (por ejemplo, punto de venta). Amplía el diccionario de sinónimos existente.

Fuera de alcance en v0: interfaz gráfica, base de datos, aprendizaje automático y sincronización remota.

### 6.2 Ubicación y formato

```
catalog/
  resolutions.toml          # reglas de resolución por defecto
  domains/
    pos.toml                # equivalencias para punto de venta
  README.md                 # cómo agregar reglas y su prioridad
```

Formato propuesto de una regla:

```toml
[[rule]]
id = "R-TYPE-001"
conflict_type = "TypeIncompatible"
when = { from = "Int32", to = "Int64" }   # tipos canónicos de audd_ir
suggestion = "CastSafe"
params = { target = "Int64" }
confidence = 0.95
rationale = "Ampliación de entero sin pérdida de información"
source = "default"        # default | user | learned
version = 1
```

Formato propuesto de equivalencias:

```toml
[domain]
name = "pos"
[[equivalence]]
canonical = "precio"
aliases = ["price", "unit_price", "precio_unitario", "pvp"]
```

### 6.3 Comportamiento

| ID | Requisito del catálogo | Criterio de aceptación |
|---|---|---|
| K01 | Carga | `audd compare --catalog <dir>` carga el catálogo; sin la opción, se usa `catalog/` si existe. Un archivo con error de formato produce un mensaje claro y no detiene la comparación si se usa `--catalog-optional` |
| K02 | Prioridad | Las reglas del catálogo se evalúan antes que las sugerencias por defecto de `SuggestionEngine`; si ninguna aplica, se usa el comportamiento actual |
| K03 | Trazabilidad | Cada sugerencia indica su origen (`catalog:R-TYPE-001` o `engine:default`) en `suggestions.json` y en `decision_log.json` |
| K04 | Equivalencias | Los alias de un dominio activo (`--domain pos`) producen coincidencias con la razón `CatalogEquivalence` (o se mapean a la razón existente más cercana si no se quiere ampliar el enum) |
| K05 | Aprendizaje manual | `audd catalog learn --from <decision_log.json> --out <archivo>` convierte las decisiones aceptadas en reglas candidatas con `source = "learned"`, para revisión humana. No se agregan solas al catálogo |
| K06 | Versionado | Cada regla tiene `id` y `version`; el reporte indica qué versión del catálogo se usó |
| K07 | Portabilidad | Sin dependencias nuevas fuera de lectura de TOML; funciona sin conexión |
| K08 | Pruebas | Una prueba de aceptación por regla del catálogo inicial (6.4), más pruebas de K01 a K05 |

### 6.4 Contenido inicial del catálogo

Tipos canónicos tomados de `audd_ir/src/types.rs`; ajustar los nombres si difieren.

| ID | Conflicto | Condición | Sugerencia | Confianza | Justificación |
|---|---|---|---|---|---|
| R-TYPE-001 | TypeIncompatible | Int32 → Int64 | CastSafe a Int64 | 0.95 | Ampliación sin pérdida |
| R-TYPE-002 | TypeIncompatible | Int32/Int64 → Decimal | CastSafe a Decimal | 0.90 | Sin pérdida si la escala lo permite |
| R-TYPE-003 | TypeIncompatible | Float → Decimal | CastRisky a Decimal | 0.70 | Posible redondeo |
| R-TYPE-004 | TypeIncompatible | Numérico o Boolean → String/Text | CastSafe a String | 0.85 | La representación textual no pierde información |
| R-TYPE-005 | TypeIncompatible | String/Text → numérico o fecha | CastRisky al tipo destino | 0.60 | Requiere validar que todos los valores se puedan convertir |
| R-TYPE-006 | TypeIncompatible | Boolean ↔ Int | CastSafe (0/1) | 0.85 | Convención habitual |
| R-TYPE-007 | TypeIncompatible | Date → Timestamp | CastSafe a Timestamp | 0.90 | Se agrega la hora 00:00 |
| R-TYPE-008 | TypeIncompatible | Timestamp → Date | CastRisky a Date | 0.65 | Se pierde la hora |
| R-LEN-001 | LengthMismatch | Longitudes distintas en cadenas | PreferType con la longitud mayor | 0.95 | Evita truncar datos |
| R-LEN-002 | LengthMismatch | Precisión o escala distintas en decimales | PreferType con la mayor precisión y escala | 0.90 | Evita perder precisión |
| R-NULL-001 | NullabilityMismatch | NULL en un lado y NOT NULL en el otro | Unificar como NULL | 0.90 | Acepta los datos de ambas fuentes |
| R-NULL-002 | NullabilityMismatch | Se quiere conservar NOT NULL | Unificar como NOT NULL con valor por defecto | 0.50 | Requiere definir el valor por defecto |
| R-CONS-001 | ConstraintMismatch | UNIQUE en un solo lado | Conservar UNIQUE solo tras validar duplicados; si no, eliminarla | 0.50 | Hasta hoy devuelve `NoSuggestion` (B03) |
| R-CONS-002 | ConstraintMismatch | CHECK distinto | Conservar la condición más permisiva | 0.60 | Evita rechazar datos válidos de una fuente |
| R-CONS-003 | ConstraintMismatch | Clave primaria o foránea distinta | NoSuggestion con lista de verificación manual | — | Decisión de modelado |
| R-NORM-001 | NormalizationCollision | Dos nombres que se normalizan igual | RenameField con sufijo de origen (`_a` o `_b`) | 0.80 | Elimina la ambigüedad |
| R-NORM-002 | NormalizationCollision | Mismo significado confirmado | MergeFields | 0.50 | Requiere confirmación humana |

Equivalencias iniciales del dominio `pos`, para validar y ampliar con los esquemas de Grow Venta:

| Canónico | Alias |
|---|---|
| producto | product, item, articulo |
| precio | price, unit_price, precio_unitario, pvp |
| existencia | stock, inventario, qty_on_hand |
| cliente | customer, client |
| venta | sale, ticket, order |
| fecha | date, created_at, fecha_venta |
| codigo_barras | barcode, sku, upc |
| cantidad | quantity, qty |

---

## 7. Cambios que se reflejan en la tesis cuando se cumpla cada parte

| Si se cumple | Actualizar en la tesis |
|---|---|
| Sección 1 completa | Tablas 12 y 13, apartados 1.6, 4.3.3 y 5.4 con los datos verificados (formatos, número de pruebas, lectura de registros) |
| Secciones 2 y 3 | Indicador PRM (5.3) y Tabla 15 |
| Sección 4 (B01 y B02) | 5.5: documentar los ajustes como problemas derivados resueltos |
| Sección 5 | Tablas 22 y siguientes, 5.6.2 a 5.6.4 y capítulo 6 |
| Sección 6 | RF6 pasa de «diferido» a «versión básica»; Tabla 15 (estado del catálogo); 5.5 (el catálogo se retoma sobre la IR, cerrando el pivote); trabajo futuro (6.5): aprendizaje automático de reglas |

---

## 8. Orden de trabajo sugerido para el agente

1. Sección 1 (C01–C15) e informe inicial, sin cambiar código.
2. B01, B02 y B05 (requeridos por la suite).
3. Pruebas de aceptación RF1–RF5 y RNF1–RNF4.
4. Suite E01–E10 con un solo esquema base; después, el resto del corpus.
5. Catálogo K01–K08 con las reglas de 6.4.
6. Reejecutar la suite con el catálogo activo y reportar ambas versiones (sin catálogo y con catálogo) para mostrar su efecto en PSR.
7. `evaluation/VALIDATION_REPORT.md` final y etiqueta `v0.9.0-eval`.
