#!/usr/bin/env python3
"""Generador de pares mutados para el bloque B (PROTOCOLO_EVALUACION.md, sección 7).

Para cada base y semilla:
    1. Lee la IR de la base con `audd inspect` (de su DDL en el repositorio o de su fuente).
    2. Genera A renderizando esa IR como DDL MySQL y B aplicando 5 a 10 mutaciones
       sobre la misma IR. Así, A y B solo difieren en las mutaciones registradas.
       Cada mutación tiene un nivel (facil, medio, dificil) y, si renombra, una
       categoría (style, plural, affix, abbreviation, synonym, translation); además
       hay cambios combinados, cambios equivalentes y distractores (sección 12).
    3. Escribe la verdad (`truth.json`) a partir del registro de mutaciones y de la
       especificación de unificación (sección 5), sin usar el motor de comparación.
    4. Carga A y B en MySQL (backend `mysql`, oficial) o los deja como archivos DDL
       (backend `ddl`, alternativa sin servidor) y escribe `pair.json`.

`suite/validate_truth.py` comprueba después que cada conflicto de la verdad llegó a la
IR de entrada; las mutaciones que el adaptador pierde se excluyen de PCD.

Uso:
    python evaluation/mutate.py --all                         # 5 bases x semillas 1000-1009
    python evaluation/mutate.py --base chinook --seed 1000
    python evaluation/mutate.py --all --backend ddl           # sin servidor MySQL
"""

from __future__ import annotations

import argparse
import copy
import json
import random
import re
import shutil
import sys
import tempfile
import tomllib
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any, Callable, Dict, List, Optional, Tuple

sys.path.insert(0, str(Path(__file__).parent / "suite"))
from common import (EVAL_DIR, PROJECT_ROOT, bases_by_id, load_config, load_json,  # noqa: E402
                    run_inspect, source_of, write_json)

GENERATED_DIR = EVAL_DIR / "generated"
DEFAULT_BASES = ["chinook", "employees", "northwind", "unicenta", "growtravel"]
DEFAULT_SEEDS = list(range(1000, 1010))
MIN_CHANGES, MAX_CHANGES = 5, 10  # sección 12 del protocolo (2026-10-19)

# --------------------------------------------------------------------------
# Modelo mínimo del esquema (a partir de la IR)
# --------------------------------------------------------------------------


@dataclass
class Column:
    name: str
    ctype: Dict[str, Any]          # canonical_type de la IR, p. ej. {"type": "int32"}
    nullable: bool
    max_length: Optional[int] = None
    unique: bool = False


@dataclass
class Table:
    name: str
    columns: List[Column]
    pk: List[str] = field(default_factory=list)
    composite_uniques: List[List[str]] = field(default_factory=list)

    def col(self, name: str) -> Column:
        return next(c for c in self.columns if c.name == name)


def model_from_ir(ir: Dict[str, Any]) -> List[Table]:
    tables = []
    for e in ir.get("entities", []):
        cols = []
        for f in e.get("fields", []):
            cons = {c.get("constraint_type"): c for c in f.get("constraints", []) or []}
            cols.append(Column(
                name=f["field_name"],
                ctype=dict(f["canonical_type"]),
                nullable=bool(f.get("nullable", True)),
                max_length=cons.get("MaxLength", {}).get("value"),
                unique="Unique" in cons,
            ))
        t = Table(name=e["entity_name"], columns=cols)
        names = {c.name for c in cols}
        for k in e.get("keys", []) or []:
            fields = [n for n in k.get("field_names", []) if n in names]
            if k.get("key_type") == "primary" and fields:
                t.pk = fields
            elif k.get("key_type") == "unique" and len(fields) == 1:
                t.col(fields[0]).unique = True
            elif k.get("key_type") == "unique" and len(fields) > 1:
                t.composite_uniques.append(fields)
        tables.append(t)
    return tables


# --------------------------------------------------------------------------
# Render a DDL MySQL
# --------------------------------------------------------------------------

DEFAULT_VARCHAR = 255


def _family(ctype: Dict[str, Any]) -> str:
    return str(ctype.get("type", "unknown"))


def sql_type(col: Column, indexed: bool) -> str:
    t = _family(col.ctype)
    if t == "string" or (t in ("text", "unknown", "json") and indexed):
        return f"VARCHAR({col.max_length or DEFAULT_VARCHAR})"
    return {
        "int32": "INT", "int64": "BIGINT", "float32": "FLOAT", "float64": "DOUBLE",
        "boolean": "TINYINT(1)", "text": "TEXT", "binary": "BLOB", "date": "DATE",
        "time": "TIME", "date_time": "DATETIME", "timestamp": "TIMESTAMP",
        "json": "JSON", "uuid": "CHAR(36)",
    }.get(t, "TEXT") if t != "decimal" else (
        f"DECIMAL({col.ctype.get('precision', 10)},{col.ctype.get('scale', 0)})")


def render(tables: List[Table]) -> str:
    """DDL MySQL sin claves foráneas: no se comparan y complican agregar o quitar tablas."""
    out = []
    for t in tables:
        indexed = set(t.pk) | {c.name for c in t.columns if c.unique} | {
            n for u in t.composite_uniques for n in u}
        lines = []
        for c in t.columns:
            null = "NULL" if c.nullable else "NOT NULL"
            lines.append(f"  `{c.name}` {sql_type(c, c.name in indexed)} {null}")
        if t.pk:
            lines.append("  PRIMARY KEY (" + ", ".join(f"`{n}`" for n in t.pk) + ")")
        for c in t.columns:
            if c.unique and c.name not in t.pk:
                lines.append(f"  UNIQUE KEY (`{c.name}`)")
        for u in t.composite_uniques:
            lines.append("  UNIQUE KEY (" + ", ".join(f"`{n}`" for n in u) + ")")
        out.append(f"CREATE TABLE `{t.name}` (\n" + ",\n".join(lines) + "\n);\n")
    return "\n".join(out)


# --------------------------------------------------------------------------
# Especificación de unificación (sección 5): conflicto esperado por cambio de tipo
# --------------------------------------------------------------------------

# (familia A, familia B) -> (sugerencia esperada, regla del catálogo)
TYPE_CHANGES: Dict[Tuple[str, str], Tuple[str, str]] = {
    ("int32", "int64"): ("CastSafe", "R-TYPE-001"),
    ("int64", "int32"): ("CastSafe", "R-TYPE-001"),  # "Int32 vs Int64": se unifica en Int64
    ("int32", "decimal"): ("CastSafe", "R-TYPE-002"),
    ("float64", "decimal"): ("CastRisky", "R-TYPE-003"),
    ("int32", "string"): ("CastSafe", "R-TYPE-004"),
    ("string", "int32"): ("CastRisky", "R-TYPE-005"),
    ("boolean", "int32"): ("CastSafe", "R-TYPE-006"),
    ("date", "date_time"): ("CastSafe", "R-TYPE-007"),
    ("date_time", "date"): ("CastRisky", "R-TYPE-008"),
}

TARGET_TYPES = {
    "int32": {"type": "int32"}, "int64": {"type": "int64"},
    "decimal": {"type": "decimal", "precision": 12, "scale": 2},
    "string": {"type": "string"}, "date": {"type": "date"}, "date_time": {"type": "date_time"},
}

# --------------------------------------------------------------------------
# Transformaciones de nombres (sección 12 del protocolo, 2026-10-19)
# --------------------------------------------------------------------------

# Sinónimos en inglés (por palabra)
SYNONYMS = {
    "customer": "client", "client": "customer", "phone": "telephone", "email": "mail",
    "description": "details", "first": "given", "last": "family", "zip": "postal",
    "postal": "zip", "country": "nation", "company": "firm", "employee": "staff",
    "product": "item", "item": "product", "order": "purchase", "invoice": "bill",
    "address": "location", "title": "heading", "price": "cost", "sale": "ticket",
}

# Abreviaturas habituales en esquemas heredados
ABBREVIATIONS = {
    "quantity": "qty", "number": "num", "description": "desc", "address": "addr",
    "amount": "amt", "telephone": "tel", "phone": "tel", "information": "info",
    "category": "cat", "customer": "cust", "product": "prod", "department": "dept",
    "employee": "emp", "reference": "ref", "account": "acct", "payment": "pmt",
    "transaction": "txn", "invoice": "inv", "maximum": "max", "minimum": "min",
    "identifier": "id", "position": "pos", "message": "msg", "configuration": "config",
}

# Traducción inglés -> español
TRANSLATIONS = {
    "status": "estado", "name": "nombre", "city": "ciudad", "address": "direccion",
    "phone": "telefono", "country": "pais", "total": "total_importe", "notes": "notas",
    "description": "descripcion", "email": "correo", "price": "precio", "date": "fecha",
    "customer": "cliente", "product": "producto", "quantity": "cantidad", "user": "usuario",
    "title": "titulo", "company": "empresa", "state": "estado", "code": "codigo",
    "order": "pedido", "invoice": "factura", "employee": "empleado", "type": "tipo",
}

# Palabras que cambian el significado: base de los distractores
NEAR_MISSES = {
    "at": "by", "by": "at", "start": "end", "end": "start", "from": "to", "to": "from",
    "min": "max", "max": "min", "first": "last", "last": "first", "in": "out",
    "out": "in", "old": "new", "new": "old", "buy": "sell", "sell": "buy",
    "debit": "credit", "credit": "debit", "open": "close", "close": "open",
    "gross": "net", "net": "gross", "created": "updated", "updated": "created",
    "birth": "hire", "hire": "birth", "billing": "shipping", "shipping": "billing",
}


def _tokens(name: str) -> List[str]:
    return [t for t in re.split(r"[_\s]+|(?<=[a-z0-9])(?=[A-Z])", name) if t]


def _replace_token(name: str, table: Dict[str, str]) -> Optional[str]:
    toks = _tokens(name)
    for i, tok in enumerate(toks):
        low = tok.lower()
        for base, suffix in ((low, ""), (low[:-1], "s") if low.endswith("s") else (low, "")):
            if base in table:
                new = toks[:i] + [table[base] + suffix] + toks[i + 1:]
                out = "_".join(t.lower() for t in new)
                return out if out.lower() != name.lower() else None
    return None


def to_style(name: str) -> Optional[str]:
    toks = _tokens(name)
    if len(toks) > 1:
        if "_" in name:
            return "".join(t[:1].upper() + t[1:].lower() for t in toks)  # snake -> Pascal
        return "_".join(t.lower() for t in toks)  # camel/Pascal -> snake
    return None


def to_plural(name: str) -> Optional[str]:
    toks = _tokens(name)
    if not toks:
        return None
    last = toks[-1]
    low = last.lower()
    if low.endswith("ies") and len(low) > 4:
        new = last[:-3] + "y"
    elif low.endswith("s") and not low.endswith("ss") and len(low) > 3:
        new = last[:-1]
    elif low.endswith("y") and len(low) > 2 and low[-2] not in "aeiou":
        new = last[:-1] + "ies"
    else:
        new = last + "s"
    sep = "_" if "_" in name else ""
    return sep.join(toks[:-1] + [new]) if len(toks) > 1 else new


def to_affix(name: str, table: str) -> Optional[str]:
    prefix = re.sub(r"[^a-z]", "", table.lower())[:3]
    return f"{prefix}_{name.lower()}" if prefix and not name.lower().startswith(prefix + "_") else None


RENAMES: Dict[str, Callable[..., Optional[str]]] = {
    "style": lambda name, table: to_style(name),
    "plural": lambda name, table: to_plural(name),
    "affix": lambda name, table: to_affix(name, table),
    "abbreviation": lambda name, table: _replace_token(name, ABBREVIATIONS),
    "synonym": lambda name, table: _replace_token(name, SYNONYMS),
    "translation": lambda name, table: _replace_token(name, TRANSLATIONS),
}

# Nivel de dificultad de cada categoría de renombrado
RENAME_LEVEL = {"style": "facil", "plural": "facil", "affix": "medio",
                "abbreviation": "dificil", "synonym": "dificil", "translation": "dificil"}

# Cambios que la especificación considera equivalentes: no deben producir conflicto
EQUIVALENT_CHANGES = {
    "string": ({"type": "text"}, None),           # VARCHAR(n) -> TEXT
    "date_time": ({"type": "timestamp"}, None),   # DATETIME -> TIMESTAMP
    "timestamp": ({"type": "date_time"}, None),
}


# --------------------------------------------------------------------------
# Cobertura del dominio pos (réplica de la estrategia CatalogEquivalence del núcleo)
# --------------------------------------------------------------------------


def _norm_tokens(name: str) -> List[str]:
    def singular(t: str) -> str:
        if t.endswith("ies") and len(t) > 3:
            return t[:-3] + "y"
        if len(t) > 3 and t.endswith("s") and not t.endswith("ss"):
            return t[:-1]
        return t
    snake = re.sub(r"(?<=[a-z0-9])(?=[A-Z])", "_", name.strip()).lower()
    return [singular(t) for t in re.split(r"[^a-z0-9]+", snake) if t]


def load_domain_groups(path: Path) -> List[List[str]]:
    if not path.exists():
        return []
    with open(path, "rb") as f:
        data = tomllib.load(f)
    return [[e["canonical"], *e.get("aliases", [])] for e in data.get("equivalence", [])]


def domain_canonical(name: str, groups: List[List[str]]) -> str:
    key = lambda s: "_".join(_norm_tokens(s))  # noqa: E731
    lookup = {key(alias): key(g[0]) for g in groups for alias in g}
    whole = key(name)
    if whole in lookup:
        return lookup[whole]
    return "_".join(lookup.get(t, t) for t in _norm_tokens(name))


# --------------------------------------------------------------------------
# Mutaciones
# --------------------------------------------------------------------------


@dataclass
class Mutation:
    operator: str
    table: str                       # tabla en A
    column: Optional[str] = None     # columna en A (o columna afectada)
    detail: Dict[str, Any] = field(default_factory=dict)
    conflict: Optional[Dict[str, Any]] = None
    level: str = "facil"
    category: Optional[str] = None


class Mutator:
    def __init__(self, tables: List[Table], rng: random.Random):
        self.a = tables
        self.b = copy.deepcopy(tables)
        self.rng = rng
        self.table_map = {t.name: t.name for t in tables}           # A -> B (None = eliminada)
        self.col_map = {(t.name, c.name): c.name for t in tables for c in t.columns}
        self.touched: set = set()                                    # (tabla A, columna A)
        self.added_tables: List[str] = []
        self.added_cols: List[Tuple[str, str]] = []                   # (tabla A, columna B)
        self.no_match: Dict[Tuple[str, str], str] = {}               # (tabla A, col A) -> categoría
        self.match_cat: Dict[Tuple[str, Optional[str]], str] = {}    # categoría de la coincidencia
        self.log: List[Mutation] = []

    # ---- utilidades
    def b_table(self, a_name: str) -> Table:
        return next(t for t in self.b if t.name == self.table_map[a_name])

    def live_tables(self) -> List[Table]:
        return [t for t in self.a if self.table_map[t.name] is not None]

    def candidates(self, pred: Callable[[Table, Column], bool]) -> List[Tuple[Table, Column]]:
        out = []
        for t in self.live_tables():
            for c in t.columns:
                if (t.name, c.name) in self.touched or self.col_map.get((t.name, c.name)) is None:
                    continue
                if pred(t, c):
                    out.append((t, c))
        return out

    def pick(self, items):
        return self.rng.choice(items) if items else None

    def b_col(self, t: Table, c: Column) -> Column:
        return self.b_table(t.name).col(self.col_map[(t.name, c.name)])

    def conflict(self, t: Table, c: Column, ctype: str, suggestion: str, rule: str, **extra):
        return {"entity": t.name, "field": c.name, "entity_b": self.table_map[t.name],
                "field_b": self.col_map[(t.name, c.name)], "type": ctype,
                "expected_suggestion": suggestion, "rule": rule, **extra}

    def free_name(self, t: Table, name: Optional[str]) -> bool:
        return bool(name) and all(x.name.lower() != name.lower() for x in self.b_table(t.name).columns)

    # ---- cambios que producen conflicto (se reutilizan en los combinados)
    def _type_ok(self, t, c):
        return c.name not in t.pk and any(_family(c.ctype) == a for a, _ in TYPE_CHANGES)

    def _apply_type(self, t: Table, c: Column) -> Dict[str, Any]:
        options = sorted(b for a, b in TYPE_CHANGES if a == _family(c.ctype))
        target = self.rng.choice(options)
        suggestion, rule = TYPE_CHANGES[(_family(c.ctype), target)]
        bc = self.b_col(t, c)
        bc.ctype = dict(TARGET_TYPES[target])
        bc.max_length = 20 if target == "string" else None
        return self.conflict(t, c, "TypeIncompatible", suggestion, rule,
                             change={"from": _family(c.ctype), "to": target})

    def _length_ok(self, t, c):
        return _family(c.ctype) == "string" and bool(c.max_length)

    def _apply_length(self, t: Table, c: Column) -> Dict[str, Any]:
        new = c.max_length * 2 if self.rng.random() < 0.5 or c.max_length < 2 else max(1, c.max_length // 2)
        self.b_col(t, c).max_length = new
        return self.conflict(t, c, "LengthMismatch", "PreferType", "R-LEN-001",
                             change={"from": c.max_length, "to": new})

    def _nullable_ok(self, t, c):
        return c.name not in t.pk

    def _apply_nullable(self, t: Table, c: Column) -> Dict[str, Any]:
        self.b_col(t, c).nullable = not c.nullable
        return self.conflict(t, c, "NullabilityMismatch", "PreferType", "R-NULL-001",
                             change={"from": c.nullable, "to": not c.nullable})

    def _simple(self, operator, ok, apply) -> bool:
        tc = self.pick(self.candidates(ok))
        if not tc:
            return False
        t, c = tc
        conflict = apply(t, c)
        self.touched.add((t.name, c.name))
        conflict["level"] = "facil"
        self.log.append(Mutation(operator, t.name, c.name, conflict.get("change", {}), conflict))
        return True

    # ---- nivel fácil: un cambio estructural (sección 7.3)
    def change_type(self) -> bool:
        return self._simple("change_type", self._type_ok, self._apply_type)

    def change_length(self) -> bool:
        return self._simple("change_length", self._length_ok, self._apply_length)

    def toggle_nullable(self) -> bool:
        return self._simple("toggle_nullable", self._nullable_ok, self._apply_nullable)

    def toggle_unique(self) -> bool:
        indexable = {"int32", "int64", "string", "decimal", "date", "date_time", "boolean"}

        def apply(t, c):
            self.b_col(t, c).unique = not c.unique
            return self.conflict(t, c, "ConstraintMismatch", "PreferType", "R-CONS-001",
                                 change={"from": c.unique, "to": not c.unique})
        return self._simple(
            "toggle_unique",
            lambda t, c: c.name not in t.pk and _family(c.ctype) in indexable
            and not any(c.name in u for u in t.composite_uniques),
            apply)

    def add_field(self) -> bool:
        t = self.pick(self.live_tables())
        if not t:
            return False
        new = Column(name=f"extra_{self.rng.randint(1000, 9999)}", ctype={"type": "string"},
                     nullable=True, max_length=100)
        self.b_table(t.name).columns.append(new)
        self.added_cols.append((t.name, new.name))
        self.log.append(Mutation("add_field", t.name, new.name))
        return True

    def drop_field(self) -> bool:
        def ok(t, c):
            return (c.name not in t.pk and len(t.columns) > 2
                    and not any(c.name in u for u in t.composite_uniques))
        tc = self.pick(self.candidates(ok))
        if not tc:
            return False
        t, c = tc
        bt = self.b_table(t.name)
        bt.columns = [x for x in bt.columns if x.name != self.col_map[(t.name, c.name)]]
        self.col_map[(t.name, c.name)] = None
        self.touched.add((t.name, c.name))
        self.log.append(Mutation("drop_field", t.name, c.name))
        return True

    def add_table(self) -> bool:
        name = f"aux_{self.rng.randint(1000, 9999)}"
        self.b.append(Table(name=name, columns=[
            Column("id", {"type": "int32"}, False),
            Column("label", {"type": "string"}, True, max_length=50)], pk=["id"]))
        self.added_tables.append(name)
        self.log.append(Mutation("add_table", name))
        return True

    def drop_table(self) -> bool:
        cands = [t for t in self.live_tables()
                 if not any((t.name, c.name) in self.touched for c in t.columns)
                 and self.table_map[t.name] == t.name]
        if len(self.live_tables()) < 3 or not cands:
            return False
        t = self.pick(cands)
        self.b = [x for x in self.b if x.name != self.table_map[t.name]]
        self.table_map[t.name] = None
        self.log.append(Mutation("drop_table", t.name))
        return True

    def rename_unrelated(self) -> bool:
        tc = self.pick(self.candidates(lambda t, c: c.name not in t.pk))
        if not tc:
            return False
        t, c = tc
        new = f"fld_{self.rng.randint(1000, 9999)}"
        self._rename_col(t, c, new)
        self.no_match[(t.name, c.name)] = "unrelated"
        self.log.append(Mutation("rename_unrelated", t.name, c.name, {"to": new},
                                 level="dificil", category="unrelated"))
        return True

    # ---- renombrados por categoría (campos)
    def _rename_col(self, t: Table, c: Column, new: str) -> None:
        bt = self.b_table(t.name)
        old = self.col_map[(t.name, c.name)]
        bt.col(old).name = new
        bt.pk = [new if n == old else n for n in bt.pk]
        bt.composite_uniques = [[new if n == old else n for n in u] for u in bt.composite_uniques]
        self.col_map[(t.name, c.name)] = new
        self.touched.add((t.name, c.name))

    def _rename_candidates(self, category: str):
        fn = RENAMES[category]
        return [(t, c, fn(c.name, t.name)) for t, c in self.candidates(lambda t, c: True)
                if self.free_name(t, fn(c.name, t.name))]

    def rename_field(self) -> bool:
        category = self.rng.choice(sorted(RENAMES))
        cand = self.pick(self._rename_candidates(category))
        if not cand:
            return False
        t, c, new = cand
        self._rename_col(t, c, new)
        self.match_cat[(t.name, c.name)] = category
        self.log.append(Mutation(f"rename_{category}", t.name, c.name, {"to": new},
                                 level=RENAME_LEVEL[category], category=category))
        return True

    # ---- renombrados de tabla
    def rename_table(self) -> bool:
        category = self.rng.choice(["style", "plural", "affix", "synonym", "translation"])
        fn = RENAMES[category]
        cands = []
        for t in self.live_tables():
            new = fn(t.name, "tbl")
            if self.table_map[t.name] == t.name and new and all(
                    x.name.lower() != new.lower() for x in self.b):
                cands.append((t, new))
        cand = self.pick(cands)
        if not cand:
            return False
        t, new = cand
        self.b_table(t.name).name = new
        self.table_map[t.name] = new
        self.match_cat[(t.name, None)] = category
        self.log.append(Mutation(f"rename_table_{category}", t.name, None, {"to": new},
                                 level=RENAME_LEVEL[category], category=category))
        return True

    # ---- nivel medio
    def combined(self) -> bool:
        """Renombrar y además cambiar tipo, longitud o nulabilidad del mismo campo."""
        changes = [("type", self._type_ok, self._apply_type),
                   ("length", self._length_ok, self._apply_length),
                   ("nullable", self._nullable_ok, self._apply_nullable)]
        kind, ok, apply = self.rng.choice(changes)
        category = self.rng.choice(["style", "plural", "affix", "abbreviation", "synonym"])
        fn = RENAMES[category]
        cands = [(t, c, fn(c.name, t.name)) for t, c in self.candidates(ok)
                 if self.free_name(t, fn(c.name, t.name))]
        cand = self.pick(cands)
        if not cand:
            return False
        t, c, new = cand
        self._rename_col(t, c, new)
        conflict = apply(t, c)
        level = "dificil" if RENAME_LEVEL[category] == "dificil" else "medio"
        conflict["level"] = level
        self.match_cat[(t.name, c.name)] = category
        self.log.append(Mutation(f"combined_{category}_{kind}", t.name, c.name,
                                 {"to": new, **conflict.get("change", {})}, conflict,
                                 level=level, category=category))
        return True

    def equivalent_change(self) -> bool:
        """VARCHAR -> TEXT o DATETIME <-> TIMESTAMP: la especificación no lo considera conflicto."""
        tc = self.pick(self.candidates(
            lambda t, c: _family(c.ctype) in EQUIVALENT_CHANGES and c.name not in t.pk
            and not c.unique and not any(c.name in u for u in t.composite_uniques)))
        if not tc:
            return False
        t, c = tc
        ctype, max_length = EQUIVALENT_CHANGES[_family(c.ctype)]
        bc = self.b_col(t, c)
        bc.ctype = dict(ctype)
        bc.max_length = max_length
        self.touched.add((t.name, c.name))
        self.log.append(Mutation("equivalent_change", t.name, c.name,
                                 {"from": _family(c.ctype), "to": ctype["type"]},
                                 level="medio", category="equivalent"))
        return True

    def normalization_collision(self) -> bool:
        def ok(t, c):
            v = to_style(c.name)
            return "_" in c.name and v is not None and self.free_name(t, v)
        tc = self.pick(self.candidates(ok))
        if not tc:
            return False
        t, c = tc
        new = Column(name=to_style(c.name), ctype=dict(c.ctype), nullable=True, max_length=c.max_length)
        self.b_table(t.name).columns.append(new)
        self.touched.add((t.name, c.name))
        self.added_cols.append((t.name, new.name))
        conflict = {**self.conflict(t, c, "NormalizationCollision", "RenameField", "R-NORM-001"),
                    "field_b": new.name, "level": "medio"}
        self.log.append(Mutation("normalization_collision", t.name, c.name, {"added_b": new.name},
                                 conflict, level="medio", category="collision"))
        return True

    # ---- nivel difícil
    def distractor(self) -> bool:
        """Sustituir un campo por otro de nombre casi igual y significado distinto."""
        def near(c):
            return _replace_token(c.name, NEAR_MISSES) or f"{c.name.lower()}_old"
        cands = [(t, c, near(c)) for t, c in self.candidates(lambda t, c: c.name not in t.pk)
                 if self.free_name(t, near(c))]
        cand = self.pick(cands)
        if not cand:
            return False
        t, c, new = cand
        self._rename_col(t, c, new)
        self.no_match[(t.name, c.name)] = "distractor"
        self.log.append(Mutation("distractor", t.name, c.name, {"to": new},
                                 level="dificil", category="distractor"))
        return True

    OPERATORS = ["change_type", "change_length", "toggle_nullable", "toggle_unique",
                 "add_field", "drop_field", "add_table", "drop_table",
                 "rename_field", "rename_field", "rename_table",
                 "combined", "combined", "equivalent_change", "normalization_collision",
                 "distractor", "rename_unrelated"]

    def run(self, n: int) -> None:
        attempts = 0
        while len(self.log) < n and attempts < n * 20:
            attempts += 1
            getattr(self, self.rng.choice(self.OPERATORS))()


# --------------------------------------------------------------------------
# Verdad de referencia
# --------------------------------------------------------------------------


def build_truth(pair_id: str, base: str, seed: int, m: Mutator,
                domain_groups: List[List[str]]) -> Dict[str, Any]:
    matches, exclusives = [], []

    def in_domain(a: str, b: str) -> bool:
        return bool(domain_groups) and domain_canonical(a, domain_groups) == domain_canonical(b, domain_groups)

    for t in m.a:
        tb = m.table_map[t.name]
        if tb is None:
            exclusives.append({"side": "a", "entity": t.name, "field": None})
            continue
        cat = m.match_cat.get((t.name, None), "unchanged")
        matches.append({"a": t.name, "b": tb, "category": cat,
                        **({"in_domain": in_domain(t.name, tb)} if cat != "unchanged" else {})})
        for c in t.columns:
            cb = m.col_map[(t.name, c.name)]
            if cb is None:
                exclusives.append({"side": "a", "entity": t.name, "field": c.name})
            elif (t.name, c.name) in m.no_match:
                cat = m.no_match[(t.name, c.name)]
                exclusives.append({"side": "a", "entity": t.name, "field": c.name, "category": cat})
                exclusives.append({"side": "b", "entity": tb, "field": cb, "category": cat})
            else:
                cat = m.match_cat.get((t.name, c.name), "unchanged")
                matches.append({"a": f"{t.name}.{c.name}", "b": f"{tb}.{cb}", "category": cat,
                                **({"in_domain": in_domain(c.name, cb)} if cat != "unchanged" else {})})
    for ta, cb in m.added_cols:
        if m.table_map[ta] is not None:
            exclusives.append({"side": "b", "entity": m.table_map[ta], "field": cb})
    for tb in m.added_tables:
        exclusives.append({"side": "b", "entity": tb, "field": None})
    conflicts = []
    for x in m.log:
        if not x.conflict:
            continue
        c = {k: v for k, v in x.conflict.items() if k != "change"}
        # Nombres finales en B: una mutación posterior pudo renombrar la tabla
        c["entity_b"] = m.table_map[x.table]
        if x.operator != "normalization_collision":
            c["field_b"] = m.col_map[(x.table, x.column)]
        conflicts.append(c)
    return {
        "pair_id": pair_id, "block": "B", "base": base, "seed": seed,
        "spec": "unificación simétrica (PROTOCOLO_EVALUACION.md, secciones 5 y 12)",
        "matches": matches, "exclusives": exclusives, "conflicts": conflicts,
        "mutations": [asdict(x) for x in m.log],
    }


# --------------------------------------------------------------------------
# Ejecución
# --------------------------------------------------------------------------


def base_ir(cfg: Dict[str, Any], base: Dict[str, Any], work: Path) -> Dict[str, Any]:
    src = base.get("ddl")
    src = str((EVAL_DIR / src).resolve()) if src else source_of(base)
    out = work / f"{base['id']}.ir.json"
    if run_inspect(cfg, src, out) != 0:
        raise SystemExit(f"audd inspect falló para la base {base['id']} ({src})")
    return load_json(out)


def _portable(path: Path) -> str:
    """Ruta relativa al proyecto cuando es posible (los pares se versionan)."""
    path = path.resolve()
    if path.is_relative_to(PROJECT_ROOT):
        path = path.relative_to(PROJECT_ROOT)
    return str(path).replace("\\", "/")


def generate(cfg, base: Dict[str, Any], seed: int, backend: str, out_root: Path, ir: Dict[str, Any]) -> Path:
    pair_id = f"{base['id']}_s{seed}"
    rng = random.Random(f"{base['id']}:{seed}")
    tables = model_from_ir(ir)
    m = Mutator(tables, rng)
    m.run(rng.randint(MIN_CHANGES, MAX_CHANGES))

    d = out_root / pair_id
    if d.exists():
        shutil.rmtree(d)
    d.mkdir(parents=True)
    ddl_a, ddl_b = render(m.a), render(m.b)
    (d / "a.sql").write_text(ddl_a, encoding="utf-8")
    (d / "b.sql").write_text(ddl_b, encoding="utf-8")
    truth = build_truth(pair_id, base["id"], seed, m,
                        load_domain_groups(PROJECT_ROOT / "catalog" / "domains" / "pos.toml"))
    write_json(d / "truth.json", truth)

    if backend == "mysql":
        import mysql  # noqa: E402  (suite/mysql.py)
        sources = {}
        for side, ddl in (("a", ddl_a), ("b", ddl_b)):
            db = f"audd_m_{pair_id}_{side}".lower()
            mysql.recreate_database(db)
            mysql.load_sql(db, ddl)
            sources[f"source_{side}"] = mysql.source_url(db)
    else:
        sources = {f"source_{s}": _portable(d / f"{s}.sql") for s in ("a", "b")}
    write_json(d / "pair.json", {"backend": backend, **sources})
    print(f"{pair_id}: {len(m.log)} mutaciones, {len(truth['conflicts'])} conflictos esperados")
    return d


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--base", nargs="*", help=f"ids de base (por defecto: {' '.join(DEFAULT_BASES)})")
    ap.add_argument("--seed", type=int, nargs="*", help="semillas (por defecto 1000-1009)")
    ap.add_argument("--all", action="store_true", help="todas las bases y semillas por defecto")
    ap.add_argument("--backend", choices=["mysql", "ddl"], default="mysql")
    ap.add_argument("--out", type=Path, default=GENERATED_DIR)
    args = ap.parse_args()
    if not args.all and not args.base:
        ap.error("indica --base o --all")

    cfg = load_config()
    bases = bases_by_id(cfg)
    ids = args.base or DEFAULT_BASES
    seeds = args.seed or DEFAULT_SEEDS
    with tempfile.TemporaryDirectory() as tmp:
        for bid in ids:
            if bid not in bases:
                raise SystemExit(f"Base desconocida: {bid}")
            ir = base_ir(cfg, bases[bid], Path(tmp))
            for s in seeds:
                generate(cfg, bases[bid], s, args.backend, args.out, ir)
    return 0


if __name__ == "__main__":
    sys.exit(main())
