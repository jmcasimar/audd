#!/usr/bin/env python3
"""
mutate.py

Genera pares sintéticos de esquemas SQLite a partir de un esquema base,
aplicando operadores de mutación determinísticos con semilla fija.

Uso:
    python evaluation/mutate.py --base evaluation/bases/grow_venta.sql \
        --pair-id grow_venta_v1_v2 --level 2 --seed 42 \
        --out evaluation/generated/grow_venta_v1_v2

Salidas:
    a.sqlite          esquema original (lado A)
    b.sqlite          esquema mutado (lado B)
    truth.json        verdad de referencia (matches, exclusivos, conflictos)
"""

import argparse
import json
import os
import random
import re
import sqlite3
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, List, Optional, Tuple


@dataclass
class Table:
    name: str
    sql: str
    columns: List[str] = field(default_factory=list)


@dataclass
class Mutation:
    operator: str
    target: str  # table or table.column
    from_value: str
    to_value: str
    side: str  # 'a' or 'b'
    expected_conflict: Optional[str] = None
    expected_suggestion: Optional[str] = None


def parse_tables(ddl: str) -> List[Table]:
    """Extrae las definiciones de CREATE TABLE del DDL."""
    tables = []
    pattern = re.compile(
        r"CREATE\s+TABLE\s+(?P<name>\w+)\s*\((?P<body>[^;]+)\);",
        re.IGNORECASE | re.DOTALL,
    )
    for match in pattern.finditer(ddl):
        name = match.group("name")
        body = match.group("body")
        # Extraer nombres de columnas (la primera palabra de cada línea no vacía)
        columns = []
        for line in body.splitlines():
            line = line.strip()
            if not line or line.upper().startswith(("PRIMARY KEY", "FOREIGN KEY", "UNIQUE", "CHECK", "CONSTRAINT")):
                continue
            if line.startswith("--"):
                continue
            col_name = line.split()[0].strip("`\"[]")
            columns.append(col_name)
        tables.append(Table(name=name, sql=match.group(0), columns=columns))
    return tables


def load_base_schema(path: Path) -> str:
    with open(path, "r", encoding="utf-8") as f:
        return f.read()


def apply_ddl(conn: sqlite3.Connection, ddl: str) -> None:
    conn.executescript(ddl)
    conn.commit()


def save_database(conn: sqlite3.Connection, path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    if path.exists():
        path.unlink()
    backup_conn = sqlite3.connect(str(path))
    conn.backup(backup_conn)
    backup_conn.close()


# ---------- Operadores de mutación ----------

OPERATORS = {
    "rename_table_style": {
        "description": "Cambiar el estilo de nombre de una tabla",
        "conflict": "NormalizationCollision",
        "suggestion": "RenameField",
    },
    "rename_field_synonym": {
        "description": "Renombrar un campo a un sinónimo del dominio POS",
        "conflict": None,  # se resuelve como coincidencia por equivalencia
        "suggestion": None,
    },
    "widen_type": {
        "description": "Ampliar un tipo numérico o cadena",
        "conflict": "TypeIncompatible",
        "suggestion": "CastSafe",
    },
    "narrow_length": {
        "description": "Reducir la longitud de una cadena",
        "conflict": "LengthMismatch",
        "suggestion": "PreferType",
    },
    "toggle_nullable": {
        "description": "Cambiar la nulabilidad de una columna",
        "conflict": "NullabilityMismatch",
        "suggestion": "PreferType",
    },
    "add_unique": {
        "description": "Agregar restricción UNIQUE a una columna",
        "conflict": "ConstraintMismatch",
        "suggestion": "NoSuggestion",
    },
    "drop_field": {
        "description": "Eliminar un campo",
        "conflict": None,
        "suggestion": None,
    },
    "add_field": {
        "description": "Agregar un campo nuevo",
        "conflict": None,
        "suggestion": None,
    },
    "rename_unrelated": {
        "description": "Renombrar una tabla sin relación semántica",
        "conflict": None,
        "suggestion": None,
    },
}


def mutate_rename_table_style(
    ddl: str, tables: List[Table], rng: random.Random, side: str
) -> Tuple[str, Optional[Mutation]]:
    table = rng.choice(tables)
    old = table.name
    if old[0].islower():
        new = old.capitalize()
    else:
        new = old.lower()
    new_ddl = re.sub(rf"\b{re.escape(old)}\b", new, ddl)
    mutation = Mutation(
        operator="rename_table_style",
        target=old,
        from_value=old,
        to_value=new,
        side=side,
        expected_conflict="NormalizationCollision",
        expected_suggestion="RenameField",
    )
    return new_ddl, mutation


def mutate_rename_field_synonym(
    ddl: str, tables: List[Table], rng: random.Random, side: str
) -> Tuple[str, Optional[Mutation]]:
    synonyms = {
        "precio": "price",
        "price": "precio",
        "producto": "product",
        "product": "producto",
        "cliente": "customer",
        "customer": "cliente",
        "venta": "sale",
        "sale": "venta",
        "fecha": "date",
        "date": "fecha",
        "cantidad": "quantity",
        "quantity": "cantidad",
        "codigo_barras": "barcode",
        "barcode": "codigo_barras",
        "existencia": "stock",
        "stock": "existencia",
    }
    candidates = [(t.name, c) for t in tables for c in t.columns if c in synonyms]
    if not candidates:
        return ddl, None
    table_name, col = rng.choice(candidates)
    new_col = synonyms[col]
    # Evitar colisiones con columnas existentes
    table = next(t for t in tables if t.name == table_name)
    if new_col in table.columns:
        return ddl, None
    new_ddl = re.sub(rf"\b{re.escape(col)}\b", new_col, ddl)
    mutation = Mutation(
        operator="rename_field_synonym",
        target=f"{table_name}.{col}",
        from_value=col,
        to_value=new_col,
        side=side,
        expected_conflict=None,
        expected_suggestion=None,
    )
    return new_ddl, mutation


def mutate_widen_type(
    ddl: str, tables: List[Table], rng: random.Random, side: str
) -> Tuple[str, Optional[Mutation]]:
    candidates = []
    for t in tables:
        for c in t.columns:
            candidates.append((t.name, c))
    table_name, col = rng.choice(candidates)
    # Patrones de reemplazo que cambian el tipo base y son detectables por AUDD
    replacements = [
        (rf"\b{re.escape(col)}\s+INTEGER\b", f"{col} BIGINT"),
        (rf"\b{re.escape(col)}\s+BOOLEAN\b", f"{col} VARCHAR(10)"),
        (rf"\b{re.escape(col)}\s+DATETIME\b", f"{col} DATE"),
        (rf"\b{re.escape(col)}\s+DATE\b", f"{col} DATETIME"),
    ]
    new_ddl = ddl
    applied = False
    for pattern, repl in replacements:
        if re.search(pattern, ddl, re.IGNORECASE):
            new_ddl = re.sub(pattern, repl, ddl, count=1, flags=re.IGNORECASE)
            applied = True
            break
    if not applied:
        return ddl, None
    mutation = Mutation(
        operator="widen_type",
        target=f"{table_name}.{col}",
        from_value="original",
        to_value="widened",
        side=side,
        expected_conflict="TypeIncompatible",
        expected_suggestion="CastSafe",
    )
    return new_ddl, mutation


def mutate_narrow_length(
    ddl: str, tables: List[Table], rng: random.Random, side: str
) -> Tuple[str, Optional[Mutation]]:
    # En SQLite las longitudes de VARCHAR no se conservan, así que convertimos a TEXT
    # para obtener un TypeIncompatible detectable por AUDD.
    candidates = []
    for t in tables:
        for c in t.columns:
            if re.search(rf"\b{re.escape(c)}\s+VARCHAR\s*\(", ddl, re.IGNORECASE):
                candidates.append((t.name, c))
    if not candidates:
        return ddl, None
    table_name, col = rng.choice(candidates)
    pattern = rf"\b{re.escape(col)}\s+VARCHAR\s*\(\s*\d+\s*\)"
    new_ddl = re.sub(pattern, f"{col} TEXT", ddl, count=1, flags=re.IGNORECASE)
    mutation = Mutation(
        operator="narrow_length",
        target=f"{table_name}.{col}",
        from_value="VARCHAR",
        to_value="TEXT",
        side=side,
        expected_conflict="TypeIncompatible",
        expected_suggestion="CastSafe",
    )
    return new_ddl, mutation


def mutate_toggle_nullable(
    ddl: str, tables: List[Table], rng: random.Random, side: str
) -> Tuple[str, Optional[Mutation]]:
    candidates = []
    for t in tables:
        for c in t.columns:
            if c.lower() == "id":
                continue
            candidates.append((t.name, c))
    table_name, col = rng.choice(candidates)
    # Buscar definición de columna
    pattern = rf"\b{re.escape(col)}\s+([^,\n]+)"
    match = re.search(pattern, ddl, re.IGNORECASE)
    if not match:
        return ddl, None
    defn = match.group(1)
    if "NOT NULL" in defn.upper():
        new_defn = re.sub(r"\s*NOT\s+NULL\s*", " ", defn, flags=re.IGNORECASE).strip()
    else:
        new_defn = defn.rstrip() + " NOT NULL"
    new_ddl = re.sub(pattern, f"{col} {new_defn}", ddl, count=1, flags=re.IGNORECASE)
    mutation = Mutation(
        operator="toggle_nullable",
        target=f"{table_name}.{col}",
        from_value="original",
        to_value="toggled",
        side=side,
        expected_conflict="NullabilityMismatch",
        expected_suggestion="PreferType",
    )
    return new_ddl, mutation


def mutate_add_unique(
    ddl: str, tables: List[Table], rng: random.Random, side: str
) -> Tuple[str, Optional[Mutation]]:
    table = rng.choice(tables)
    col = rng.choice(table.columns)
    constraint_name = f"uq_{table.name}_{col}"
    stmt = f"\nCREATE UNIQUE INDEX {constraint_name} ON {table.name}({col});"
    new_ddl = ddl + stmt
    mutation = Mutation(
        operator="add_unique",
        target=f"{table.name}.{col}",
        from_value="no unique",
        to_value="unique",
        side=side,
        expected_conflict="ConstraintMismatch",
        expected_suggestion="NoSuggestion",
    )
    return new_ddl, mutation


def mutate_drop_field(
    ddl: str, tables: List[Table], rng: random.Random, side: str
) -> Tuple[str, Optional[Mutation]]:
    table = rng.choice(tables)
    # Evitar columnas que parezcan claves (id, GenreId, etc.) para no romper PK/FK
    droppable = [c for c in table.columns if c.lower() != "id" and not c.lower().endswith("id")]
    if not droppable:
        return ddl, None
    col = rng.choice(droppable)
    # Usar ALTER TABLE DROP COLUMN para evitar modificar el CREATE TABLE
    stmt = f"\nALTER TABLE {table.name} DROP COLUMN {col};"
    new_ddl = ddl + stmt
    mutation = Mutation(
        operator="drop_field",
        target=f"{table.name}.{col}",
        from_value=col,
        to_value="<dropped>",
        side=side,
        expected_conflict=None,
        expected_suggestion=None,
    )
    return new_ddl, mutation


def mutate_add_field(
    ddl: str, tables: List[Table], rng: random.Random, side: str
) -> Tuple[str, Optional[Mutation]]:
    table = rng.choice(tables)
    new_col = f"extra_field_{rng.randint(1000, 9999)}"
    # Usar ALTER TABLE ADD COLUMN para evitar modificar el CREATE TABLE
    stmt = f"\nALTER TABLE {table.name} ADD COLUMN {new_col} VARCHAR(255);"
    new_ddl = ddl + stmt
    mutation = Mutation(
        operator="add_field",
        target=f"{table.name}.{new_col}",
        from_value="<absent>",
        to_value=new_col,
        side=side,
        expected_conflict=None,
        expected_suggestion=None,
    )
    return new_ddl, mutation


def mutate_rename_unrelated(
    ddl: str, tables: List[Table], rng: random.Random, side: str
) -> Tuple[str, Optional[Mutation]]:
    table = rng.choice(tables)
    old = table.name
    new = f"tbl_{rng.randint(1000, 9999)}"
    new_ddl = re.sub(rf"\b{re.escape(old)}\b", new, ddl)
    mutation = Mutation(
        operator="rename_unrelated",
        target=old,
        from_value=old,
        to_value=new,
        side=side,
        expected_conflict=None,
        expected_suggestion=None,
    )
    return new_ddl, mutation


MUTATION_FUNCTIONS = [
    mutate_rename_table_style,
    mutate_rename_field_synonym,
    mutate_widen_type,
    mutate_narrow_length,
    mutate_toggle_nullable,
    mutate_add_unique,
    mutate_drop_field,
    mutate_add_field,
    mutate_rename_unrelated,
]


def is_valid_ddl(ddl: str) -> bool:
    """Verifica que el DDL se pueda ejecutar en SQLite."""
    conn = sqlite3.connect(":memory:")
    try:
        conn.executescript(ddl)
        conn.close()
        return True
    except sqlite3.Error:
        conn.close()
        return False


def generate_pair(
    base_ddl: str,
    pair_id: str,
    level: int,
    seed: int,
) -> Tuple[str, str, List[Mutation], List[Mutation]]:
    """Genera DDL_a y DDL_b aplicando mutaciones determinísticas."""
    rng = random.Random(seed)

    # Lado A: identidad (puede tener 0-1 mutaciones leves según el nivel)
    a_mutations: List[Mutation] = []
    ddl_a = base_ddl
    if level >= 2 and rng.random() < 0.3:
        op = rng.choice(MUTATION_FUNCTIONS)
        new_ddl, mut = op(ddl_a, parse_tables(ddl_a), rng, "a")
        if mut and is_valid_ddl(new_ddl):
            ddl_a = new_ddl
            a_mutations.append(mut)

    # Lado B: 3-6 mutaciones según el nivel
    b_mutations: List[Mutation] = []
    ddl_b = base_ddl
    num_mutations = rng.randint(3, 6)
    attempts = 0
    while len(b_mutations) < num_mutations and attempts < num_mutations * 10:
        attempts += 1
        op = rng.choice(MUTATION_FUNCTIONS)
        current_tables = parse_tables(ddl_b)
        new_ddl, mut = op(ddl_b, current_tables, rng, "b")
        if mut and is_valid_ddl(new_ddl):
            ddl_b = new_ddl
            b_mutations.append(mut)

    return ddl_a, ddl_b, a_mutations, b_mutations


SYNONYMS = {
    "precio": ["price", "unit_price", "precio_unitario", "pvp"],
    "producto": ["product", "item", "articulo"],
    "cliente": ["customer", "client"],
    "venta": ["sale", "ticket", "order"],
    "fecha": ["date", "created_at", "fecha_venta"],
    "cantidad": ["quantity", "qty"],
    "codigo_barras": ["barcode", "sku", "upc"],
    "existencia": ["stock", "inventario", "qty_on_hand"],
}


def normalize(name: str) -> str:
    return name.lower().replace("_", "").replace(" ", "")


def canonical_name(name: str) -> str:
    """Devuelve el nombre canónico considerando sinónimos del dominio POS."""
    norm = normalize(name)
    for canonical, aliases in SYNONYMS.items():
        if norm == normalize(canonical) or norm in [normalize(a) for a in aliases]:
            return canonical
    return name


@dataclass
class Column:
    name: str
    dtype: str
    nullable: bool
    default: Optional[str]


@dataclass
class Schema:
    tables: Dict[str, List[Column]]
    indexes: Dict[str, List[str]]  # table -> list of unique-indexed columns


def extract_schema(path: Path) -> Schema:
    conn = sqlite3.connect(str(path))
    conn.row_factory = sqlite3.Row
    cur = conn.cursor()
    tables: Dict[str, List[Column]] = {}
    indexes: Dict[str, List[str]] = {}
    cur.execute("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'")
    for row in cur.fetchall():
        table_name = row["name"]
        columns = []
        cur.execute(f"PRAGMA table_info({table_name})")
        for col in cur.fetchall():
            columns.append(Column(
                name=col["name"],
                dtype=col["type"].upper() if col["type"] else "",
                nullable=not bool(col["notnull"]),
                default=col["dflt_value"],
            ))
        tables[table_name] = columns

        unique_cols = []
        cur.execute(f"PRAGMA index_list({table_name})")
        for idx in cur.fetchall():
            if idx["unique"]:
                cur.execute(f"PRAGMA index_info({idx['name']})")
                for info in cur.fetchall():
                    unique_cols.append(info["name"])
        indexes[table_name] = unique_cols
    conn.close()
    return Schema(tables=tables, indexes=indexes)


def parse_length(dtype: str) -> Optional[int]:
    m = re.search(r"\((\d+)\)", dtype)
    return int(m.group(1)) if m else None


def base_type(dtype: str) -> str:
    """Extrae el tipo base sin parámetros (p. ej. VARCHAR(200) -> VARCHAR)."""
    return re.split(r"[\s(]", dtype.strip().upper())[0]


def build_truth(
    pair_id: str,
    level: int,
    seed: int,
    path_a: Path,
    path_b: Path,
) -> Dict:
    """Construye truth.json comparando los esquemas finales A y B."""
    schema_a = extract_schema(path_a)
    schema_b = extract_schema(path_b)

    matches = []
    exclusives = []
    conflicts = []

    # Mapear tablas B por nombre canónico
    b_tables_by_canonical = {canonical_name(t): t for t in schema_b.tables}

    for a_table, a_cols in schema_a.tables.items():
        a_canonical = canonical_name(a_table)
        b_table = b_tables_by_canonical.get(a_canonical)

        if b_table is None:
            exclusives.append({"side": "a", "entity": a_table})
            continue

        reason = "ExactName" if a_table == b_table else "NormalizedName"
        if normalize(a_table) != normalize(b_table) and a_table.lower() != b_table.lower():
            if canonical_name(a_table) == canonical_name(b_table) and a_table != b_table:
                reason = "CatalogEquivalence"
        matches.append({"a": a_table, "b": b_table, "reason": reason})

        b_cols_by_canonical = {canonical_name(c.name): c for c in schema_b.tables[b_table]}

        for a_col in a_cols:
            a_col_canonical = canonical_name(a_col.name)
            b_col = b_cols_by_canonical.get(a_col_canonical)

            if b_col is None:
                exclusives.append({"side": "a", "entity": a_table, "field": a_col.name})
                continue

            col_reason = "ExactName" if a_col.name == b_col.name else "NormalizedName"
            if canonical_name(a_col.name) == canonical_name(b_col.name) and a_col.name != b_col.name:
                col_reason = "CatalogEquivalence"
            matches.append({
                "a": f"{a_table}.{a_col.name}",
                "b": f"{b_table}.{b_col.name}",
                "reason": col_reason,
            })

            # Tipos y longitudes
            a_type = a_col.dtype.upper()
            b_type = b_col.dtype.upper()
            a_base = base_type(a_type)
            b_base = base_type(b_type)
            a_len = parse_length(a_type)
            b_len = parse_length(b_type)

            if a_base != b_base:
                conflicts.append({
                    "entity": a_table,
                    "field": a_col.name,
                    "type": "TypeIncompatible",
                    "side": "b",
                    "expected_suggestion": "CastSafe",
                })
            elif a_len is not None and b_len is not None and a_len != b_len:
                conflicts.append({
                    "entity": a_table,
                    "field": a_col.name,
                    "type": "LengthMismatch",
                    "side": "b",
                    "expected_suggestion": "PreferType",
                })

            # Nulabilidad
            if a_col.nullable != b_col.nullable:
                conflicts.append({
                    "entity": a_table,
                    "field": a_col.name,
                    "type": "NullabilityMismatch",
                    "side": "b",
                    "expected_suggestion": "PreferType",
                })

            # UNIQUE
            a_unique = a_col.name in schema_a.indexes.get(a_table, [])
            b_unique = b_col.name in schema_b.indexes.get(b_table, [])
            if a_unique != b_unique:
                conflicts.append({
                    "entity": a_table,
                    "field": a_col.name,
                    "type": "ConstraintMismatch",
                    "side": "b",
                    "expected_suggestion": "NoSuggestion",
                })

    # Tablas exclusivas de B
    for b_table in schema_b.tables:
        if canonical_name(b_table) not in {canonical_name(t) for t in schema_a.tables}:
            exclusives.append({"side": "b", "entity": b_table})

    return {
        "pair_id": pair_id,
        "level": level,
        "seed": seed,
        "matches": matches,
        "exclusives": exclusives,
        "conflicts": conflicts,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description="Genera pares sintéticos de esquemas SQLite.")
    parser.add_argument("--base", required=True, type=Path, help="DDL base")
    parser.add_argument("--pair-id", required=True, help="Identificador del par")
    parser.add_argument("--level", type=int, default=2, choices=[1, 2, 3, 4], help="Nivel del corpus")
    parser.add_argument("--seed", type=int, required=True, help="Semilla determinística")
    parser.add_argument("--out", required=True, type=Path, help="Directorio de salida")
    args = parser.parse_args()

    base_ddl = load_base_schema(args.base)
    base_tables = parse_tables(base_ddl)

    ddl_a, ddl_b, mutations_a, mutations_b = generate_pair(
        base_ddl, args.pair_id, args.level, args.seed
    )

    args.out.mkdir(parents=True, exist_ok=True)

    # Guardar DDL intermedios para depuración
    (args.out / "a.sql").write_text(ddl_a, encoding="utf-8")
    (args.out / "b.sql").write_text(ddl_b, encoding="utf-8")

    # Crear bases SQLite
    conn_a = sqlite3.connect(":memory:")
    apply_ddl(conn_a, ddl_a)
    save_database(conn_a, args.out / "a.sqlite")
    conn_a.close()

    conn_b = sqlite3.connect(":memory:")
    apply_ddl(conn_b, ddl_b)
    save_database(conn_b, args.out / "b.sqlite")
    conn_b.close()

    # Truth
    truth = build_truth(
        args.pair_id, args.level, args.seed, args.out / "a.sqlite", args.out / "b.sqlite"
    )
    (args.out / "truth.json").write_text(
        json.dumps(truth, indent=2, ensure_ascii=False), encoding="utf-8"
    )

    print(f"Generated pair in {args.out}")
    print(f"  A mutations: {len(mutations_a)}; B mutations: {len(mutations_b)}")
    print(f"  Matches: {len(truth['matches'])}; Exclusives: {len(truth['exclusives'])}; Conflicts: {len(truth['conflicts'])}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
