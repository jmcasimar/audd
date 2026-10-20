# Protocolo de evaluación de AUDD

Versión 1.1 · octubre de 2026

Este documento se fija **antes** de ejecutar la evaluación oficial. Cualquier cambio posterior
se registra en la sección 13 con fecha y motivo. El significado detallado de cada métrica y
su lectura para la tesis están en `AJUSTES_REDACCION_TESIS.md`.

**Contenido**

1. Lógica de la evaluación
2. Estructura de carpetas
3. Especificación de conflictos
4. Preparación del entorno (incluye la importación a MySQL con Laragon)
5. Bloque A: cada base contra sí misma y contra las demás
6. Bloque B: prueba controlada de mutaciones
7. Bloque C: prueba real con verdad elaborada por un experto
8. Regla de conteo
9. Indicadores
10. Ejecución completa
11. Qué va a la tesis
12. Anexo: problemas de la suite anterior
13. Registro de cambios

---

## 1. Lógica de la evaluación

La evaluación sigue tres etapas, en este orden. Cada etapa responde una pregunta distinta y
solo tiene sentido si la anterior salió bien.

| Etapa | Pregunta que responde | Qué se compara | Verdad de referencia | Analogía |
|---|---|---|---|---|
| **A. Matriz cruzada** | ¿La herramienta funciona de forma estable, repetible y sin inventar diferencias? | Cada base contra sí misma (identidad) y contra todas las demás, en ambos sentidos | No hace falta: en la identidad la respuesta correcta se conoce de antemano (cero diferencias) | Calibrar el instrumento con una medida conocida |
| **B. Mutaciones controladas** | ¿Qué tipos de diferencias detecta el algoritmo y dónde está su límite? | Una base contra una variante suya con cambios introducidos a propósito | Automática: el generador registra cada cambio que hizo | Experimento de laboratorio con variables controladas |
| **C. Pares reales con verdad de experto** | ¿Sirve AUDD en un caso real y cuánto tiempo ahorra? | Dos sistemas reales distintos (p. ej. UniCenta contra Grow Venta) | Manual: un experto analiza los esquemas antes de ver AUDD | Prueba de campo |

### 1.1 Por qué primero cada base contra sí misma

Comparar una base consigo misma es el **control negativo** de la evaluación: la respuesta
correcta es «todo coincide, no hay exclusivos ni conflictos». Si AUDD reporta algo en ese
caso, es un error seguro (un falso positivo que no depende de ninguna interpretación). Esta
prueba también verifica que:

- los adaptadores leen las seis bases sin fallar (robustez);
- tres ejecuciones con las mismas entradas dan salidas idénticas (reproducibilidad);
- invertir A y B da el mismo resultado (simetría), que es lo que exige un esquema unificado.

Solo después de pasar este control tiene sentido medir qué detecta AUDD (bloque B) y si sirve
en la práctica (bloque C).

### 1.2 Por qué una prueba sintética antes de la real

En una comparación real nadie sabe con certeza cuántas diferencias hay; la verdad depende del
criterio de una persona. En la prueba de mutaciones **se sabe exactamente** qué cambió, porque
el generador lo introdujo y lo registró. Eso permite:

- medir la detección por tipo de conflicto y por nivel de dificultad;
- distinguir si un fallo es del adaptador (la diferencia se perdió al leer el esquema) o del
  núcleo (la diferencia llegó y no se detectó);
- repetir el experimento con las mismas semillas.

Por eso es la **fuente oficial** de PCD, PRE, PSR y F1 para el criterio de TRL 4 (validación
en entorno controlado). Su limitación es que es una **verificación**: mide si el algoritmo
cumple su especificación con cambios conocidos, no si resuelve cualquier caso real.

### 1.3 Por qué una prueba real con un experto al final

El bloque C enfrenta a AUDD con lo que no se puede sintetizar: nombres con historia, tablas que
se dividen o se fusionan, convenciones de cada equipo y cambios de significado. La verdad la
construye un experto **antes** de ejecutar AUDD y queda sellada con un commit. Es la única
fuente de TRI (ahorro de tiempo) y la evidencia de comportamiento en esquemas reales. Se
reporta aparte y no se promedia con el bloque B, porque tiene pocos pares y una verdad humana.

### 1.4 Resumen de los bloques

| Bloque | Pares | Verdad | Indicadores oficiales | Indicadores de apoyo |
|---|---|---|---|---|
| A | 6 bases: 6 de identidad + 30 dirigidas = 36 | No requiere | ROB, REP, SYM, FCI | Mediana de tiempo |
| B | 5 bases × 10 semillas = 50 | Automática, validada contra la IR | **PCD, PRE, PSR, F1** | PCD por tipo y nivel, F1 por categoría, ROB, REP |
| C | 3 obligatorios + 2 opcionales | Manual, ciega y sellada | **TRI** | PCD, PRE, PSR y F1 en esquemas reales |

**Bases (6):** chinook, employees, northwind, unicenta, growtravel y growventa.

**Pares del bloque C:**

| Id | A (origen) | B (destino) | Motivo |
|---|---|---|---|
| M1 | growtravel | growventa | Caso documentado en 5.4 |
| M2 | unicenta | growventa | Migración real entre dos puntos de venta: el caso de uso principal |
| M3 | northwind | growventa | Dominio comercial con productos, clientes y pedidos |
| M4 (opcional) | chinook | northwind | Dos sistemas públicos; un tercero puede replicar la verdad |
| M5 (opcional) | employees | northwind | Dominios casi ajenos: control de falsos positivos |

---

## 2. Estructura de carpetas

```
evaluation/
  PROTOCOLO_EVALUACION.md       este documento
  AJUSTES_REDACCION_TESIS.md    significado de las métricas y ajustes de redacción
  config/
    suite.toml                  bases, pares manuales, alcance y opciones de AUDD
    snapshots.lock.json         hash de la IR de cada base (se crea en la preparación)
  bases/<id>/                   DDL versionado de cada base y, tras la preparación, schema.sql
  mutate.py                     generador del bloque B
  suite/
    common.py                   reglas de conteo compartidas
    mysql.py                    acceso a MySQL con los clientes mysql y mysqldump
    setup_mysql.py              importa las bases a MySQL y genera schema.sql
    cross.py                    bloque A
    mutations.py                bloque B
    validate_truth.py           valida la verdad del bloque B contra la IR
    manual.py                   bloque C (init, freeze, run, score, tri)
    score.py                    cálculo de VP, FP, FN y de los indicadores
    report.py                   reporte final y contraste con las metas
  manual/
    _plantilla/                 verdad.csv, tiempos.csv, adjudicacion.csv, INSTRUCCIONES.md
    M2_unicenta_growventa/      una carpeta por par manual (la crea manual.py init)
  generated/                    pares del bloque B
  results/<fecha>_<commit>/     una carpeta por corrida completa (no se sobrescribe)
```

---

## 3. Especificación de conflictos

La verdad de referencia se construye con esta especificación escrita, **no** con la salida de
AUDD. Se adoptó la **especificación de unificación simétrica**: AUDD produce un esquema
unificado C a partir de A y B, así que toda diferencia que obligue a decidir cómo queda C es un
conflicto; la severidad distingue lo seguro de lo riesgoso. Con ella, invertir A y B da el
mismo resultado.

| Diferencia entre A y B (mismo campo) | ¿Conflicto? | Tipo | Sugerencia esperada |
|---|---|---|---|
| Int32 vs Int64 | Sí (baja) | TypeIncompatible | CastSafe (R-TYPE-001) |
| Entero vs Decimal | Sí (baja) | TypeIncompatible | CastSafe (R-TYPE-002) |
| Float vs Decimal | Sí (media) | TypeIncompatible | CastRisky (R-TYPE-003) |
| Entero (A) vs texto (B) | Sí | TypeIncompatible | CastSafe hacia texto (R-TYPE-004) |
| Texto (A) vs entero (B) | Sí | TypeIncompatible | CastRisky (R-TYPE-005) |
| Boolean vs entero | Sí (media) | TypeIncompatible | CastSafe (R-TYPE-006) |
| Date (A) vs DateTime/Timestamp (B) | Sí | TypeIncompatible | CastSafe (R-TYPE-007) |
| DateTime/Timestamp (A) vs Date (B) | Sí | TypeIncompatible | CastRisky (R-TYPE-008) |
| String vs Text; DateTime vs Timestamp | **No** | — | — |
| Longitud máxima distinta (ambas definidas) | Sí | LengthMismatch | PreferType (R-LEN-001) |
| Nulabilidad distinta (en cualquier sentido) | Sí | NullabilityMismatch | PreferType (R-NULL-001) |
| UNIQUE en un solo lado (como restricción o clave única de una columna) | Sí | ConstraintMismatch | PreferType (R-CONS-001) |
| Dos campos de la misma tabla que se normalizan igual en un esquema (`cliente_id` y `ClienteId`), si el otro esquema no tiene los mismos nombres | Sí | NormalizationCollision | RenameField (R-NORM-001) |
| CHECK distinto; pertenencia distinta a la clave primaria | No se compara (E4 no implementado) | — | — |
| Mismo nombre, distinto significado | Fuera de alcance del núcleo | Semantic | Se registra; no cuenta para PCD |

Cambios del núcleo que implementan esta especificación (sección 13): E1 (conversiones seguras y
con pérdida como conflicto con severidad), E2 (nulabilidad en ambos sentidos), E3 (UNIQUE como
clave de una columna), colisión dentro de cada esquema y E5 (pruebas de aceptación, incluida la
de simetría).

---

## 4. Preparación del entorno (una sola vez)

### 4.1 Requisitos

- Rust estable (la evaluación se preparó con rustc 1.93) y Python 3.11 o superior.
- Laragon con MySQL 8 en marcha. Esta instalación usa MySQL 8.4.3; sus clientes `mysql` y
  `mysqldump` están en `C:\laragon\bin\mysql\mysql-8.4.3-winx64\bin` y ya se encuentran en el
  PATH. Laragon incluye además HeidiSQL, que sirve para revisar las bases importadas.
- El repositorio en la rama o etiqueta que se va a evaluar.

### 4.2 Compilar AUDD

Desde la raíz del repositorio, en PowerShell:

```powershell
cargo test --workspace
cargo build --release -p audd-cli
```

El binario queda en `target\release\audd.exe`, que es el que usa la suite
(`[audd].binary` en `config/suite.toml`).

### 4.3 Encender MySQL en Laragon y preparar el usuario

1. Abrir Laragon y pulsar **Start All** (o iniciar solo MySQL). El indicador de MySQL debe
   quedar en verde.
2. Comprobar desde PowerShell que el servidor responde. En una instalación nueva de Laragon,
   el usuario `root` no tiene contraseña:

   ```powershell
   mysql -u root -h 127.0.0.1 -e "SELECT VERSION(); SHOW VARIABLES LIKE 'lower_case_table_names';"
   ```

   Si `root` tiene contraseña, agregar `-p` y escribirla cuando la pida.
3. **Recomendado:** crear un usuario solo para la evaluación, para no usar `root`. En la
   consola de MySQL (`mysql -u root -h 127.0.0.1`) o en la pestaña de consultas de HeidiSQL:

   ```sql
   CREATE USER 'audd'@'localhost' IDENTIFIED BY 'una_clave_local';
   CREATE USER 'audd'@'127.0.0.1' IDENTIFIED BY 'una_clave_local';
   GRANT ALL PRIVILEGES ON *.* TO 'audd'@'localhost';
   GRANT ALL PRIVILEGES ON *.* TO 'audd'@'127.0.0.1';
   FLUSH PRIVILEGES;
   ```

   Los permisos globales son necesarios porque la suite crea y borra bases de datos (las de
   las bases y las temporales `audd_m_*` del bloque B). Es un servidor local de laboratorio.
4. Definir la credencial en la sesión de PowerShell. **Nunca** se escribe en el repositorio:
   `config/suite.toml` y los `pair.json` guardan `${AUDD_MYSQL}`, no la clave.

   ```powershell
   $env:AUDD_MYSQL = "audd:una_clave_local@127.0.0.1:3306"
   # Con root sin contraseña: $env:AUDD_MYSQL = "root@127.0.0.1:3306"
   ```

   La variable dura lo que dure la ventana de PowerShell; hay que definirla de nuevo en cada
   sesión.

**Nota sobre mayúsculas.** En Windows, MySQL guarda los nombres de tabla en minúsculas
(`lower_case_table_names = 1`): la tabla `Album` del DDL de Chinook queda como `album`. AUDD
empareja sin distinguir mayúsculas, pero el experto debe escribir en `verdad.csv` los nombres
tal como los reporta MySQL (sección 7.4).

### 4.4 Importar las bases a MySQL

Se importan las cuatro bases que tienen DDL MySQL. Grow Venta y Northwind se usan en su forma
nativa SQLite (`bases/growventa/growventa.db` y `bases/northwind/northwind.db`) y no se
importan.

| Id | Base de datos en MySQL | DDL que se carga |
|---|---|---|
| chinook | `chinook` | `bases/chinohook/Chinook_MySql.sql` |
| employees | `employees` | `bases/employees/employees.sql` |
| unicenta | `unicentaopos` | `bases/unicenta-opos/MySQL-create.sql` |
| growtravel | `growtravel` | `bases/growtravel/growvtravel.sql` |

**Opción A (recomendada): importación automática.**

```powershell
python evaluation/suite/setup_mysql.py
```

Para cada base, el script:

1. borra y vuelve a crear la base de datos indicada en `source` de `config/suite.toml`;
2. quita del DDL las sentencias `CREATE DATABASE`, `DROP DATABASE` y `USE`, para que todo se
   cargue en esa base (el DDL de Chinook y el de Employees traen las suyas);
3. carga el DDL con `mysql`, con las claves foráneas desactivadas durante la carga;
4. guarda la estructura exacta con `mysqldump --no-data` en `bases/<id>/schema.sql`;
5. imprime el sha256 de `schema.sql`, que se registra en `datasets.toml`.

Para importar solo algunas: `python evaluation/suite/setup_mysql.py --only chinook employees`.

**Opción B: importación manual con HeidiSQL o con la consola.**

1. Crear la base: `mysql -u root -h 127.0.0.1 -e "CREATE DATABASE unicentaopos CHARACTER SET utf8mb4"`.
2. Cargar el DDL: en HeidiSQL, seleccionar la base y usar **Archivo > Cargar archivo SQL** y
   ejecutar; o en la consola: `Get-Content evaluation\bases\unicenta-opos\MySQL-create.sql | mysql -u root -h 127.0.0.1 unicentaopos`.
   Si el archivo trae `CREATE DATABASE` o `USE` con otro nombre, quitarlos antes.
3. Generar la estructura: `mysqldump -u root -h 127.0.0.1 --no-data --skip-comments unicentaopos > evaluation\bases\unicenta\schema.sql`.

**Verificar que AUDD lee la base con el conector real** (no con el archivo DDL):

```powershell
.\target\release\audd.exe inspect --source "db:mysql://$env:AUDD_MYSQL/unicentaopos" --out unicenta.ir.json
```

La salida debe indicar el número de entidades (UniCenta tiene 41 tablas). Repetir con
`chinook`, `employees` y `growtravel`.

### 4.5 Congelar las entradas y etiquetar

1. Registrar en `evaluation/datasets.toml` los sha256 que imprimió `setup_mysql.py`.
2. Verificar las licencias marcadas `[VERIFICAR]` en `config/suite.toml` (campo `origen`). Solo
   se usa la estructura de las bases, nunca sus datos.
3. Hacer commit de `bases/*/schema.sql` y de `datasets.toml`.
4. Crear la etiqueta de la versión evaluada: `git tag v0.9.0-eval`.
5. La primera corrida del bloque A guarda la IR de cada base. Copiar sus hashes
   (`manifest.json` → `snapshots`) a `config/snapshots.lock.json`. A partir de ahí, si una
   base cambia, la suite avisa.

---

## 5. Bloque A: cada base contra sí misma y contra las demás

**Objetivo.** Comprobar que la herramienta es estable y no inventa diferencias, antes de medir
qué detecta.

**Qué se ejecuta.** Las 6 bases se comparan:

- **contra sí mismas** (6 comparaciones de identidad): el control negativo;
- **contra todas las demás en ambos sentidos** (30 comparaciones dirigidas): robustez con
  esquemas de tamaños y dominios distintos, y simetría.

Cada comparación se repite 3 veces con `--deterministic`.

```powershell
python evaluation/suite/cross.py --out evaluation/results/<id>
```

**Qué produce y cómo se lee.**

| Salida | Indicador | Valor esperado | Si no se cumple |
|---|---|---|---|
| `cross_indicators.json` | ROB | 100 % de comparaciones sin error y con JSON válidos | Un adaptador no lee una base o el núcleo falla con un esquema |
| | REP | 100 % de comparaciones con salidas idénticas en 3 ejecuciones | Hay no determinismo (orden de listas, marcas de tiempo) |
| | FCI | 0 conflictos y 0 exclusivos en las 6 identidades | Falso positivo seguro: el emparejamiento o la detección inventa diferencias |
| `symmetry.json` | SYM | 100 % de los 15 pares no ordenados simétricos | Algún criterio depende de qué esquema es A y cuál es B |
| `cross.csv` | Tiempo | Mediana por comparación | Referencia de rendimiento |

Las 30 comparaciones dirigidas no tienen verdad de referencia: sus coincidencias y conflictos
no se puntúan; sirven para robustez, reproducibilidad y simetría.

---

## 6. Bloque B: prueba controlada de mutaciones

**Objetivo.** Medir sintéticamente el alcance actual del algoritmo: qué diferencias detecta, con
qué precisión, qué sugiere y en qué punto deja de funcionar.

### 6.1 Cómo se generan los pares

Para cada una de las 5 bases (chinook, employees, northwind, unicenta, growtravel) y cada
semilla de 1000 a 1009:

1. `mutate.py` lee la IR de la base con `audd inspect`.
2. Genera **A** renderizando esa IR como DDL MySQL, sin cambios.
3. Genera **B** aplicando de 5 a 10 mutaciones elegidas al azar con la semilla fija. Como A y B
   salen de la misma IR, **solo difieren en lo mutado**.
4. Escribe `truth.json`: el registro de mutaciones traducido a lo que AUDD debería reportar,
   según la especificación de la sección 3.
5. Con el backend `mysql` (oficial) carga A y B en bases temporales `audd_m_<par>_a` y
   `audd_m_<par>_b`, para que AUDD las lea con el conector real; con el backend `ddl` deja los
   archivos `a.sql` y `b.sql` (alternativa sin servidor).

### 6.2 Operadores y niveles de dificultad

| Nivel | Operador | Ejemplo | Resultado correcto |
|---|---|---|---|
| Fácil | Cambiar tipo | `INT` → `BIGINT`, `VARCHAR` → `INT` | Conflicto de tipo con la sugerencia de la sección 3 |
| Fácil | Cambiar longitud | `VARCHAR(50)` → `VARCHAR(100)` | Conflicto de longitud |
| Fácil | Invertir nulabilidad | `NULL` ↔ `NOT NULL` (no PK) | Conflicto de nulabilidad |
| Fácil | Agregar o quitar UNIQUE | | Conflicto de restricción |
| Fácil | Agregar o quitar campo o tabla | | Exclusivo en B o en A |
| Fácil | Renombrar por estilo o plural | `cliente_id` → `ClienteId`, `customer` → `customers` | Coincidencia |
| Medio | Renombrar con prefijo heredado | `email` → `cus_email` | Coincidencia |
| Medio | Cambio combinado | renombrar **y** cambiar tipo, longitud o nulabilidad | Coincidencia y conflicto |
| Medio | Cambio equivalente | `VARCHAR` → `TEXT`, `DATETIME` ↔ `TIMESTAMP` | **Ningún** conflicto |
| Medio | Colisión de normalización | agregar `ClienteId` junto a `cliente_id` | Conflicto de colisión |
| Difícil | Abreviatura | `quantity` → `qty` | Coincidencia |
| Difícil | Sinónimo o traducción | `phone` → `telephone`, `status` → `estado` | Coincidencia |
| Difícil | Renombrado sin relación | `price` → `fld_1234` | Exclusivo en A y en B (límite conocido) |
| Difícil | Distractor | `created_at` → `created_by` | **Ninguna** coincidencia |

Cada coincidencia de la verdad indica su **categoría** (`style`, `plural`, `affix`,
`abbreviation`, `synonym`, `translation`, o `unchanged` si el campo no cambió) y si el
dominio `pos` la cubre. Cada conflicto indica su **nivel**.

### 6.3 Validación de la verdad contra la IR

Antes de puntuar, `mutations.py` lee la IR de A y de B con `audd inspect` y `validate_truth.py`
revisa cada conflicto de la verdad **sin usar el motor de comparación**:

| Estado | Significado | ¿Cuenta para PCD? |
|---|---|---|
| `observable` | La diferencia está en la IR: llegó al núcleo | Sí |
| `not_observable` | El adaptador perdió la diferencia al leer el esquema | No; se reporta como limitación del adaptador, con su número |
| `label_review` | Hay una diferencia, pero de otro tipo que la etiqueta | No; se revisa la etiqueta a mano y se regenera |

No es circular: comprueba que la mutación llegó al núcleo, no que el núcleo la detecte.

### 6.4 Ejecución

```powershell
# Generar los 50 pares (backend oficial: carga A y B en MySQL)
python evaluation/mutate.py --all --backend mysql

# Corrida oficial, sin dominio
python evaluation/suite/mutations.py --pairs evaluation/generated --out evaluation/results/<id>

# Análisis adicional: aporte del dominio pos
python evaluation/suite/mutations.py --pairs evaluation/generated --out evaluation/results/<id>_pos --domain pos
```

**Agregación micro.** Se suman VP, FP y FN de todos los pares y luego se divide. Se reportan
PCD global, por tipo de conflicto y por nivel; F1 global, de elementos afectados y por
categoría.

---

## 7. Bloque C: prueba real con verdad elaborada por un experto

**Objetivo.** Medir el comportamiento de AUDD en comparaciones reales y el tiempo que ahorra.
La verdad de referencia es el análisis de un experto, hecho **antes** de ver cualquier salida
de AUDD y sellado con un commit.

### 7.1 Roles y cegamiento

- **Evaluador 1 (experto que construye la verdad):** conoce los sistemas; elabora la verdad y
  mide el tiempo de la condición manual.
- **Evaluador 2 (desarrollador de Grow Venta, sin acceso previo a la verdad):** mide la
  condición asistida. Con dos personas se evita que quien hizo el análisis manual reutilice su
  memoria en la condición asistida.
- **Si solo participa el autor:** alternar el orden entre pares (por ejemplo, condición asistida
  primero en M3 y manual primero en M2), dejar al menos 7 días entre las dos condiciones del
  mismo par y declarar el sesgo de aprendizaje en las limitaciones.
- Nadie abre salidas de AUDD del par (`run_0/`, `report.md`) antes de sellar la verdad.
  `manual.py freeze` se niega a sellar si ya existen salidas de AUDD sin una verdad previa.

### 7.2 Materiales permitidos

| Permitido en la condición manual | No permitido |
|---|---|
| `bases/<id>/schema.sql` de cada base (generado en 4.4); para Grow Venta, `bases/growventa/grow_venta.sql`; para Northwind, la estructura de `northwind.db` vista en DBeaver o HeidiSQL | Cualquier salida de AUDD (`diff.json`, `report.md`, `unified_schema.json`) |
| Diccionarios de datos, documentación de los sistemas, conocimiento del negocio | Resultados de otras herramientas de comparación automática |
| Editor de texto, hoja de cálculo, HeidiSQL o DBeaver | |

### 7.3 Paso 1: crear la carpeta del par

```powershell
python evaluation/suite/manual.py init M2_unicenta_growventa
```

Crea `evaluation/manual/M2_unicenta_growventa/` con `verdad.csv`, `tiempos.csv`,
`adjudicacion.csv` e `INSTRUCCIONES.md`.

### 7.4 Paso 2: fijar el alcance

Los sistemas reales tienen decenas de tablas (UniCenta tiene 41) que no siempre corresponden al
dominio comparado. El alcance limita la verdad a las tablas del dominio central y **se fija
antes de construir la verdad; no se modifica después**.

1. Obtener los nombres exactos de las tablas como los ve AUDD:

   ```powershell
   .\target\release\audd.exe inspect --source "db:mysql://$env:AUDD_MYSQL/unicentaopos" --out unicenta.ir.json
   .\target\release\audd.exe inspect --source "db:sqlite://evaluation/bases/growventa/growventa.db" --out growventa.ir.json
   ```

   O, en MySQL: `mysql -u root -h 127.0.0.1 -e "SHOW TABLES" unicentaopos`.
2. Elegir las tablas del dominio central (en M2: productos, categorías, clientes, tickets o
   ventas, líneas de ticket, existencias, pagos e impuestos).
3. Copiarlas **exactamente** (con las mayúsculas que reporta la fuente) en `scope_a` y
   `scope_b` del par en `config/suite.toml`. `[]` significa el esquema completo.
4. Hacer commit de `config/suite.toml` con el mensaje «Alcance del par M2».

### 7.5 Paso 3: construir la verdad (condición manual, cronometrada)

1. Anotar la hora de inicio en `tiempos.csv`:

   ```
   condicion,evaluador,inicio,fin,segundos_audd,nota
   manual,evaluador1,2026-10-25T10:05:00,,,
   ```

2. Llenar `verdad.csv`, una fila por elemento, con estas columnas:

   | Columna | Contenido |
   |---|---|
   | `tipo` | `coincidencia`, `exclusivo_a`, `exclusivo_b` o `conflicto` |
   | `entidad_a`, `campo_a` | Tabla y columna en A (vacías en `exclusivo_b`) |
   | `entidad_b`, `campo_b` | Tabla y columna en B (vacías en `exclusivo_a`) |
   | `tipo_conflicto` | Solo en conflictos: `TypeIncompatible`, `NullabilityMismatch`, `ConstraintMismatch`, `LengthMismatch`, `NormalizationCollision` o `Semantic` |
   | `sugerencia_esperada` | Solo en conflictos: `CastSafe`, `CastRisky`, `PreferType`, `RenameField`, `NoSuggestion` o vacío |
   | `nota` | Justificación breve; obligatoria en los casos dudosos |

   **Reglas para cada tipo de fila:**

   - **`coincidencia`:** una columna de A que significa lo mismo que una de B, aunque se llamen
     distinto (`pricesell` ↔ `precio`). Las tablas también: una fila con el campo vacío
     (`products` ↔ `producto`).
   - **`exclusivo_a` / `exclusivo_b`:** una columna sin equivalente en el otro lado. Si **toda
     la tabla** no tiene equivalente, se escribe **una sola fila con el campo vacío**, no una
     por columna.
   - **`conflicto`:** una coincidencia cuya definición difiere. El tipo y la sugerencia se
     deciden con la tabla de la sección 3, no con lo que el experto «cree que AUDD dirá». Un
     conflicto también cuenta como coincidencia: no hace falta repetirlo como `coincidencia`.
     Si una columna tiene dos diferencias (tipo y nulabilidad), se escriben dos filas de
     conflicto.
   - **`Semantic`:** mismo nombre o par emparejado con distinto significado (por ejemplo, `curdebt`
     y `saldo` miden cosas distintas). Se registra; no cuenta para PCD y documenta un límite.

   **Guía para decidir el tipo de conflicto** (leer el DDL de ambos lados):

   | Si en el DDL ves… | Escribe |
   |---|---|
   | `INT` contra `BIGINT` | `TypeIncompatible`, `CastSafe` |
   | `DOUBLE` contra `DECIMAL(10,2)` | `TypeIncompatible`, `CastRisky` |
   | `VARCHAR` (A) contra `INT` (B) | `TypeIncompatible`, `CastRisky` |
   | `INT` (A) contra `VARCHAR` (B) | `TypeIncompatible`, `CastSafe` |
   | `DATETIME` (A) contra `DATE` (B) | `TypeIncompatible`, `CastRisky` |
   | `VARCHAR(50)` contra `VARCHAR(100)` | `LengthMismatch`, `PreferType` |
   | `VARCHAR` contra `TEXT`, o `DATETIME` contra `TIMESTAMP` | **Ningún** conflicto |
   | `NOT NULL` en un lado y `NULL` en el otro | `NullabilityMismatch`, `PreferType` |
   | `UNIQUE` en un solo lado | `ConstraintMismatch`, `PreferType` |
   | Dos columnas de la misma tabla que solo difieren en guiones bajos o mayúsculas | `NormalizationCollision`, `RenameField` |
   | Diferencias en `CHECK`, valores por defecto o claves foráneas | No se registran (fuera de alcance) |

   **Ejemplo de filas:**

   ```
   tipo,entidad_a,campo_a,entidad_b,campo_b,tipo_conflicto,sugerencia_esperada,nota
   coincidencia,products,,producto,,,,"misma entidad"
   coincidencia,products,name,producto,nombre,,,"mismo significado, nombre distinto"
   conflicto,products,pricesell,producto,precio,TypeIncompatible,CastRisky,"DOUBLE en A, DECIMAL en B"
   conflicto,customers,email,cliente,correo,NullabilityMismatch,PreferType,
   exclusivo_a,products,stockcost,,,,,"campo sin equivalente en B"
   exclusivo_a,attributes,,,,,,"tabla completa sin equivalente"
   exclusivo_b,,,producto,codigo_barras,,,
   conflicto,customers,curdebt,cliente,saldo,Semantic,,"mismo papel, distinto significado"
   ```

   **Errores frecuentes:** escribir los nombres con otras mayúsculas o sin el prefijo que tiene
   la fuente; poner una fila por columna de una tabla exclusiva; registrar como conflicto un
   `VARCHAR` contra `TEXT`; dejar sin nota una decisión dudosa.

3. Anotar la hora de fin en la misma fila de `tiempos.csv`.

### 7.6 Paso 4: sellar la verdad y hacer commit de inmediato

```powershell
python evaluation/suite/manual.py freeze M2_unicenta_growventa
git add evaluation/manual/M2_unicenta_growventa
git commit -m "Verdad de referencia M2 (ciega, antes de ejecutar AUDD)"
```

`freeze` valida cada fila de `verdad.csv` (tipos permitidos, columnas obligatorias), genera
`truth.json` y guarda en `truth.lock.json` el sha256 del CSV y la fecha. **El commit es la
prueba de que la verdad existía antes de ejecutar AUDD.**

### 7.7 Paso 5: ejecutar AUDD con los conectores reales

```powershell
python evaluation/suite/manual.py run M2_unicenta_growventa
```

Ejecuta `audd compare` 3 veces con las fuentes de `config/suite.toml`: en M2, el **conector
MySQL** contra `unicentaopos` y el conector SQLite contra `growventa.db`. Se niega si la verdad
no está sellada. Guarda `run_0/` a `run_2/` y la IR de B (`ir_b.json`).

### 7.8 Paso 6: condición asistida (cronometrada, evaluador 2)

1. Anotar la hora de inicio en `tiempos.csv` con `condicion = asistido`.
2. Abrir `run_0/report.md` y producir el **mismo documento de correspondencias** que en la
   condición manual: confirmar, corregir o completar lo que reporta AUDD.
3. Anotar la hora de fin y, en `segundos_audd`, el tiempo de ejecución de AUDD: el valor
   `seconds` que `manual.py run` imprime para `run_0`.

```
manual,evaluador1,2026-10-25T10:05:00,2026-10-25T12:20:00,,
asistido,evaluador2,2026-10-27T09:00:00,2026-10-27T09:40:00,1.2,
```

### 7.9 Paso 7: puntuar y adjudicar

```powershell
python evaluation/suite/manual.py score M2_unicenta_growventa
```

Calcula PCD, PRE, PSR y F1 contra la verdad y lista en `score.json` (`detail`) cada falso
negativo y cada falso positivo. Para cada uno se escribe una fila en `adjudicacion.csv`:

```
id,tipo_error,entidad,campo,tipo_conflicto,decision,justificacion
1,FN,customers,curdate,TypeIncompatible,error_audd,"AUDD no emparejó curdate con fecha_alta"
2,FP,products,code,ConstraintMismatch,error_verdad,"el UNIQUE de B se omitió en la verdad"
```

| Decisión | Cuándo | Efecto |
|---|---|---|
| `error_audd` | El núcleo se equivocó | Se queda como error |
| `error_verdad` | La verdad estaba mal (el experto omitió o etiquetó mal algo) | Se corrige `verdad.csv`, se vuelve a ejecutar `freeze` (queda una nueva versión en el historial) y `score` |
| `ambiguo` | Ambas lecturas son defendibles | Se conserva la verdad original |

Se reportan el número de versiones de la verdad y el número de correcciones. Una verdad con
muchas correcciones `error_verdad` indica que el análisis manual tiene errores, lo cual también
es un hallazgo para la tesis.

### 7.10 Paso 8: calcular TRI

Después de terminar todos los pares:

```powershell
python evaluation/suite/manual.py tri
```

Lee `tiempos.csv` de cada par y calcula `TRI = (t_manual − t_asistido) / t_manual × 100`, donde
el tiempo asistido incluye la revisión y la ejecución de AUDD. El resultado queda en
`evaluation/manual/tri.json`. Solo M1 a M3 entran a la meta de TRI.

### 7.11 Lista de verificación por par

- [ ] Alcance fijado en `suite.toml` y con commit, antes de la verdad.
- [ ] Hora de inicio y fin de la condición manual en `tiempos.csv`.
- [ ] `verdad.csv` completo, con nota en los casos dudosos.
- [ ] `manual.py freeze` sin errores y commit inmediato.
- [ ] `manual.py run` con los conectores reales (MySQL o SQLite según la base).
- [ ] Condición asistida cronometrada por el evaluador 2 (o con 7 días de separación).
- [ ] `manual.py score` y una fila en `adjudicacion.csv` por cada FP y FN.
- [ ] Commit final del par.

---

## 8. Regla de conteo (igual en todos los bloques)

- **Conflicto VP:** coinciden entidad, campo y tipo, con nombres normalizados (minúsculas, sin
  guiones bajos, espacios ni comillas). Se acepta el nombre del lado A o del lado B. Cada
  conflicto de la verdad se empareja una sola vez.
- **Coincidencia VP:** la clave del lado A coincide y el campo de B es el correcto (se resuelve
  con la IR de B). Emparejar con el campo equivocado de B cuenta como un FP y un FN.
- **Exclusivo:** se compara a la granularidad que reporta AUDD: una tabla completa sin
  equivalente es un elemento; un campo sin pareja dentro de una tabla emparejada es otro. Los
  exclusivos de B se nombran con la tabla de B.
- **Alcance:** en el bloque C solo cuentan las tablas de `scope_a` y `scope_b`.

---

## 9. Indicadores

El significado de cada indicador, lo que representa dentro del algoritmo y cómo leerlo están en
`AJUSTES_REDACCION_TESIS.md`, sección 2.

| Indicador | Fórmula | Fuente oficial | Meta |
|---|---|---|---|
| PCD | VP / (VP + FN), solo tipos estructurales y conflictos `observable` | Bloque B (micro) | ≥ 70 % |
| PRE | VP / (VP + FP) | Bloque B (micro) | ≥ 90 % |
| PSR | VP con sugerencia del tipo esperado / VP con sugerencia esperada definida | Bloque B | ≥ 50 % |
| F1 | 2PR / (P + R) de las coincidencias: de elementos afectados y por categoría | Bloque B | Línea base |
| TRI | (t manual − t asistido) / t manual, promedio de M1 a M3 | Bloque C | ≥ 50 % |
| REP | % de comparaciones con salidas idénticas en 3 repeticiones | Bloques A y B (el menor) | 100 % |
| ROB | % de comparaciones sin error y con JSON válidos | Bloques A y B (el menor) | ≥ 90 % |
| SYM | % de pares no ordenados simétricos | Bloque A | Se reporta |
| FCI | Conflictos y exclusivos en las comparaciones de identidad | Bloque A | 0 |
| PRM | % de RF1–RF6 y RNF1–RNF4 con todas sus pruebas de aceptación aprobadas | `requirements_coverage.py` | ≥ 80 % |
| IAC-1 | Descargas acumuladas de audd-node en npm a la fecha de corte | API de npm | ≥ 20 |
| IAC-2 | Issues, PR y comentarios de cuentas externas, sin cuentas bot | API de GitHub | Se reporta |

TRL 4 se declara si se cumplen PCD, PRE, REP, ROB y PRM (Tabla 18 de la tesis).

---

## 10. Ejecución completa

En PowerShell, desde la raíz del repositorio y con MySQL en marcha:

```powershell
# 0. Preparación (sección 4, una sola vez)
$env:AUDD_MYSQL = "audd:una_clave_local@127.0.0.1:3306"
cargo build --release -p audd-cli
python evaluation/suite/setup_mysql.py
git tag v0.9.0-eval
$RUN = "evaluation/results/20261101_eval"

# 1. Bloque A
python evaluation/suite/cross.py --out $RUN

# 2. Bloque B
python evaluation/mutate.py --all --backend mysql
python evaluation/suite/mutations.py --pairs evaluation/generated --out $RUN
python evaluation/suite/mutations.py --pairs evaluation/generated --out "${RUN}_pos" --domain pos

# 3. Bloque C (por cada par; ver sección 7)
python evaluation/suite/manual.py init M2_unicenta_growventa
#    ... alcance, verdad, freeze y commit ...
python evaluation/suite/manual.py run M2_unicenta_growventa
python evaluation/suite/manual.py score M2_unicenta_growventa
python evaluation/suite/manual.py tri

# 4. PRM e IAC
python evaluation/requirements_coverage.py
python evaluation/adoption.py

# 5. Reporte
python evaluation/suite/report.py --run $RUN
```

`report.py` genera `summary.md` (tablas listas para la tesis) e `indicators.json`. Cada corrida
queda en su propia carpeta con `manifest.json`, que registra commit, etiqueta, hash del
binario, hash del catálogo, opciones, entorno y hash de la IR de cada base.

---

## 11. Qué va a la tesis

| Tesis | Fuente |
|---|---|
| Tabla 19 (diseño de la evaluación) | Secciones 1, 5, 6 y 7 de este protocolo |
| Tabla 20 (operadores del bloque B) | Sección 6.2 |
| Tabla 21 (resultados) | `summary.md`: contraste con las metas y bloques A, B y C |
| PCD por tipo y por nivel; F1 por categoría | Tablas del bloque B en `summary.md` |
| TRI por par | `evaluation/manual/tri.json` |
| 5.6.4 Limitaciones | Conflictos `not_observable`, asimetrías de `symmetry.json`, número de correcciones de la verdad, sesgo de evaluador |
| Anexo de la suite | Este protocolo, `config/suite.toml`, `manifest.json`, `summary.md` y las carpetas `manual/M*` |

---

## 12. Anexo: problemas de la suite anterior (no reportar sus cifras)

La revisión de `evaluation/run.py`, del mutador anterior y de `results/summary.md` (corrida del
18-oct-2026) encontró errores que invalidan sus cifras. **El PCD de 12 % y el PRE, PSR y ROB de
100 % de esa corrida no deben usarse en la tesis.** `run.py` quedó marcado como obsoleto.

| # | Problema | Efecto | Corrección |
|---|---|---|---|
| S1 | «PRE» se calculaba como reproducibilidad, no como VP/(VP+FP) | PRE = 100 % sin significado | `score.py` |
| S2 | «ROB» se calculaba como % de archivos estables | ROB inflado | `cross.py` y `mutations.py` |
| S3 | «PSR» contaba cualquier sugerencia, sin compararla con la esperada | PSR = 100 % sin significado | `score.py` |
| S4 | Los pares de «identidad» sí tenían mutaciones | El nivel 1 no probaba identidad | Identidad en el bloque A |
| S5 | Las mutaciones se cargaban en SQLite, que no conservaba longitudes ni Int32/Int64 | Mutaciones invisibles contadas como FN | Generación desde la IR, carga en MySQL y `validate_truth.py` |
| S6 | La verdad contaba como conflicto cambios compatibles | FN que no eran del núcleo | Especificación de la sección 3 |
| S7 | `requirements_coverage.py` no incluía RF6 y contaba pruebas existentes, no aprobadas | PRM mal definido | Cuenta solo pruebas aprobadas |
| S8 | `adoption.py` reportaba descargas del último mes | IAC-1 subestimado | Descargas acumuladas |
| S9 | `datasets.toml` tenía hashes de ejemplo | Entradas sin verificar | Hashes reales y `snapshots.lock.json` |

---

## 13. Registro de cambios al protocolo

| Fecha | Cambio | Motivo |
|---|---|---|
| 2026-10-19 | Especificación adoptada: **simétrica**. Implementados E1, E2, E3 y E5 en `audd_compare` (E4 no). | Decisión del autor (sección 3) |
| 2026-10-19 | Reglas de tipo del catálogo: `from` se compara con A y `to` con B por nombre exacto; `symmetric = true` en R-TYPE-001, 002, 003 y 006; alternativas con `\|` (p. ej. `DateTime\|Timestamp`). | La comparación anterior era por subcadena y sin sentido (String→Int32 recibía la regla de Int32→String) |
| 2026-10-19 | Northwind se usa en su versión SQLite (`bases/northwind/northwind.db`). | No hay DDL MySQL de Northwind en el repositorio |
| 2026-10-19 | `mutate.py` genera A y B renderizando la IR de cada base como DDL MySQL (A sin cambios, B con mutaciones); la verdad sale del registro de mutaciones y de la tabla de la sección 3. Backend `ddl` disponible como alternativa sin servidor; la corrida oficial usa `mysql`. | A y B solo difieren en lo mutado; permite usar bases que no están en MySQL |
| 2026-10-19 | `validate_truth.py`: la pertenencia a la PK no cuenta como UNIQUE. | E4 no se implementó; contarla generaría falsos negativos que no son del núcleo |
| 2026-10-19 | **Colisión de normalización (núcleo).** Además de la comparación entre campos sin pareja de lados distintos, el motor detecta la colisión **dentro de cada esquema**: dos campos de la misma entidad que se normalizan igual (p. ej. `cliente_id` y `ClienteId` en B). No es conflicto si el otro esquema tiene exactamente los mismos nombres (cada campo se empareja con su homónimo). El conflicto lleva el nombre del campo que tiene pareja, para que A→B y B→A coincidan. | La versión anterior solo buscaba entre campos no emparejados y además dejaba vacío el campo del conflicto, por lo que el tipo daba siempre 0 % |
| 2026-10-19 | **Dominio como estrategia de emparejamiento (núcleo, K04).** `--domain` ya no renombra los esquemas: las equivalencias son una estrategia más (`CatalogEquivalence`), que compara los nombres normalizados y en singular, completos o por palabra (`unit_price` ≈ `price`). Un dominio solo puede agregar coincidencias. | Al renombrar solo por nombre exacto, `--domain pos` rompía coincidencias existentes (`customers` contra `Customer` → `cliente`) |
| 2026-10-19 | **Mutaciones del bloque B, diseño ampliado (antes de la corrida oficial).** De 5 a 10 cambios por variante (antes 3 a 6), con estos operadores y niveles: **fácil** (cambio de tipo, longitud, nulabilidad o UNIQUE; agregar o quitar campo o tabla; renombrado de estilo o plural); **medio** (prefijo heredado, p. ej. `cus_email`; cambio combinado: renombrar y cambiar tipo, longitud o nulabilidad del mismo campo; cambio equivalente `VARCHAR`→`TEXT` o `DATETIME`↔`TIMESTAMP`, que no debe producir conflicto; colisión de normalización); **difícil** (abreviatura, sinónimo en inglés, traducción inglés→español, renombrado sin relación y **distractor**: sustituir un campo por otro de nombre casi igual y significado distinto, p. ej. `created_at`→`created_by`, que no debe emparejarse). Los renombrados aplican a campos y a tablas. Cada coincidencia de la verdad indica su categoría y si el dominio `pos` la cubre. | Con el diseño anterior casi todo el bloque B era trivial: sin ruido y con cambios que el núcleo está diseñado para detectar |
| 2026-10-19 | **F1 por categoría.** Además del F1 global se reporta el **F1 de elementos afectados** (sin los campos que no cambiaron) y el F1 por categoría (`style`, `plural`, `affix`, `abbreviation`, `synonym`, `translation`, `distractor`, `unrelated`). Emparejar con el campo equivocado de B cuenta como FP y FN; emparejar un distractor o un renombrado sin relación cuenta como FP de su categoría. PCD se reporta también por nivel de la mutación. La corrida oficial va sin dominio; se repite con `--domain pos` como análisis adicional. | El F1 global (0.99) lo dominaban ~10 800 campos sin cambios; sobre los renombrados era 0 |
| 2026-10-19 | **Protocolo versión 1.1.** Se reorganizó el documento: lógica de las tres etapas (sección 1), preparación con Laragon e importación a MySQL (sección 4), protocolo del experto paso a paso (sección 7) y anexo de la suite anterior (sección 12). Los exclusivos de B dentro de una tabla emparejada se nombran con la tabla de B, tanto en la verdad del bloque B como al puntuar (`score.py` traduce el nombre que trae `diff.json`). | Hacer el protocolo ejecutable por una persona ajena al desarrollo; con el nombre de A, los exclusivos de B escritos por el experto nunca coincidían |
