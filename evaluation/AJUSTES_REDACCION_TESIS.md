# Ajustes pendientes para la redacción de la tesis (secciones 5.3, 5.4 y 5.6)

Fecha: 2026-10-19 · Rama: `eval/symmetric-spec` · Commit medido: `842bcdd`

Este documento reúne los resultados preliminares de la suite de evaluación y los ajustes que
hay que hacer en la redacción de la tesis y en el repositorio. Complementa
`PROTOCOLO_EVALUACION.md` (diseño de la evaluación) y `TAREAS_AGENTE.md` (tareas técnicas).

> **Cifras preliminares.** El bloque B se ejecutó con el backend `ddl` en lugar de MySQL, el
> bloque A usó el DDL versionado de cada base en lugar de su fuente MySQL y el bloque C
> (evaluación manual) no se ha ejecutado. Las cifras oficiales salen de la corrida descrita en
> la sección 10 del protocolo.

---

## 1. Métricas actuales

| Indicador | Valor | Meta | Resultado | Fuente y observaciones |
|---|---|---|---|---|
| PCD | 92.96 % (66 VP / 5 FN) | ≥ 70 % | Cumple | Bloque B, 50 pares |
| PRE | 100 % (0 FP) | ≥ 90 % | Cumple | Bloque B |
| PSR | 100 % | ≥ 50 % | Cumple | Todas las sugerencias son del tipo que exige la especificación |
| F1 | 0.99 | Línea base | — | **Engañoso**: ver sección 1.2 |
| REP | 100 % | 100 % | Cumple | Bloques A y B |
| ROB | 100 % | ≥ 90 % | Cumple | 36 comparaciones (A) y 50 pares (B); mediana 0.05 s por comparación |
| SYM | 100 % | Se reporta | — | Bloque A, 15 pares no ordenados |
| FCI | 0 | 0 | Cumple | Bloque A, cada base contra sí misma |
| PRM | 100 % (10 de 10) | ≥ 80 % | Cumple | Subió de 40 % al agregar las pruebas de RF4, RF5, RF6, RNF1, RNF3 y RNF4 |
| IAC-1 | 1072 descargas | ≥ 20 | Cumple | Acumuladas en npm desde la publicación |
| IAC-2 | **0 reales** | Se reporta | — | Las 12 interacciones contadas son todas de la cuenta bot de Copilot |
| TRI | — | ≥ 50 % | Pendiente | Requiere el bloque C |

**Criterio de TRL 4** (PCD, PRE, REP, ROB y PRM): se cumple de forma preliminar. Falta
confirmarlo con la corrida oficial sobre MySQL.

### 1.1 PCD por tipo de conflicto (bloque B)

| Tipo | VP | FN | PCD |
|---|---:|---:|---:|
| TypeIncompatible | 22 | 1 | 95.65 % |
| LengthMismatch | 12 | 0 | 100 % |
| ConstraintMismatch | 13 | 0 | 100 % |
| NullabilityMismatch | 19 | 1 | 95.00 % |
| NormalizationCollision | 0 | 3 | 0 % |

- Los 2 FN de tipo y de nulabilidad están en tablas renombradas por sinónimo: sin `--domain`,
  AUDD no empareja esas tablas y no ve sus conflictos.
- La colisión de normalización da 0 % porque el motor solo la detecta entre campos *no
  emparejados* de lados distintos, mientras que la especificación la define como dos campos
  de B que se normalizan igual.
- Los 71 conflictos de la verdad quedaron como `observable` (ninguno excluido por limitación
  del adaptador).

Operadores aplicados en los 50 pares: `drop_table` 25, `drop_field` 23, `change_type` 23,
`rename_synonym` 23, `add_table` 23, `toggle_nullable` 20, `rename_table_synonym` 19,
`add_field` 19, `rename_unrelated` 15, `toggle_unique` 13, `change_length` 12,
`normalization_collision` 3.

### 1.2 Por qué F1 = 0.99 es engañoso

De las 181 coincidencias de la verdad que involucran un campo o una tabla renombrados por
sinónimo, AUDD detecta **0**. El valor de 0.99 se debe a unos 10 800 campos que no cambiaron.
Hay que reportar por separado el F1 (o la exhaustividad) sobre elementos renombrados.

---

## 2. Ajustes en la redacción

### 2.1 Sección 5.3 (indicadores)

- **Fórmula de PSR.** La fórmula actual cuenta cualquier sugerencia, que es el defecto S3 del
  protocolo (daba 100 % sin significado). El párrafo ya describe la versión correcta; la
  fórmula debe ser:

  `PSR = VP con sugerencia del tipo esperado / VP con sugerencia esperada definida × 100`

- **PCD.** Agregar que solo cuentan los conflictos *observables* en la IR, es decir, aquellos
  cuya diferencia llega al núcleo después de que el adaptador carga el esquema. Las mutaciones
  que el adaptador pierde se reportan aparte como limitación del adaptador.

- **Especificación de conflictos.** Declarar que se adoptó la **especificación de unificación
  simétrica** (sección 5 del protocolo):
  - toda diferencia que obligue a decidir cómo queda el esquema unificado es conflicto,
    incluidas las conversiones seguras (severidad baja) y con pérdida (severidad media);
  - String/Text y DateTime/Timestamp son equivalentes y no generan conflicto;
  - la nulabilidad distinta es conflicto en cualquier sentido;
  - UNIQUE cuenta como restricción del campo o como clave única de una sola columna.

- **F1.** Reportar dos valores: F1 global y F1 (o exhaustividad) sobre elementos renombrados.
  El segundo es el que mide «incluidos los renombrados».

- **REP.** Precisar que el hash se calcula sobre `diff.json`, `unified_schema.json`,
  `suggestions.json` y `decision_log.json` en 3 ejecuciones con `--deterministic`. Además,
  hoy las salidas son idénticas byte a byte.

- **ROB.** Precisar que una comparación es robusta si termina con código 0 y produce JSON
  válidos; se reporta el menor valor entre los bloques A y B.

- **PRM.** Actualizar la frase de RF6: ya no es solo «carga, aplicación y registro». RF6 tiene
  pruebas de K01 a K08:
  - K01, carga del catálogo (incluido `--catalog-optional`);
  - K02, prioridad del catálogo sobre el motor;
  - K03, trazabilidad del origen (`catalog:<regla>` o `engine:default`);
  - K04, equivalencias de dominio;
  - K05, aprendizaje con `audd catalog learn`;
  - K06, versionado del catálogo en el reporte;
  - K07, sin dependencias de red;
  - K08, una prueba por cada una de las 14 reglas.

- **IAC-2.** Indicar que se excluyen las cuentas bot.

- **Agregar SYM y FCI** como indicadores que se reportan sin meta:
  - SYM: porcentaje de pares no ordenados en que X→Y y Y→X dan las mismas coincidencias, los
    exclusivos intercambiados y los mismos tipos de conflicto;
  - FCI: conflictos y exclusivos al comparar una base consigo misma (lo esperado es 0).

- **Tabla 18.**
  - Sustituir la columna «Corpus» (niveles 1 a 4) por los bloques A, B y C.
  - Fuente oficial de PCD, PRE, PSR y F1: bloque B. TRI: bloque C. REP y ROB: el menor de A y B.
    SYM y FCI: bloque A.

### 2.2 Sección 5.6.1 (diseño de la evaluación)

El corpus por niveles ya no corresponde con lo que se construyó.

- **Nivel 1 (identidad entre motores).** Ya no es válido como «sin conflictos»: con la
  especificación simétrica, Chinook en MySQL contra SQLite produce 24 conflictos reales
  (INT es Int32 e INTEGER es Int64, conversión segura de severidad baja). Además, Sakila no
  tiene versión SQLite en el repositorio. La identidad se mide ahora como cada base contra sí
  misma (FCI).
- **Nivel 3 (migraciones de Grow Venta).** No se construyó. Quitarlo o pasarlo a trabajo
  futuro.

**Tabla 19 propuesta (por bloques):**

| Bloque | Propósito | Pares | Verdad de referencia | Indicadores |
|---|---|---|---|---|
| A. Matriz cruzada | Identidad, simetría y robustez | 6 bases, todas contra todas y cada una contra sí misma: 36 | No requiere | ROB, REP, SYM, FCI, tiempo |
| B. Mutaciones controladas | Detección con cambios conocidos | 50 (5 bases × semillas 1000 a 1009, de 3 a 6 cambios cada una) | Automática: registro de mutaciones, validada contra la IR | **PCD, PRE, PSR, F1** |
| C. Pares reales con verdad manual | Comportamiento en esquemas reales y tiempo | 3 obligatorios (M1 a M3) + 2 opcionales | Manual, ciega y sellada con un commit antes de ejecutar AUDD | **TRI**; PCD, PRE, PSR y F1 como evidencia |

Bases: chinook, employees, northwind, unicenta, growtravel y growventa. Pares manuales:
M1 growtravel–growventa, M2 unicenta–growventa, M3 northwind–growventa (M4 chinook–northwind
y M5 employees–northwind, opcionales).

- **Generador del bloque B.** No muta Grow Venta: toma la IR de cada una de las 5 bases,
  la renderiza como DDL MySQL (A, sin cambios) y aplica las mutaciones sobre la misma IR (B).
  Así, A y B solo difieren en lo mutado.

**Tabla 20 corregida (operadores reales del generador):**

| Cambio aplicado al esquema B | Resultado esperado de AUDD | Sugerencia esperada |
|---|---|---|
| Int32 ↔ Int64 | Conflicto de tipo (severidad baja) | CastSafe (R-TYPE-001) |
| Entero → Decimal | Conflicto de tipo | CastSafe (R-TYPE-002) |
| Float → Decimal | Conflicto de tipo | CastRisky (R-TYPE-003) |
| Entero → texto | Conflicto de tipo | CastSafe (R-TYPE-004) |
| Texto → entero | Conflicto de tipo | CastRisky (R-TYPE-005) |
| Boolean → entero | Conflicto de tipo | CastSafe (R-TYPE-006) |
| Date → DateTime | Conflicto de tipo | CastSafe (R-TYPE-007) |
| DateTime → Date | Conflicto de tipo | CastRisky (R-TYPE-008) |
| Cambiar la longitud (VARCHAR(n) → VARCHAR(2n) o n/2) | Conflicto de longitud | PreferType (R-LEN-001) |
| Invertir la nulabilidad (columnas que no son PK) | Conflicto de nulabilidad | PreferType (R-NULL-001) |
| Agregar o quitar UNIQUE | Conflicto de restricción | PreferType (R-CONS-001) |
| Agregar un campo que se normaliza igual que otro (`cliente_id` y `ClienteId`) | Colisión de normalización | RenameField (R-NORM-001) |
| Renombrar un campo o una tabla por sinónimo | Coincidencia (solo se detecta con `--domain`; límite conocido de la corrida oficial) | — |
| Renombrar un campo con un nombre sin relación | Un exclusivo en A y otro en B (límite conocido) | — |
| Agregar o eliminar un campo o una tabla | Exclusivo en B o en A | — |

Correcciones respecto de la versión actual de la tabla:

- quitar «cambiar el estilo de un nombre» y «pluralizar»: no son operadores del generador;
- quitar CHECK y clave primaria: E4 no está implementado y el motor no emite esos conflictos;
- quitar la fila duplicada de «Longitud»;
- entero → texto es CastSafe (R-TYPE-004), no «conversión con riesgo»;
- la clave primaria no es «conflicto de tipo».

- **Procedimiento.** Alinear el párrafo del procedimiento con la sección 10 del protocolo:
  1. preparar las bases en MySQL y etiquetar la versión;
  2. ejecutar el bloque A (`cross.py`);
  3. generar y ejecutar el bloque B (`mutate.py`, `mutations.py`);
  4. ejecutar el bloque C (`manual.py`);
  5. calcular PRM e IAC;
  6. generar el reporte (`report.py`).

  Cada corrida queda en su propia carpeta con `manifest.json` (commit, etiqueta, hash del
  binario, hash del catálogo, entorno y hash de la IR de cada base).

- **TRI.** Describir el flujo de `manual.py` (verdad sellada con commit antes de ejecutar AUDD,
  condición manual y asistida, adjudicación de FP y FN) y resolver el `[PENDIENTE]` sobre si
  participa una segunda persona de Grow Venta.

### 2.3 Sección 5.4 (caso Grow)

Con la especificación simétrica, el caso Grow Travel contra Grow Venta ya no da 17 / 145 / 4,
sino **17 coincidencias, 176 exclusivos y 12 conflictos**. Los exclusivos cuentan campos de
ambos lados, incluidos los de tablas sin equivalente. Actualizar las cifras y explicar el cambio.

### 2.4 Sección 5.6.2 (resultados)

Cambiar la Tabla 21 de niveles a bloques (A, B, C) y agregar:

- PCD por tipo de conflicto (sección 1.1 de este documento);
- F1 global y F1 sobre elementos renombrados;
- SYM y FCI del bloque A;
- PRM desglosado por requerimiento;
- IAC-1 e IAC-2 a la fecha de corte.

### 2.5 Sección 5.6.4 (limitaciones)

Agregar a las cuatro limitaciones ya previstas:

- la colisión de normalización no se detecta según la especificación (PCD de 0 % en ese tipo);
- el renombrado por sinónimo no se detecta sin dominio activo;
- E4 (CHECK y pertenencia a la clave primaria) no está implementado; R-CONS-002 y R-CONS-003
  solo son alcanzables desde el catálogo;
- Northwind se usa en SQLite porque no hay DDL MySQL en el repositorio;
- IAC-2 es 0 a la fecha de corte.

---

## 3. Ajustes pendientes en el repositorio (por prioridad)

1. **Corrida oficial sobre MySQL.**
   - Definir `AUDD_MYSQL` (`usuario:clave@127.0.0.1:3306`).
   - `python evaluation/suite/setup_mysql.py`
   - `python evaluation/mutate.py --all --backend mysql`
   - Ejecutar el resto de la sección 10 del protocolo y `report.py`.
   - Crear la etiqueta `v0.9.0-eval` en el commit evaluado.
2. **Bloque C.**
   - Copiar los nombres reales de las tablas en `scope_a` y `scope_b` (marcados `[AJUSTAR]`)
     de `config/suite.toml`.
   - Construir y sellar la verdad de M1 a M3 con `manual.py` y hacer commit antes de ejecutar
     AUDD.
   - Registrar los tiempos para obtener TRI.
3. **`adoption.py`:** excluir las cuentas bot (`type == "Bot"`) de IAC-2.
4. **Colisión de normalización:** decidir si se alinea el motor con la especificación o se
   declara la semántica actual en la tesis.
5. **F1 sobre elementos renombrados:** calcularlo por separado en `score.py` y `report.py`.
6. **Catálogo frente a la tabla 6.4:** agregar R-LEN-002, R-NULL-002 y R-NORM-002, o declarar
   en la tesis que la versión 0 tiene 14 reglas.
7. **Limpieza.**
   - `evaluation/VALIDATION_REPORT.md` y `evaluation/results/` antiguos muestran cifras
     inválidas (PCD de 12 %): marcarlos como obsoletos o retirarlos.
   - Registrar en `datasets.toml` los hashes de `bases/<id>/schema.sql` después del setup.
   - Verificar las licencias marcadas `[VERIFICAR]` en `config/suite.toml`.
