# Bases de datos para la evaluación de AUDD

Este directorio contiene esquemas reales y de referencia para validar AUDD en
diferentes motores y formatos. Los archivos se agrupan por dataset.

## Estructura

```
bases/
├── chinohook/           # Chinook (música)
│   ├── Chinook_MySql.sql       # DDL para MySQL
│   └── Chinook_Sqlite.sqlite   # Base SQLite lista para usar
├── employees/           # Empleados (dataset didáctico pequeño)
│   └── employees.sql
├── growventa/           # Grow Venta (caso real POS)
│   ├── growventa.db            # Base SQLite lista para usar
│   └── grow_venta.sql          # DDL extraído de growventa.db para mutaciones
├── northwind/           # Northwind (clásico de Microsoft)
│   └── northwind.db            # Base SQLite lista para usar
├── sakila-db/           # Sakila (MySQL sample database)
│   ├── sakila-schema.sql       # Solo estructura
│   ├── sakila-data.sql         # Solo datos (no útil directamente para AUDD)
│   └── sakila.mwb              # Modelo MySQL Workbench (no leíble por AUDD)
└── unicenta-opos/       # UniCenta OPOS (POS open source)
    └── MySQL-create.sql        # DDL para MySQL
```

> Nota: `growtravel/growvtravel.sql` se mantiene en `evaluation/bases/growtravel/`
> porque fue aportado después de la estructura inicial.

## Formatos soportados directamente por AUDD

| Formato | Extensión | Cómo usar | Comentario |
|---|---|---|---|
| SQLite | `.sqlite`, `.db`, `.sqlite3` | `db:sqlite://ruta/al/archivo.db` | Listo para usar. |
| SQL/DDL | `.sql` | `audd compare --source-a ruta.sql ...` | El parser SQL interno extrae `CREATE TABLE`. Funciona para MySQL, PostgreSQL y SQL/DDL genérico. |
| CSV | `.csv` | `ruta.csv` | Requiere encabezados; infiere tipos de los datos. |
| JSON | `.json` | `ruta.json` | Array de objetos; infiere tipos del primer objeto. |

## Formatos NO soportados

| Formato | Extensión | Razón |
|---|---|---|
| MySQL Workbench | `.mwb` | Binario propietario; convertir a `.sql` con MySQL Workbench. |
| SQL Server `.bak` | `.bak` | Backup binario; restaurar primero. |
| Datos sin DDL | `-data.sql` | AUDD trabaja sobre estructura, no sobre datos. |

## Pares de prueba recomendados

### 1. Chinook cross-engine (mismo esquema, distinto motor)

```bash
audd compare \
  --source-a evaluation/bases/chinohook/Chinook_MySql.sql \
  --source-b db:sqlite://evaluation/bases/chinohook/Chinook_Sqlite.sqlite \
  --out evaluation/results/chinook_cross_engine
```

Objetivo: verificar que AUDD empareja la misma base de datos aunque venga de
motores distintos. Se esperan muchos matches y pocos exclusivos/conflictos.

### 2. Grow Travel vs Grow Venta (caso real)

```bash
audd compare \
  --source-a evaluation/bases/growtravel/growvtravel.sql \
  --source-b db:sqlite://evaluation/bases/growventa/growventa.db \
  --out evaluation/results/grow_real_case
```

Resultado esperado (valores reportados en la tesis): 17 coincidencias,
~145 exclusivos y 4 conflictos.

### 3. POS vs POS (UniCenta vs Grow Venta)

```bash
audd compare \
  --source-a evaluation/bases/unicenta-opos/MySQL-create.sql \
  --source-b db:sqlite://evaluation/bases/growventa/growventa.db \
  --out evaluation/results/unicenta_vs_growventa
```

Objetivo: comparar dos sistemas de punto de venta del mismo dominio.

### 4. Northwind vs Chinook

```bash
audd compare \
  --source-a db:sqlite://evaluation/bases/northwind/northwind.db \
  --source-b db:sqlite://evaluation/bases/chinohook/Chinook_Sqlite.sqlite \
  --out evaluation/results/northwind_vs_chinook
```

Objetivo: comparar dos esquemas clásicos de dominios distintos (ventas vs
música).

## Notas sobre los datasets

- **Chinook**: disponible en https://github.com/lerocha/chinook-database. La
  versión SQLite es la más fiel al original; la versión MySQL tiene pequeñas
  diferencias sintácticas que el parser de AUDD tolera.
- **Northwind**: versión SQLite de https://github.com/jpwhite3/northwind-SQLite3.
  Contiene identificadores con espacios (p. ej. `Order Details`); el adaptador
  SQLite de AUDD los soporta desde la corrección de validación de identificadores.
- **Sakila**: usar solo `sakila-schema.sql`. `sakila-data.sql` contiene solo
  `INSERT`s y `sakila.mwb` no es legible.
- **Grow Venta / Grow Travel**: esquemas propios anonimizados. Solo estructura
  relevante; los datos presentes en `growventa.db` no se leen por AUDD.
