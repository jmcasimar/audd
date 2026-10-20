# Ajustes para la redacción de la tesis (secciones 5.3, 5.4 y 5.6)

Fecha: 2026-10-19 · Rama: `eval/symmetric-spec` · Medido después del commit `e5fb8f0`, con
los cambios de colisión, dominio y mutaciones descritos en la sección 13 del protocolo.

Este documento sirve para redactar la evaluación en la tesis. Tiene cuatro partes:

1. **Métricas actuales:** los resultados preliminares.
2. **Cómo se evalúa AUDD:** texto base para las secciones 5.3 y 5.6, con una ficha detallada
   de cada métrica (qué mide, qué parte del algoritmo evalúa, cómo se lee y qué dice hoy).
3. **Ajustes en la redacción:** qué cambiar en el texto actual de la tesis.
4. **Ajustes pendientes en el repositorio.**

Complementa `PROTOCOLO_EVALUACION.md` (el procedimiento paso a paso) y `TAREAS_AGENTE.md`
(tareas técnicas).

> **Cifras preliminares.** El bloque B se ejecutó con el backend `ddl` en lugar de MySQL, el
> bloque A usó el DDL versionado de cada base en lugar de su fuente MySQL y el bloque C
> (evaluación manual) no se ha ejecutado. Las cifras oficiales salen de la corrida descrita en
> la sección 10 del protocolo.

---

## 1. Métricas actuales

Bloque B: 50 pares (5 bases × semillas 1000 a 1009), de 5 a 10 mutaciones cada uno, con los
operadores de la sección 6.2 del protocolo. Corrida oficial sin dominio.

| Indicador | Valor | Meta | Resultado | Fuente y observaciones |
|---|---|---|---|---|
| PCD | 84.96 % (113 VP / 20 FN) | ≥ 70 % | Cumple | Bloque B. Fácil 100 %, medio 78 %, difícil 0 % (sección 1.2) |
| PRE | 100 % (0 FP) | ≥ 90 % | Cumple | Bloque B. Ningún conflicto falso, ni siquiera en los 22 cambios equivalentes |
| PSR | 100 % | ≥ 50 % | Cumple | Sobre los conflictos detectados; ver la advertencia de la sección 1.4 |
| F1 global | 0.996 | Línea base | — | Lo dominan unos 10 900 campos sin cambios; no es informativo |
| **F1 de elementos afectados** | **0.61** (P 1.00, R 0.44) | Línea base | — | El valor que hay que reportar (sección 1.3) |
| REP | 100 % | 100 % | Cumple | Bloques A y B |
| ROB | 100 % | ≥ 90 % | Cumple | 36 comparaciones (A) y 50 pares (B); mediana 0.04 s por comparación |
| SYM | 100 % | Se reporta | — | Bloque A, 15 pares no ordenados |
| FCI | 0 | 0 | Cumple | Bloque A, cada base contra sí misma |
| PRM | 100 % (10 de 10) | ≥ 80 % | Cumple | Pruebas de aceptación aprobadas de RF1 a RF6 y RNF1 a RNF4 |
| IAC-1 | 1072 descargas | ≥ 20 | Cumple | Acumuladas en npm desde la publicación |
| IAC-2 | **0 reales** | Se reporta | — | Las 12 interacciones contadas son todas de la cuenta bot de Copilot |
| TRI | — | ≥ 50 % | Pendiente | Requiere el bloque C |

**Criterio de TRL 4** (PCD, PRE, REP, ROB y PRM): se cumple de forma preliminar. Falta
confirmarlo con la corrida oficial sobre MySQL.

### 1.1 PCD por tipo de conflicto

| Tipo | VP | FN | PCD |
|---|---:|---:|---:|
| ConstraintMismatch | 22 | 0 | 100 % |
| NormalizationCollision | 16 | 0 | 100 % |
| LengthMismatch | 27 | 5 | 84.4 % |
| NullabilityMismatch | 27 | 8 | 77.1 % |
| TypeIncompatible | 21 | 7 | 75.0 % |

- Los 133 conflictos de la verdad son `observable`: ninguno se excluyó por pérdida del adaptador.
- La colisión de normalización pasó de 0 % a 100 % después de corregir el núcleo (sección 13
  del protocolo): ahora se detecta dentro de cada esquema y el conflicto indica el campo.
- Todos los FN están en cambios **combinados** (renombrar y además cambiar tipo, longitud o
  nulabilidad) o en tablas renombradas: si AUDD no empareja el campo o la tabla, no puede ver
  su conflicto.

### 1.2 PCD por nivel de la mutación

| Nivel | VP | FN | PCD | Qué incluye |
|---|---:|---:|---:|---|
| Fácil | 81 | 0 | 100 % | Un cambio estructural sobre un campo que conserva su nombre |
| Medio | 32 | 9 | 78.0 % | Cambio combinado con renombrado de estilo, plural o prefijo; colisión |
| Difícil | 0 | 11 | 0 % | Cambio combinado con abreviatura o sinónimo |

Interpretación: el núcleo detecta todos los conflictos cuando sabe qué campo corresponde con
cuál; su límite está en el emparejamiento de nombres, no en la detección de conflictos.

### 1.3 Coincidencias por categoría (F1)

| Categoría | Ejemplo | VP | FN | R | F1 sin dominio | F1 con `--domain pos` |
|---|---|---:|---:|---:|---:|---:|
| style | `cliente_id` → `ClienteId` | 22 | 0 | 1.00 | 1.00 | 1.00 |
| plural | `customer` → `customers` | 18 | 1 | 0.95 | 0.97 | 0.97 |
| affix | `email` → `cus_email` | 2 | 11 | 0.15 | 0.27 | 0.27 |
| abbreviation | `quantity` → `qty` | 0 | 12 | 0.00 | 0.00 | 0.00 |
| synonym | `phone` → `telephone` | 0 | 16 | 0.00 | 0.00 | 0.22 |
| translation | `status` → `estado` | 0 | 13 | 0.00 | 0.00 | 0.38 |
| **Elementos afectados** | todos los anteriores | 42 | 53 | 0.44 | **0.61** | **0.66** |

- **Precisión de 1.00 en todas las categorías:** AUDD no emparejó ninguno de los 26
  distractores (p. ej. `created_at` → `created_by`) ni ninguno de los 25 renombrados sin
  relación. No inventa correspondencias; su debilidad es la exhaustividad.
- **El dominio `pos` aporta poco** (de 0.61 a 0.66): solo cubre 2 de 16 sinónimos y 3 de 13
  traducciones, porque su diccionario es pequeño y orientado a punto de venta.
- Las abreviaturas y los prefijos heredados no se detectan ni con dominio: no hay estrategia
  para ellos en el núcleo.

### 1.4 Advertencia sobre los 100 %

- El bloque B es una **verificación**: comprueba que la implementación cumple la
  especificación con cambios controlados. Los 100 % de PRE, PSR, REP, ROB y FCI no significan
  que AUDD resuelva cualquier caso real.
- PSR se calcula sobre los conflictos detectados, y en el bloque B la sugerencia esperada sale
  de la misma tabla que implementa el catálogo. En el bloque C la sugerencia esperada la
  escribe una persona antes de ver AUDD; ahí se pone a prueba de verdad.
- Las métricas sí fallan cuando hay defectos: durante el desarrollo dieron PCD 12 % (verdad mal
  construida), PSR 69.7 % (reglas sin sentido A→B), falsos positivos (emparejamiento muchos a
  muchos) y 0 % en colisiones (definición distinta). Cada falla llevó a corregir un defecto.

---

## 2. Cómo se evalúa AUDD (texto base para 5.3 y 5.6)

Esta sección está escrita para adaptarse directamente a la tesis. Explica primero qué partes
del algoritmo existen, luego cómo se organiza la evaluación y, finalmente, qué significa cada
métrica y qué dice de cada parte del algoritmo.

### 2.1 Las partes del algoritmo que se evalúan

AUDD procesa dos esquemas, A y B, en cinco etapas. Cada métrica evalúa una o varias de ellas;
por eso, cuando una métrica baja, indica **dónde** está el problema.

| Etapa | Qué hace | Ejemplo | Métricas que la evalúan |
|---|---|---|---|
| **1. Adaptadores** | Leen cada fuente (archivo CSV, JSON, XML o DDL; base SQLite, MySQL, PostgreSQL, MongoDB o SQL Server) y la traducen a una representación intermedia común, la **IR**: entidades, campos, tipo canónico, nulabilidad, restricciones y claves | `VARCHAR(50) NOT NULL` en MySQL se vuelve «String, longitud 50, no nulo» | ROB, número de conflictos `not_observable`, RNF1 y RNF3 (PRM) |
| **2. Emparejador** | Decide qué entidad de A corresponde con cuál de B y, dentro de cada par, qué campo con cuál. Usa, en orden de prioridad: nombre exacto, nombre normalizado, coincidencia semántica (plurales, sinónimos, similitud de palabras y caracteres) y, si hay un dominio activo, equivalencias del catálogo. Cada elemento se empareja una sola vez | `customer_id` ↔ `CustomerId`; `customers` ↔ `Customer` | **F1** (global, de elementos afectados y por categoría), FCI, SYM |
| **3. Detector de conflictos** | En cada par emparejado compara tipo, longitud, nulabilidad y UNIQUE, y busca colisiones de normalización. Clasifica cada diferencia y le asigna severidad | `INT` contra `BIGINT`: conflicto de tipo, severidad baja | **PCD**, **PRE**, FCI, SYM |
| **4. Motor de sugerencias** | Propone cómo resolver cada conflicto. Consulta primero el catálogo de reglas (R-TYPE-001, R-NULL-001, …) y, si ninguna aplica, usa reglas por defecto. Registra el origen de cada sugerencia | `INT` contra `BIGINT` → «convertir a Int64 sin pérdida» (R-TYPE-001) | **PSR**, RF6 (PRM) |
| **5. Unificación y salidas** | Construye el esquema unificado y escribe `diff.json`, `unified_schema.json`, `suggestions.json`, `decision_log.json` y `report.md` | | **REP**, RF4, RF5 y RNF4 (PRM), **TRI** |

### 2.2 Vocabulario

- **Esquema A y esquema B:** las dos fuentes que se comparan. A es el origen y B el destino.
- **Coincidencia:** un elemento de A que corresponde a uno de B (una tabla con una tabla, una
  columna con una columna), aunque se llamen distinto.
- **Exclusivo:** un elemento que solo existe en un lado. Si una tabla completa no tiene
  equivalente, cuenta como un solo exclusivo.
- **Conflicto:** una coincidencia cuya definición difiere (tipo, longitud, nulabilidad,
  restricción) o dos nombres que chocan al normalizarse.
- **Verdad de referencia:** el registro, hecho antes de ejecutar AUDD, de las coincidencias,
  exclusivos y conflictos que realmente existen entre A y B.
- **VP (verdadero positivo):** algo que AUDD reporta y existe en la verdad.
- **FP (falso positivo):** algo que AUDD reporta y no existe en la verdad.
- **FN (falso negativo):** algo que existe en la verdad y AUDD no reporta.
- **Observable:** un conflicto cuya diferencia llegó a la IR. Si el adaptador la perdió al
  leer el esquema, el núcleo nunca pudo verla.

### 2.3 Las tres etapas de la evaluación

La evaluación avanza de lo más controlado a lo más real. Cada etapa responde una pregunta y
solo tiene sentido si la anterior salió bien.

**Etapa A. Cada base contra sí misma y contra las demás (matriz cruzada).**
Antes de medir qué detecta AUDD hay que comprobar que la herramienta es confiable. Comparar una
base consigo misma es el **control negativo**: la respuesta correcta se conoce sin ambigüedad
(todo coincide, no hay exclusivos ni conflictos), así que cualquier diferencia reportada es un
error seguro. Esta etapa compara las seis bases consigo mismas y entre sí, en ambos sentidos
(36 comparaciones), y repite cada comparación tres veces. Mide si la herramienta procesa todos
los esquemas sin fallar (ROB), si da siempre la misma salida (REP), si el resultado es igual al
invertir A y B (SYM) y si inventa diferencias (FCI). No requiere verdad de referencia.

**Etapa B. Prueba controlada de mutaciones (verificación sintética).**
Para saber qué detecta el algoritmo hace falta conocer con exactitud qué diferencias existen.
Un generador toma el esquema de cada base y produce una variante con entre 5 y 10 cambios
elegidos al azar con una semilla fija (por ejemplo, cambiar un `INT` por `BIGINT`, renombrar
`quantity` por `qty` o sustituir `created_at` por `created_by`). Como el generador registra cada
cambio, la verdad de referencia se obtiene automáticamente y sin intervención humana. Los
cambios se organizan en tres niveles de dificultad, de modo que la prueba muestra hasta dónde
llega el algoritmo y dónde deja de funcionar. Antes de puntuar, cada conflicto esperado se
valida contra la IR para separar los fallos del adaptador de los del núcleo. Esta etapa es la
fuente oficial de PCD, PRE, PSR y F1, porque es la única con una verdad exacta y suficientes
casos (50 pares, más de 130 conflictos y 11 000 coincidencias).

**Etapa C. Prueba real con una verdad elaborada por un experto (validación).**
La última etapa enfrenta a AUDD con lo que un generador no puede sintetizar: sistemas reales,
con nombres con historia, convenciones propias y cambios de significado. Un experto analiza los
esquemas de cada par (por ejemplo, UniCenta contra Grow Venta) **sin ver ninguna salida de
AUDD**, registra la verdad en un formato fijo y la sella con un commit antes de ejecutar la
herramienta. Después, AUDD lee las bases con sus conectores reales (MySQL y SQLite) y sus
resultados se comparan contra esa verdad. Se mide también el tiempo: el experto (o un segundo
evaluador) elabora el documento de correspondencias sin AUDD y con AUDD, y la diferencia da el
indicador TRI. Esta etapa tiene pocos pares y una verdad humana, por eso se reporta aparte y no
se promedia con la etapa B.

### 2.4 Fichas de las métricas

Cada ficha tiene la misma estructura: qué pregunta responde, cómo se calcula, qué parte del
algoritmo evalúa, cómo se lee, qué dice hoy y qué área de oportunidad revela.

---

#### PCD · Proporción de conflictos detectados

- **Pregunta:** de los conflictos que realmente existen, ¿cuántos encuentra AUDD?
- **Fórmula:** `PCD = VP / (VP + FN) × 100`, sobre los cinco tipos estructurales
  (tipo, longitud, nulabilidad, restricción y colisión) y solo con conflictos observables.
- **Cómo se calcula:** `score.py` compara cada conflicto de `diff.json` con la verdad; un
  conflicto es VP si coinciden la entidad, el campo y el tipo. `mutations.py` suma los VP y FN
  de los 50 pares antes de dividir (agregación micro).
- **Qué parte del algoritmo evalúa:** el **detector de conflictos** y, de forma indirecta, el
  **emparejador**: un conflicto solo se puede detectar si antes se emparejó el campo.
- **Cómo se lee:**
  - PCD alto con F1 bajo: el detector funciona, pero el emparejador deja campos sin pareja.
  - PCD bajo en un solo tipo: falta una regla de detección para ese tipo.
  - PCD bajo en el nivel fácil: el detector tiene un defecto, porque ahí el campo conserva su
    nombre.
- **Desgloses obligatorios:** por tipo de conflicto (dónde falla el detector) y por nivel de
  dificultad (si falla el detector o el emparejador).
- **Resultado actual:** 84.96 %. Fácil 100 %, medio 78 %, difícil 0 %. Por tipo, de 75 %
  (tipo) a 100 % (restricción y colisión).
- **Fortaleza:** cuando el campo conserva su nombre, el detector no falla (100 % en el nivel
  fácil).
- **Debilidad:** todos los FN vienen de campos o tablas renombrados que no se emparejaron.
- **Área de oportunidad:** mejorar el emparejador (abreviaturas, prefijos, sinónimos); el
  detector no necesita cambios.
- **Meta:** ≥ 70 %. **Fuente oficial:** bloque B.

---

#### PRE · Precisión de la detección

- **Pregunta:** de los conflictos que reporta AUDD, ¿cuántos son reales?
- **Fórmula:** `PRE = VP / (VP + FP) × 100`.
- **Por qué hace falta:** sin PRE, un detector que reportara todo como conflicto tendría PCD
  de 100 %. PRE castiga las alarmas falsas, que en una migración hacen perder tiempo revisando
  diferencias que no existen.
- **Qué parte del algoritmo evalúa:** el **detector de conflictos** (que no confunda
  diferencias equivalentes con conflictos) y el **emparejador** (un campo emparejado con el
  equivocado produce conflictos falsos).
- **Cómo se lee:** PRE bajo indica reglas demasiado estrictas (por ejemplo, tratar `VARCHAR`
  contra `TEXT` como conflicto) o emparejamientos erróneos.
- **Prueba específica:** el bloque B incluye **cambios equivalentes** (`VARCHAR` → `TEXT`,
  `DATETIME` ↔ `TIMESTAMP`), que según la especificación no son conflicto. Si AUDD los
  reportara, PRE bajaría.
- **Resultado actual:** 100 %; 0 conflictos falsos, incluidos los 22 cambios equivalentes.
- **Fortaleza:** el detector distingue bien lo equivalente de lo conflictivo.
- **Área de oportunidad:** confirmar el valor en el bloque C, donde aparecen diferencias que el
  generador no produce.
- **Meta:** ≥ 90 %. **Fuente oficial:** bloque B.

---

#### PSR · Pertinencia de las sugerencias

- **Pregunta:** cuando AUDD detecta un conflicto, ¿propone la resolución adecuada?
- **Fórmula:** `PSR = VP con sugerencia del tipo esperado / VP con sugerencia esperada
  definida × 100`.
- **Cómo se calcula:** para cada conflicto detectado, la verdad indica el tipo de sugerencia
  esperado según la especificación (por ejemplo, `CastRisky` para texto → entero). Se busca en
  `suggestions.json` una sugerencia de ese tipo para el mismo campo y tipo de conflicto.
- **Por qué no se cuenta «alguna sugerencia»:** AUDD siempre produce alguna sugerencia, así que
  esa fórmula daría 100 % aunque las sugerencias fueran incorrectas. Comparar contra el tipo
  esperado sí puede fallar: antes de corregir el catálogo dio 69.7 %, porque aplicaba la regla
  de entero → texto también a texto → entero.
- **Qué parte del algoritmo evalúa:** el **motor de sugerencias** y el **catálogo de reglas**
  (que la regla correcta aplique en el sentido correcto).
- **Cómo se lee:** PSR bajo en un tipo indica una regla faltante o mal condicionada en el
  catálogo. El origen de cada sugerencia (`catalog:R-…` o `engine:default`) dice qué regla
  respondió.
- **Resultado actual:** 100 %.
- **Advertencia:** en el bloque B la sugerencia esperada sale de la misma tabla que implementa
  el catálogo, así que mide que el catálogo cumple su especificación. En el bloque C la decide
  el experto y mide si la sugerencia es útil en la práctica.
- **Meta:** ≥ 50 %. **Fuente oficial:** bloque B; el bloque C como evidencia.

---

#### F1 · Calidad de las correspondencias

- **Pregunta:** ¿qué tan bien identifica AUDD qué elemento de A corresponde a qué elemento de B?
- **Fórmula:** precisión `P = VP / (VP + FP)`, exhaustividad `R = VP / (VP + FN)` y
  `F1 = 2 × P × R / (P + R)`, calculadas sobre las coincidencias (tablas y campos).
- **Regla de conteo:** una coincidencia es VP solo si el campo de B es el correcto. Emparejar
  con el campo equivocado cuenta como FP y FN. Emparejar un distractor o un renombrado sin
  relación (que no deben emparejarse) cuenta como FP.
- **Tres valores que se reportan:**
  1. **F1 global:** todas las coincidencias. Sirve solo como control, porque la mayoría de los
     campos no cambian y son triviales de emparejar (hoy 0.996).
  2. **F1 de elementos afectados:** sin los campos que no cambiaron. Es el valor que mide la
     capacidad real del emparejador (hoy **0.61**).
  3. **F1 por categoría de cambio:** estilo, plural, prefijo, abreviatura, sinónimo y
     traducción. Indica exactamente qué tipo de renombrado reconoce y cuál no.
- **Qué parte del algoritmo evalúa:** el **emparejador** y sus estrategias. La corrida con
  `--domain pos` mide el aporte de las equivalencias de dominio del catálogo.
- **Cómo se lee:**
  - P alto y R bajo: el emparejador es conservador; no inventa correspondencias, pero se le
    escapan muchas.
  - P bajo: empareja elementos que no corresponden (se notaría en los distractores).
  - F1 bajo en una categoría: falta la estrategia para ese tipo de nombre.
- **Resultado actual:** P = 1.00 en todas las categorías; R de 1.00 (estilo) a 0 (abreviatura,
  sinónimo, traducción). Con dominio, F1 de afectados sube de 0.61 a 0.66.
- **Fortalezas:** reconoce cambios de estilo y plurales, y no empareja ninguno de los 26
  distractores ni de los 25 renombrados sin relación.
- **Debilidades:** no reconoce abreviaturas, prefijos heredados, sinónimos ni traducciones
  fuera del diccionario de dominio.
- **Áreas de oportunidad:** diccionario de abreviaturas, eliminación de prefijos heredados y
  ampliación del dominio `pos` con los esquemas de Grow Venta.
- **Meta:** sin meta (línea base). **Fuente oficial:** bloque B; el bloque C como evidencia.

---

#### TRI · Reducción del tiempo de análisis

- **Pregunta:** ¿cuánto tiempo ahorra AUDD frente a hacer el análisis a mano?
- **Fórmula:** `TRI = (t_manual − t_asistido) / t_manual × 100`. El tiempo asistido suma la
  ejecución de AUDD y la revisión del reporte.
- **Cómo se calcula:** en cada par del bloque C se registran en `tiempos.csv` el inicio y el
  fin de la condición manual (solo DDL y documentación) y de la condición asistida (a partir de
  `report.md`). `manual.py tri` calcula el valor por par y el promedio.
- **Ejemplo:** si el análisis manual de M2 toma 135 minutos y el asistido 40, TRI = 70 %.
- **Qué parte del algoritmo evalúa:** el resultado de todo el algoritmo, desde la perspectiva
  del usuario: depende de cuántas correspondencias acierta (menos trabajo de corrección) y de
  qué tan legible es el reporte (RNF4).
- **Cómo se lee:** un TRI alto con F1 moderado indica que el reporte acelera el trabajo aun
  cuando hay que completarlo. Un TRI bajo indica que corregir el reporte cuesta casi lo mismo
  que hacer el análisis desde cero.
- **Sesgos a declarar:** pocos pares (3) y, si participa una sola persona, efecto de
  aprendizaje entre condiciones.
- **Resultado actual:** pendiente.
- **Meta:** ≥ 50 %. **Fuente oficial:** bloque C.

---

#### REP · Reproducibilidad

- **Pregunta:** con las mismas entradas y la misma configuración, ¿AUDD da siempre la misma
  salida?
- **Fórmula:** `REP = comparaciones con salidas idénticas en 3 ejecuciones / comparaciones ×
  100`. Se comparan los hashes de `diff.json`, `unified_schema.json`, `suggestions.json` y
  `decision_log.json`, con `--deterministic` (que fija las marcas de tiempo).
- **Qué parte del algoritmo evalúa:** la **etapa de salidas** y el determinismo de todo el
  proceso (orden de recorrido, desempates del emparejador).
- **Por qué importa:** sin reproducibilidad, las demás métricas no se pueden repetir ni
  auditar; tampoco se puede confiar en un registro de decisiones.
- **Cómo se lee:** cualquier valor menor que 100 % es un defecto. Durante el desarrollo esta
  prueba encontró dos: el orden de unas listas en `diff.json` y el de los tipos de conflicto en
  `report.md` cambiaban entre ejecuciones.
- **Resultado actual:** 100 %; las salidas son además idénticas byte a byte.
- **Meta:** 100 %. **Fuente oficial:** bloques A y B (se reporta el menor).

---

#### ROB · Robustez

- **Pregunta:** ¿AUDD procesa todos los esquemas sin fallar?
- **Fórmula:** `ROB = comparaciones que terminan con código 0 y producen JSON válidos /
  comparaciones × 100`. Se reporta junto con la mediana del tiempo de ejecución.
- **Qué parte del algoritmo evalúa:** los **adaptadores** (que lean esquemas de tamaños y
  dialectos distintos) y la estabilidad del núcleo.
- **Cómo se lee:** un ROB bajo señala qué base o qué adaptador falla (`cross.csv` indica la
  comparación).
- **Resultado actual:** 100 % en 86 comparaciones; mediana 0.04 s.
- **Fortaleza:** durante la preparación se corrigieron casos que antes fallaban o leían mal
  (comentarios dentro de `CREATE TABLE`, `CREATE UNIQUE INDEX` separado, nombres con espacios).
- **Meta:** ≥ 90 %. **Fuente oficial:** bloques A y B (se reporta el menor).

---

#### SYM · Simetría

- **Pregunta:** ¿el resultado es el mismo al comparar X contra Y que Y contra X?
- **Fórmula:** porcentaje de los 15 pares no ordenados en que X→Y y Y→X dan el mismo número
  de coincidencias, los exclusivos intercambiados y los mismos tipos de conflicto.
- **Por qué importa:** AUDD produce un esquema **unificado**, no una migración en un solo
  sentido; con la especificación simétrica, cualquier asimetría es un defecto.
- **Qué parte del algoritmo evalúa:** el **emparejador** (desempates independientes del orden)
  y el **detector** (reglas sin sentido preferente).
- **Cómo se lee:** `symmetry.json` lista qué tipos de conflicto aparecen en un sentido y no en
  el otro.
- **Resultado actual:** 100 %. Antes del emparejamiento uno a uno, los exclusivos eran
  asimétricos en todos los pares.
- **Meta:** se reporta. **Fuente:** bloque A.

---

#### FCI · Falsos conflictos en identidad

- **Pregunta:** al comparar una base consigo misma, ¿AUDD reporta algo que no existe?
- **Fórmula:** suma de conflictos y exclusivos en las 6 comparaciones de identidad. Lo correcto
  es 0.
- **Qué parte del algoritmo evalúa:** el **emparejador** (que cada elemento se empareje con su
  homónimo) y el **detector** (que no vea conflictos entre definiciones idénticas).
- **Cómo se lee:** cualquier valor mayor que 0 es un error seguro. Antes del emparejamiento uno
  a uno, Grow Venta contra sí misma daba 32 exclusivos falsos porque `Product` se emparejaba
  también con `Product_Product`.
- **Resultado actual:** 0.
- **Meta:** 0. **Fuente:** bloque A.

---

#### PRM · Requerimientos implementados

- **Pregunta:** ¿qué parte de los requerimientos del prototipo está implementada y verificada?
- **Fórmula:** `PRM = requerimientos con todas sus pruebas de aceptación aprobadas / 10 ×
  100`, sobre RF1 a RF6 y RNF1 a RNF4.
- **Cómo se calcula:** `requirements_coverage.py` busca las pruebas llamadas
  `acceptance_<requerimiento>_*`, ejecuta `cargo test acceptance` y marca un requerimiento como
  cumplido solo si tiene al menos una prueba y todas pasan.
- **Qué parte del algoritmo evalúa:** el prototipo completo, requerimiento por requerimiento:
  RF1 y RF2 los adaptadores y la IR; RF3 el detector; RF4 la unificación; RF5 el registro de
  decisiones; RF6 el catálogo; RNF1 la portabilidad; RNF2 la reproducibilidad; RNF3 la
  extensibilidad; RNF4 la transparencia del reporte.
- **Cómo se lee:** PRM dice que existe una prueba y que pasa; su rigor depende de lo que
  revise cada prueba. Algunas son más débiles que su criterio: RNF1 revisa la configuración de
  integración continua para Linux, Windows y macOS, pero no ejecuta en macOS localmente; RNF3
  usa un adaptador de prueba mínimo.
- **Resultado actual:** 100 % (10 de 10).
- **Meta:** ≥ 80 %. **Fuente:** pruebas de aceptación.

---

#### IAC-1 e IAC-2 · Adopción comunitaria

- **Pregunta:** ¿hay interés externo en la herramienta?
- **IAC-1:** descargas acumuladas del paquete `audd-node` en npm, desde su publicación hasta la
  fecha de corte. **IAC-2:** issues, solicitudes de cambio y comentarios de cuentas ajenas al
  proyecto en GitHub, **excluidas las cuentas bot**.
- **Qué evalúa:** el potencial de transferencia, no la validez técnica.
- **Cómo se lee:** las descargas de npm incluyen instalaciones automáticas (integración
  continua, espejos), por lo que indican interés inicial y no uso confirmado.
- **Resultado actual:** IAC-1 = 1072 (meta ≥ 20); IAC-2 = 0 interacciones reales (las 12
  contadas eran del bot de Copilot).
- **Fuente:** estadísticas públicas de npm y GitHub.

---

#### Indicador de apoyo · Conflictos no observables

- **Qué es:** número de conflictos de la verdad del bloque B cuya diferencia el adaptador
  perdió al leer el esquema (`not_observable`). Se excluyen de PCD y se reportan aparte.
- **Qué evalúa:** la fidelidad de los **adaptadores**.
- **Cómo se lee:** si son muchos, PCD puede verse bien mientras el adaptador oculta problemas;
  por eso su número se reporta siempre. Con la suite anterior eran muchos (SQLite convertía
  todo entero en Int64 y no conservaba longitudes).
- **Resultado actual:** 0 de 133.

### 2.5 Mapa de métricas y partes del algoritmo

| Parte del algoritmo | Métricas | Estado actual | Lectura |
|---|---|---|---|
| Adaptadores | ROB, no observables, RNF1, RNF3 | 100 %, 0 no observables | Fortaleza |
| Emparejador | F1 de afectados y por categoría, FCI, SYM | F1 0.61; FCI 0; SYM 100 % | **Principal área de oportunidad** |
| Detector de conflictos | PCD por tipo y nivel, PRE | Fácil 100 %, PRE 100 % | Fortaleza |
| Motor de sugerencias y catálogo | PSR, RF6 | 100 % | Fortaleza en entorno controlado; falta el bloque C |
| Unificación y salidas | REP, RF4, RF5, RNF4 | 100 % | Fortaleza |
| Herramienta completa | TRI | Pendiente | Se mide en el bloque C |

### 2.6 Cómo leer las métricas en conjunto

| Combinación | Interpretación |
|---|---|
| PCD alto en el nivel fácil y bajo en el difícil | El detector funciona; el límite está en el emparejador |
| PRE alto y F1 con P alto pero R bajo | El algoritmo es conservador: no inventa, pero se le escapan correspondencias |
| F1 por categoría alto en estilo y plural, bajo en abreviatura y sinónimo | Las estrategias léxicas funcionan; faltan estrategias de vocabulario |
| F1 con dominio mayor que sin dominio | El catálogo de equivalencias aporta; su cobertura limita la mejora |
| FCI = 0 y SYM = 100 % | El emparejamiento uno a uno es consistente y no depende del orden |
| TRI alto con F1 moderado | El reporte ahorra tiempo aunque haya que completarlo |

### 2.7 Diagnóstico actual

**Fortalezas**

- Detección de conflictos: 100 % cuando el campo conserva su nombre, y ningún conflicto falso,
  ni en cambios equivalentes.
- Precisión del emparejamiento: no empareja distractores ni nombres sin relación.
- Confiabilidad: robustez, reproducibilidad y simetría de 100 %, y cero diferencias falsas al
  comparar una base consigo misma.
- Sugerencias trazables al catálogo, con la regla correcta en el sentido correcto.

**Debilidades**

- Exhaustividad del emparejamiento: no reconoce abreviaturas, prefijos heredados, sinónimos ni
  traducciones fuera del diccionario. En consecuencia, los conflictos de esos campos tampoco se
  detectan (PCD 0 % en el nivel difícil).
- El diccionario del dominio `pos` es pequeño: el dominio solo sube F1 de afectados de 0.61 a
  0.66.
- No se comparan CHECK ni la pertenencia a la clave primaria (E4).

**Áreas de oportunidad** (trabajo futuro, capítulo 6)

1. Diccionario de abreviaturas y normalización de prefijos heredados en el emparejador.
2. Ampliación del dominio `pos` con los esquemas de Grow Venta y equivalencias en español.
3. Comparación de CHECK y de claves primarias (E4).
4. Validación con más pares reales y con más de un experto.

---

## 3. Ajustes en la redacción

### 3.1 Sección 5.3 (indicadores)

- **Fórmula de PSR.** La fórmula actual cuenta cualquier sugerencia, que es el defecto S3 del
  protocolo (daba 100 % sin significado). El párrafo ya describe la versión correcta; la
  fórmula debe ser:

  `PSR = VP con sugerencia del tipo esperado / VP con sugerencia esperada definida × 100`

- **PCD.** Agregar que solo cuentan los conflictos *observables* en la IR y que los no
  observables se reportan aparte, con su número. Reportar además PCD por tipo y por nivel.

- **Especificación de conflictos.** Declarar que se adoptó la **especificación de unificación
  simétrica** (sección 3 del protocolo):
  - toda diferencia que obligue a decidir cómo queda el esquema unificado es conflicto,
    incluidas las conversiones seguras (severidad baja) y con pérdida (severidad media);
  - String/Text y DateTime/Timestamp son equivalentes y no generan conflicto;
  - la nulabilidad distinta es conflicto en cualquier sentido;
  - UNIQUE cuenta como restricción del campo o como clave única de una sola columna;
  - hay colisión de normalización cuando dos campos de la misma entidad se normalizan igual
    en un esquema y el otro esquema no tiene exactamente los mismos nombres.

- **F1.** Sustituir el F1 único por F1 de elementos afectados, F1 por categoría y la
  precisión ante distractores, con el F1 global solo como control (ficha de F1, sección 2.4).

- **Dominio.** Indicar que la corrida oficial va sin dominio y que la corrida con
  `--domain pos` se reporta aparte para medir el aporte del diccionario.

- **REP, ROB y PRM.** Usar las definiciones de las fichas de la sección 2.4. En PRM, actualizar
  la frase de RF6: ya no es solo «carga, aplicación y registro», sino K01 a K08 (carga con
  `--catalog-optional`, prioridad, trazabilidad del origen, equivalencias de dominio que solo
  agregan coincidencias, aprendizaje con `audd catalog learn`, versionado, sin dependencias de
  red y una prueba por cada una de las 14 reglas).

- **IAC-2.** Indicar que se excluyen las cuentas bot.

- **Agregar SYM y FCI** como indicadores que se reportan (fichas de la sección 2.4).

- **Agregar una tabla o un párrafo que relacione cada métrica con la parte del algoritmo que
  evalúa** (sección 2.5). Así, cada resultado se interpreta como fortaleza o debilidad de una
  parte concreta.

- **Tabla 18.**
  - Sustituir la columna «Corpus» (niveles 1 a 4) por los bloques A, B y C.
  - Fuente oficial de PCD, PRE, PSR y F1: bloque B. TRI: bloque C. REP y ROB: el menor de A y B.
    SYM y FCI: bloque A.
  - Cambiar «F1 de correspondencias» por «F1 de elementos afectados y por categoría».

### 3.2 Sección 5.6.1 (diseño de la evaluación)

- **Reemplazar los cuatro niveles del corpus por las tres etapas** de la sección 2.3, con su
  justificación: control negativo (A), verificación sintética (B) y validación con un experto
  (C).
- **Nivel 1 (identidad entre motores).** Ya no es válido como «sin conflictos»: con la
  especificación simétrica, Chinook en MySQL contra SQLite produce 24 conflictos reales
  (INT es Int32 e INTEGER es Int64). La identidad se mide como cada base contra sí misma.
- **Nivel 3 (migraciones de Grow Venta).** No se construyó. Quitarlo o pasarlo a trabajo
  futuro.
- **Los niveles de dificultad ahora están dentro del bloque B** (fácil, medio, difícil).

**Tabla 19 propuesta (por bloques):**

| Bloque | Propósito | Pares | Verdad de referencia | Indicadores |
|---|---|---|---|---|
| A. Matriz cruzada | Control negativo, simetría y robustez | 6 bases, todas contra todas y cada una contra sí misma: 36 | No requiere | ROB, REP, SYM, FCI, tiempo |
| B. Mutaciones controladas | Detección y emparejamiento con cambios conocidos, en tres niveles de dificultad | 50 (5 bases × semillas 1000 a 1009, de 5 a 10 cambios cada una) | Automática: registro de mutaciones, validada contra la IR | **PCD, PRE, PSR, F1** |
| C. Pares reales con verdad de experto | Comportamiento en esquemas reales y tiempo | 3 obligatorios (M1 a M3) + 2 opcionales | Manual, ciega y sellada con un commit antes de ejecutar AUDD | **TRI**; PCD, PRE, PSR y F1 como evidencia |

Bases: chinook, employees, northwind, unicenta, growtravel y growventa. Pares manuales:
M1 growtravel–growventa, M2 unicenta–growventa, M3 northwind–growventa (M4 chinook–northwind
y M5 employees–northwind, opcionales).

- **Generador del bloque B.** No muta Grow Venta: toma la IR de cada una de las 5 bases,
  la renderiza como DDL MySQL (A, sin cambios) y aplica las mutaciones sobre la misma IR (B).
  Así, A y B solo difieren en lo mutado.

**Tabla 20 corregida (operadores reales del generador):**

| Nivel | Cambio aplicado al esquema B | Resultado esperado de AUDD | Sugerencia esperada |
|---|---|---|---|
| Fácil | Int32 ↔ Int64 | Conflicto de tipo (severidad baja) | CastSafe (R-TYPE-001) |
| Fácil | Entero → Decimal | Conflicto de tipo | CastSafe (R-TYPE-002) |
| Fácil | Float → Decimal | Conflicto de tipo | CastRisky (R-TYPE-003) |
| Fácil | Entero → texto | Conflicto de tipo | CastSafe (R-TYPE-004) |
| Fácil | Texto → entero | Conflicto de tipo | CastRisky (R-TYPE-005) |
| Fácil | Boolean → entero | Conflicto de tipo | CastSafe (R-TYPE-006) |
| Fácil | Date → DateTime | Conflicto de tipo | CastSafe (R-TYPE-007) |
| Fácil | DateTime → Date | Conflicto de tipo | CastRisky (R-TYPE-008) |
| Fácil | Cambiar la longitud (VARCHAR(n) → 2n o n/2) | Conflicto de longitud | PreferType (R-LEN-001) |
| Fácil | Invertir la nulabilidad (columnas que no son PK) | Conflicto de nulabilidad | PreferType (R-NULL-001) |
| Fácil | Agregar o quitar UNIQUE | Conflicto de restricción | PreferType (R-CONS-001) |
| Fácil | Agregar o eliminar un campo o una tabla | Exclusivo en B o en A | — |
| Fácil | Renombrar por estilo (`cliente_id` → `ClienteId`) o plural | Coincidencia | — |
| Medio | Renombrar con prefijo heredado (`email` → `cus_email`) | Coincidencia | — |
| Medio | Renombrar **y** cambiar tipo, longitud o nulabilidad del mismo campo | Coincidencia y conflicto | La del cambio |
| Medio | Cambio equivalente (`VARCHAR` → `TEXT`, `DATETIME` ↔ `TIMESTAMP`) | **Ningún** conflicto | — |
| Medio | Agregar un campo que se normaliza igual que otro (`cliente_id` y `ClienteId`) | Colisión de normalización | RenameField (R-NORM-001) |
| Difícil | Renombrar por abreviatura (`quantity` → `qty`) | Coincidencia | — |
| Difícil | Renombrar por sinónimo (`phone` → `telephone`) o traducción (`status` → `estado`) | Coincidencia (con `--domain`, solo si el diccionario la cubre) | — |
| Difícil | Renombrar sin relación (`fld_1234`) | Un exclusivo en A y otro en B (límite conocido) | — |
| Difícil | Distractor: sustituir por un nombre casi igual de significado distinto (`created_at` → `created_by`) | **Ninguna** coincidencia | — |

Correcciones respecto de la versión actual de la tabla:

- quitar CHECK y clave primaria: E4 no está implementado y el motor no emite esos conflictos;
- quitar la fila duplicada de «Longitud»;
- entero → texto es CastSafe (R-TYPE-004), no «conversión con riesgo»;
- la clave primaria no es «conflicto de tipo».

- **Bloque C.** Describir el protocolo del experto (sección 7 del protocolo): alcance fijado
  antes, verdad construida solo con el DDL y la documentación, sellada con un commit, ejecución
  con los conectores reales, condición asistida cronometrada y adjudicación de cada FP y FN
  (`error_audd`, `error_verdad` o `ambiguo`). Resolver el `[PENDIENTE]` sobre si participa una
  segunda persona de Grow Venta.

- **Procedimiento.** Alinear el párrafo del procedimiento con la sección 10 del protocolo:
  1. preparar las bases en MySQL (Laragon) y etiquetar la versión;
  2. ejecutar el bloque A (`cross.py`);
  3. generar y ejecutar el bloque B (`mutate.py`, `mutations.py`; luego `mutations.py --domain pos`);
  4. ejecutar el bloque C (`manual.py`);
  5. calcular PRM e IAC;
  6. generar el reporte (`report.py`).

  Cada corrida queda en su propia carpeta con `manifest.json` (commit, etiqueta, hash del
  binario, hash del catálogo, entorno y hash de la IR de cada base).

### 3.3 Sección 5.4 (caso Grow)

Con la especificación simétrica, el caso Grow Travel contra Grow Venta ya no da 17 / 145 / 4,
sino **17 coincidencias, 176 exclusivos y 12 conflictos**. Los exclusivos cuentan campos de
ambos lados, incluidos los de tablas sin equivalente. Actualizar las cifras y explicar el cambio.

### 3.4 Sección 5.6.2 (resultados)

Cambiar la Tabla 21 de niveles a bloques (A, B, C) y agregar:

- PCD por tipo de conflicto y por nivel de dificultad (secciones 1.1 y 1.2);
- F1 de elementos afectados y por categoría, sin y con dominio (sección 1.3);
- SYM y FCI del bloque A;
- PRM desglosado por requerimiento;
- IAC-1 e IAC-2 a la fecha de corte;
- en el bloque C: resultados por par, número de versiones de la verdad y de correcciones
  `error_verdad`, y TRI por par.

### 3.5 Sección 5.6.4 (limitaciones y discusión)

Usar el diagnóstico de la sección 2.7 y agregar a las cuatro limitaciones ya previstas:

- **El emparejamiento de nombres es el cuello de botella**, no la detección de conflictos:
  PCD es 100 % en el nivel fácil y 0 % en el difícil, y F1 de elementos afectados es 0.61.
- **El dominio aporta poco** con el diccionario actual (F1 de 0.61 a 0.66).
- La precisión es alta: ningún distractor ni renombrado sin relación se emparejó, y ningún
  cambio equivalente produjo conflicto.
- El bloque B es una verificación con cambios controlados; la validez práctica la da el
  bloque C.
- E4 (CHECK y pertenencia a la clave primaria) no está implementado; R-CONS-002 y R-CONS-003
  solo son alcanzables desde el catálogo.
- Northwind se usa en SQLite porque no hay DDL MySQL en el repositorio.
- IAC-2 es 0 a la fecha de corte.

---

## 4. Ajustes pendientes en el repositorio (por prioridad)

1. **Corrida oficial sobre MySQL** (protocolo, secciones 4 y 10).
   - Encender MySQL en Laragon y definir `AUDD_MYSQL`.
   - `python evaluation/suite/setup_mysql.py` y registrar los hashes en `datasets.toml`.
   - `python evaluation/mutate.py --all --backend mysql`
   - Ejecutar el resto de la sección 10 del protocolo, `mutations.py --domain pos` y `report.py`.
   - Crear la etiqueta `v0.9.0-eval` en el commit evaluado.
2. **Bloque C** (protocolo, sección 7).
   - Fijar `scope_a` y `scope_b` (marcados `[AJUSTAR]`) con los nombres que reporta la fuente.
   - Construir y sellar la verdad de M1 a M3 con `manual.py` y hacer commit antes de ejecutar
     AUDD.
   - Registrar los tiempos para obtener TRI.
3. **`adoption.py`:** excluir las cuentas bot (`type == "Bot"`) de IAC-2.
4. **Catálogo frente a la tabla 6.4:** agregar R-LEN-002, R-NULL-002 y R-NORM-002, o declarar
   en la tesis que la versión 0 tiene 14 reglas.
5. **Mejoras opcionales del emparejamiento** (antes de la corrida oficial, registrándolas en la
   sección 13 del protocolo): diccionario de abreviaturas y quitar prefijos heredados. Si se
   dejan como están, se reportan como limitación.
6. **Limpieza.**
   - `evaluation/VALIDATION_REPORT.md` y `evaluation/results/` antiguos muestran cifras
     inválidas (PCD de 12 %): marcarlos como obsoletos o retirarlos.
   - Verificar las licencias marcadas `[VERIFICAR]` en `config/suite.toml`.

Resuelto el 2026-10-19 (sección 13 del protocolo): colisión de normalización dentro de cada
esquema, dominio como estrategia de emparejamiento, mutaciones con tres niveles, F1 por
categoría y nombres de los exclusivos de B.
