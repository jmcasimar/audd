# Protocolo de evaluación de AUDD

Versión 1.0 · octubre de 2026 · Este documento se fija **antes** de ejecutar la evaluación. Cualquier cambio posterior se registra en la sección 12 con fecha y motivo.

---

## 1. Diseño general

La evaluación combina tres bloques. Cada indicador tiene una sola fuente oficial, definida de antemano, para evitar elegir después el resultado más favorable.

| Bloque | Qué es | Pares | Verdad de referencia | Indicadores |
|---|---|---|---|---|
| A. Matriz cruzada | Las 6 bases contra todas, en ambos sentidos, más cada base contra sí misma | 30 dirigidas + 6 de identidad | No requiere | ROB, REP, SYM, FCI, tiempo |
| B. Mutaciones controladas | Variantes generadas de una base con cambios conocidos | 50 (5 bases × 10 semillas) | Automática: registro de mutaciones validado contra la IR | **PCD, PRE, PSR, F1** (oficiales), ROB, REP |
| C. Pares reales con verdad manual | 3 obligatorios + 2 opcionales | 3–5 | Manual, ciega y sellada antes de ejecutar | **TRI** (oficial); PCD, PRE, PSR y F1 como evidencia en esquemas reales |

**Por qué PCD, PRE y PSR se toman del bloque B.** TRL 4 significa validación en entorno controlado. El bloque B controla exactamente qué conflictos existen, con 50 pares y una verdad generada sin intervención humana. El bloque C muestra cómo se comporta AUDD con esquemas reales, pero con pocos pares y una verdad construida por una persona; por eso se reporta aparte y no se promedia con el bloque B.

**Bases (6):** chinook, employees, northwind, unicenta, growtravel y growventa.

**Pares manuales:**

| Id | A (origen) | B (destino) | Motivo |
|---|---|---|---|
| M1 | growtravel | growventa | Caso ya documentado en 5.4 (17 / 145 / 4) |
| M2 | unicenta | growventa | Migración real entre dos puntos de venta: el caso de uso principal |
| M3 | northwind | growventa | Dominio comercial con productos, clientes y pedidos |
| M4 (opcional) | chinook | northwind | Dos sistemas públicos; permite que un tercero replique la verdad |
| M5 (opcional) | employees | northwind | Dominios casi ajenos: control de falsos positivos |

---

## 2. Estructura de carpetas

```
evaluation/
  PROTOCOLO_EVALUACION.md     este documento
  config/
    suite.toml                bases, pares manuales, opciones de AUDD
    snapshots.lock.json       hash de la IR de cada base (se genera la primera vez)
  bases/<id>/                 DDL de cada base (solo estructura) y su origen/licencia
  suite/
    common.py                 reglas de conteo compartidas
    cross.py                  bloque A
    mutations.py              bloque B
    validate_truth.py         valida la verdad del bloque B contra la IR
    manual.py                 bloque C (init, freeze, run, score, tri)
    score.py                  cálculo de VP, FP, FN, PCD, PRE, PSR, F1
    report.py                 reporte final y contraste con metas
  manual/
    _plantilla/               verdad.csv, tiempos.csv, adjudicacion.csv, INSTRUCCIONES.md
    M2_unicenta_growventa/    una carpeta por par manual
  generated/                  pares del bloque B (mutate.py)
  results/<fecha>_<commit>/   una carpeta por corrida completa (no se sobrescribe)
```

---

## 3. Preparación (una sola vez)

1. **Congelar las bases.**
   - Cargar las cinco bases MySQL en un servidor local MySQL 8 o MariaDB, una base de datos por id: `chinook`, `employees`, `northwind`, `unicentaopos`, `growtravel`.
   - Guardar en `bases/<id>/schema.sql` el DDL exacto con `mysqldump --no-data <db> > schema.sql`.
   - Grow Venta se usa en su forma nativa (`bases/growventa/growventa.db`, solo estructura).
2. **Credenciales por variable de entorno.**
   - PowerShell: `$env:AUDD_MYSQL="usuario:clave@127.0.0.1:3306"`.
   - Nunca se escriben en el repositorio.
3. **Aplicar las correcciones del núcleo** de la sección 5. Después:
   - `cargo test --workspace`;
   - `cargo build --release -p audd-cli`;
   - `git tag v0.9.0-eval`.
4. **Primera corrida de `cross.py`.** Genera las IR de las bases. Copia sus hashes a `config/snapshots.lock.json`. A partir de ahí, si una base cambia, la suite avisa.
5. **Verificar las licencias** de chinook, employees, northwind y unicenta en `config/suite.toml` (campos `origen`). Solo se usa la estructura.

---

## 4. Problemas encontrados en la suite actual (no reportar sus cifras)

La revisión de `evaluation/run.py`, `mutate.py` y `results/summary.md` (corrida del 18-oct-2026) encontró errores que invalidan sus cifras. **El PCD de 12 % y el PRE, PSR y ROB de 100 % no deben usarse en la tesis.**

| # | Problema | Efecto |
|---|---|---|
| S1 | `run.py` calcula «PRE» como reproducibilidad (salidas idénticas), no como VP/(VP+FP) | PRE = 100 % sin significado |
| S2 | «ROB» se calcula como % de archivos estables, no como % de pares sin error | ROB inflado |
| S3 | «PSR» cuenta cualquier sugerencia distinta de NoSuggestion, sin comparar con la esperada | PSR = 100 % sin significado |
| S4 | Los pares de «identidad» de `mutate.py` sí tienen mutaciones (p. ej. `nombre VARCHAR(200)` → `TEXT`, `existencia` → `stock`) | El nivel 1 no prueba identidad |
| S5 | Las mutaciones se cargan en SQLite, cuyo adaptador no conserva longitudes, convierte todo `INT` en Int64 y guarda UNIQUE como clave, no como restricción | Mutaciones invisibles para el núcleo cuentan como falsos negativos |
| S6 | La verdad etiqueta como conflicto cambios que la especificación del núcleo considera compatibles (INTEGER→BIGINT, VARCHAR→TEXT) | Falsos negativos que no son errores del núcleo |
| S7 | `requirements_coverage.py` no incluye RF6 y cuenta pruebas existentes, no aprobadas | PRM mal definido |
| S8 | `adoption.py` reporta descargas del último mes; la meta es acumulada | IAC-1 subestimado |
| S9 | `datasets.toml` tiene hashes sha256 de ejemplo, no calculados | Reproducibilidad de entradas sin verificar |

Con el puntaje nuevo (`score.py`) sobre los mismos 34 pares mutados, sin validar la verdad, el resultado fue VP = 2, FN = 56, FP = 0. Por tipo: nulabilidad 2 de 13, restricción 0 de 20 y tipo 0 de 25. La causa principal es S5 + S6 + la especificación de la sección 5, no necesariamente el motor de comparación.

---

## 5. Especificación de conflictos (decisión previa obligatoria)

La verdad de referencia debe construirse con una especificación escrita, **no** con la salida de AUDD. Hoy el código (`audd_compare/src/engine.rs` y `types.rs`) aplica estas reglas:

| Situación | Código actual | Consecuencia |
|---|---|---|
| Tipos con conversión segura (Int32→Int64, Date→DateTime, Int→Decimal) | No es conflicto | R-TYPE-001, 002 y 007 del catálogo nunca se activan |
| Tipos que requieren conversión con pérdida (Int64→Int32, DateTime→Date, Float↔Decimal, Boolean↔Int) | **No es conflicto** | R-TYPE-003, 006 y 008 nunca se activan; se pierde la advertencia más útil para una migración |
| Tipos incompatibles (String↔Int, etc.) | Conflicto | R-TYPE-004 y 005 se activan |
| String↔Text, DateTime↔Timestamp | Equivalentes | Sin conflicto (correcto) |
| Nulabilidad | Solo si A es NOT NULL y B es NULL | La diferencia inversa no se reporta; el resultado cambia al invertir A y B |
| UNIQUE | Solo como restricción de campo | Los UNIQUE que el adaptador guarda como clave (SQLite, índices) no se comparan |
| CHECK y clave primaria | No se comparan | R-CONS-002 y 003 nunca se activan |
| Longitud | Solo si ambos lados tienen MaxLength | Correcto, pero SQLite nunca lo reporta |

**Recomendación: especificación de unificación (simétrica).** AUDD produce un esquema unificado C a partir de A y B, no una migración en un solo sentido. Por eso, toda diferencia que obligue a decidir cómo queda C debe reportarse como conflicto, y la severidad distingue lo seguro de lo riesgoso. Esto vuelve alcanzables las 14 reglas del catálogo y hace que la inversión A↔B dé el mismo resultado, que es la hipótesis de simetría que se quiere validar.

Cambios en el núcleo (pequeños, dentro de `audd_compare`):

1. **E1.** Reportar `TypeIncompatible` también para `WideningSafe` (severidad baja) y `RequiresCast` (severidad media). `Compatible` (String/Text, DateTime/Timestamp) sigue sin conflicto.
2. **E2.** Reportar `NullabilityMismatch` cuando `nullable` difiera en cualquier sentido.
3. **E3.** Considerar UNIQUE también cuando el campo es una clave única de una sola columna en `entity.keys`.
4. **E4 (opcional).** Comparar CHECK y la pertenencia a la clave primaria (activa R-CONS-002 y 003).
5. **E5.** Pruebas de aceptación para cada cambio, incluida una de simetría: comparar A→B y B→A da los mismos tipos de conflicto.

Estos cambios modifican una decisión de diseño previa. Si no se aplican, la alternativa válida es evaluar contra la especificación actual (direccional) y declararla en la tesis; en ese caso la verdad del bloque B debe excluir las conversiones seguras y con pérdida, y el SYM esperado no es 100 %.

**Especificación usada para la verdad** (adoptar una y anotarla en la sección 12):

| Diferencia entre A y B (mismo campo) | ¿Conflicto? | Tipo | Sugerencia esperada |
|---|---|---|---|
| Int32 vs Int64 | Sí (baja) | TypeIncompatible | CastSafe (R-TYPE-001) |
| Int vs Decimal | Sí (baja) | TypeIncompatible | CastSafe (R-TYPE-002) |
| Float vs Decimal | Sí (media) | TypeIncompatible | CastRisky (R-TYPE-003) |
| Int vs String | Sí | TypeIncompatible | CastSafe hacia String (R-TYPE-004) |
| String vs Int | Sí | TypeIncompatible | CastRisky (R-TYPE-005) |
| Boolean vs Int | Sí (media) | TypeIncompatible | CastSafe (R-TYPE-006) |
| Date vs DateTime/Timestamp | Sí | TypeIncompatible | CastSafe (R-TYPE-007) o CastRisky (R-TYPE-008), según el sentido |
| String vs Text; DateTime vs Timestamp | No | — | — |
| MaxLength distinto (ambos definidos) | Sí | LengthMismatch | PreferType (R-LEN-001) |
| Nulabilidad distinta | Sí | NullabilityMismatch | PreferType (R-NULL-001) |
| UNIQUE en un solo lado | Sí | ConstraintMismatch | PreferType (R-CONS-001) |
| CHECK distinto | Sí (si se aplica E4) | ConstraintMismatch | PreferType (R-CONS-002) |
| Pertenencia a PK distinta | Sí (si se aplica E4) | ConstraintMismatch | NoSuggestion (R-CONS-003) |
| Dos campos de B que se normalizan igual | Sí | NormalizationCollision | RenameField (R-NORM-001) |
| Mismo nombre, distinto significado | Fuera de alcance | Semantic | Se registra; no cuenta para PCD |

---

## 6. Bloque A: matriz cruzada

```
python evaluation/suite/cross.py --out evaluation/results/<id>
```

- 36 comparaciones (6 de identidad + 30 dirigidas), 3 repeticiones cada una con `--deterministic`.
- **ROB** = % de comparaciones con código de salida 0 y JSON válidos.
- **REP** = % de comparaciones cuyas salidas (`diff.json`, `unified_schema.json`, `suggestions.json`, `decision_log.json`) tienen el mismo hash canónico en las 3 repeticiones.
- **FCI** = conflictos y exclusivos reportados al comparar una base consigo misma. Lo esperado es 0; cualquier valor distinto es un falso positivo seguro.
- **SYM** = % de los 15 pares no ordenados en que X→Y y Y→X producen:
  - el mismo número de coincidencias;
  - los exclusivos de A y B intercambiados;
  - los mismos tipos de conflicto.
  `symmetry.json` lista las asimetrías por tipo. Con la especificación actual se esperan asimetrías en nulabilidad y conversiones (sección 5); con la simétrica, cualquier asimetría es un defecto.
- **Tiempo:** mediana por comparación.

---

## 7. Bloque B: mutaciones controladas

**Generación.** Se usa `mutate.py`, con estas correcciones previas (tarea para el agente):

1. Generar las variantes sobre MySQL (cargar `a.sql` y `b.sql` en bases temporales `audd_m_<par>_a/b`) y escribir `pair.json` con `source_a` y `source_b`. Así se conservan longitudes, Int32/Int64 y UNIQUE.
2. Eliminar el «nivel 1» del mutador; la identidad se evalúa en el bloque A.
3. Ajustar los operadores a la tabla de la sección 5: cambiar tipo entre familias, ampliar o reducir longitud, invertir nulabilidad en columnas que no son PK, agregar o quitar UNIQUE, crear colisión de normalización, renombrar por sinónimo, renombrar sin relación, agregar o quitar campo o tabla.
4. Usar 5 bases (chinook, employees, northwind, unicenta, growtravel) × 10 semillas fijas (1000–1009), con 3 a 6 cambios por variante.

**Validación de la verdad.** `mutations.py` ejecuta `audd inspect` sobre A y B y aplica `validate_truth.py` a cada conflicto:

| Estado | Significado | Cuenta para PCD |
|---|---|---|
| `observable` | La diferencia existe en la IR | Sí |
| `not_observable` | El adaptador perdió la diferencia | No; se reporta como limitación del adaptador |
| `label_review` | Hay diferencia, pero de otro tipo | No; revisar la etiqueta a mano y regenerar |

La validación lee solo la IR de entrada y no usa el motor de comparación, por lo que no es circular: comprueba que la mutación llegó al núcleo, no que el núcleo la detecte.

```
python evaluation/mutate.py ...            # genera evaluation/generated/*
python evaluation/suite/mutations.py --pairs evaluation/generated --out evaluation/results/<id>
```

**Agregación micro.** Se suman VP, FP y FN de todos los pares y luego se divide. Se reporta además el PCD por tipo de conflicto.

**Dominio.** La corrida oficial se hace **sin** `--domain pos`. Después puede repetirse con el dominio activado para medir su aporte al F1 de M2 y M3, y reportarse como análisis adicional.

---

## 8. Bloque C: evaluación manual (3 a 5 pares)

### 8.1 Quién y con qué

- **Evaluador 1 (autor):** construye la verdad y mide la condición manual.
- **Evaluador 2 (desarrollador de Grow Venta, sin acceso previo a la verdad):** mide la condición asistida. Con dos personas se evita que quien hizo el análisis manual reutilice su memoria en la condición asistida.
- Si solo participa el autor: hacer primero la condición asistida en M3 y la manual en M2 (orden alternado), dejar al menos 7 días entre condiciones del mismo par y declarar el sesgo de aprendizaje.
- Material de la condición manual: solo el DDL de cada base (`schema.sql`) y las herramientas habituales (editor, DBeaver o similar). Nada producido por AUDD.

### 8.2 Pasos por par

1. `python evaluation/suite/manual.py init M2_unicenta_growventa`
2. **Delimitar el alcance.** Copiar los nombres exactos de las tablas del dominio central a `scope_a` y `scope_b` en `config/suite.toml`. En unicenta (41 tablas), por ejemplo: productos, categorías, clientes, tickets, líneas de ticket, existencias e impuestos. El alcance se fija antes de construir la verdad y no se modifica después.
3. **Construir la verdad (condición manual, cronometrada).**
   - Registrar la hora de inicio en `tiempos.csv` (`condicion = manual`).
   - Llenar `verdad.csv` con una fila por:
     - cada coincidencia, incluidas las de nombre distinto;
     - cada elemento exclusivo (una tabla completa = una sola fila con el campo vacío);
     - cada conflicto, con su tipo y sugerencia esperada según la tabla de la sección 5.
   - Registrar la hora de fin.
4. `python evaluation/suite/manual.py freeze M2_unicenta_growventa` valida el CSV, genera `truth.json` y lo sella con su hash.
5. **Hacer `git commit` inmediatamente.** El commit demuestra que la verdad existía antes de ejecutar AUDD.
6. `python evaluation/suite/manual.py run M2_unicenta_growventa` ejecuta AUDD 3 veces. Se niega si la verdad no está sellada.
7. **Condición asistida (cronometrada, evaluador 2).**
   - Parte del `report.md` de AUDD y produce el mismo documento de correspondencias.
   - Registrar inicio y fin en `tiempos.csv` (`condicion = asistido`) y en `segundos_audd` el tiempo de ejecución.
8. `python evaluation/suite/manual.py score M2_unicenta_growventa` calcula los indicadores y lista los FP y FN.
9. **Adjudicación.** Para cada FP y FN, decidir en `adjudicacion.csv`: `error_audd`, `error_verdad` o `ambiguo`.
   - Solo `error_verdad` permite corregir `verdad.csv`; se vuelve a ejecutar `freeze`, que guarda la nueva versión en el historial.
   - Se reportan el número de versiones de la verdad y el número de correcciones. Los casos `ambiguo` se mantienen como en la verdad original.
10. Después de todos los pares: `python evaluation/suite/manual.py tri`.

### 8.3 Regla de conteo (igual en todos los bloques)

- **Conflicto VP:** coinciden entidad, campo y tipo, con nombres normalizados (minúsculas, sin guiones bajos ni espacios). Se acepta el nombre del lado A o del B. Cada conflicto de la verdad se empareja una sola vez.
- **Coincidencia VP:** la clave del lado A coincide y, cuando la IR de B permite resolverlo, el campo de B también.
- **Exclusivo:** se compara a la granularidad que reporta AUDD (una tabla completa sin equivalente = un elemento). Por esta razón, los 145 exclusivos reportados en 5.4 deben verificarse; la reproducción de `results/grow_case` da 27 elementos exclusivos (22 de A y 5 de B). Hay que aclarar si 145 cuenta campos dentro de tablas exclusivas.

---

## 9. Indicadores: definición y fuente

| Indicador | Fórmula | Fuente oficial | Meta |
|---|---|---|---|
| PCD | VP / (VP + FN), solo tipos estructurales y conflictos `observable` | Bloque B (micro) | ≥ 70 % |
| PRE | VP / (VP + FP) | Bloque B (micro) | ≥ 90 % |
| PSR | VP con sugerencia del tipo esperado / VP con sugerencia esperada definida | Bloque B | ≥ 50 % |
| F1 | 2PR / (P + R) de las coincidencias | Bloque B | Línea base |
| TRI | (t manual − t asistido) / t manual, promedio de los pares con `tri = true` | Bloque C | ≥ 50 % |
| REP | % de comparaciones con salidas idénticas en 3 repeticiones | Bloques A y B (se reporta el menor) | 100 % |
| ROB | % de comparaciones sin error | Bloques A y B (se reporta el menor) | ≥ 90 % |
| SYM | % de pares no ordenados simétricos | Bloque A | Se reporta |
| FCI | Conflictos y exclusivos en comparaciones de identidad | Bloque A | 0 |
| PRM | % de RF1–RF6 y RNF1–RNF4 con prueba de aceptación **aprobada** | `cargo test acceptance` | ≥ 80 % |
| IAC-1 | Descargas acumuladas de audd-node en npm a la fecha de corte | API de npm (rango) | ≥ 20 |
| IAC-2 | Issues, PR y comentarios de cuentas externas | API de GitHub | Se reporta |

TRL 4 se declara si se cumplen PCD, PRE, REP, ROB y PRM (Tabla 18 de la tesis).

---

## 10. Ejecución completa

```
# 0. una sola vez: preparación (sección 3)
cargo build --release -p audd-cli
set RUN=evaluation/results/20261101_eval        (PowerShell: $RUN="...")

# 1. Bloque A
python evaluation/suite/cross.py --out %RUN%

# 2. Bloque B
python evaluation/mutate.py ... (10 semillas por base)
python evaluation/suite/mutations.py --pairs evaluation/generated --out %RUN%

# 3. Bloque C (por cada par; ver 8.2)
python evaluation/suite/manual.py init|freeze|run|score <par>
python evaluation/suite/manual.py tri

# 4. PRM e IAC
python evaluation/requirements_coverage.py --run-tests
python evaluation/adoption.py

# 5. Reporte
python evaluation/suite/report.py --run %RUN%
```

`report.py` genera `summary.md` (tablas listas para la tesis) e `indicators.json`. Cada corrida queda en su propia carpeta con `manifest.json`, que registra commit, etiqueta, hash del binario, hash del catálogo, opciones, entorno y hash de la IR de cada base.

---

## 11. Qué va a la tesis

| Tesis | Fuente |
|---|---|
| Tabla 19 (corpus) | Secciones 1 y 7 de este protocolo |
| Tabla 21 (resultados) | `summary.md`: contraste con metas y bloques A, B y C |
| PCD por tipo de conflicto | Tabla del bloque B en `summary.md` |
| TRI por par | Tabla de tiempos del bloque C |
| 5.6.4 Limitaciones | Conflictos `not_observable` (por adaptador), asimetrías de `symmetry.json`, número de correcciones de la verdad, sesgo de evaluador |
| Anexo de la suite | Este protocolo, `config/suite.toml`, `manifest.json` y `summary.md` |

---

## 12. Registro de cambios al protocolo

| Fecha | Cambio | Motivo |
|---|---|---|
| 2026-10-19 | Especificación adoptada: **simétrica**. Implementados E1, E2, E3 y E5 en `audd_compare` (E4 no). | Decisión del autor (sección 5) |
| 2026-10-19 | Reglas de tipo del catálogo: `from` se compara con A y `to` con B por nombre exacto; `symmetric = true` en R-TYPE-001, 002, 003 y 006; alternativas con `\|` (p. ej. `DateTime\|Timestamp`). | La comparación anterior era por subcadena y sin sentido (String→Int32 recibía la regla de Int32→String) |
| 2026-10-19 | Northwind se usa en su versión SQLite (`bases/northwind/northwind.db`). | No hay DDL MySQL de Northwind en el repositorio |
| 2026-10-19 | `mutate.py` genera A y B renderizando la IR de cada base como DDL MySQL (A sin cambios, B con 3 a 6 mutaciones); la verdad sale del registro de mutaciones y de la tabla de la sección 5. Backend `ddl` disponible como alternativa sin servidor; la corrida oficial usa `mysql`. | A y B solo difieren en lo mutado; permite usar bases que no están en MySQL |
| 2026-10-19 | `validate_truth.py`: la pertenencia a la PK no cuenta como UNIQUE. | E4 no se implementó; contarla generaría falsos negativos que no son del núcleo |
