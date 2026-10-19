#!/usr/bin/env python3
"""Valida la verdad de referencia de los pares mutados contra la IR de entrada.

Problema que resuelve: el generador escribe la verdad en términos de DDL
(p. ej. "VARCHAR(20) -> TEXT es un conflicto de tipo"), pero AUDD compara la
IR que produce cada adaptador. Si el adaptador no conserva lo que se mutó
(SQLite no guarda longitudes; INTEGER y BIGINT son ambos Int64), la mutación
no existe para el núcleo y contarla como falso negativo sería injusto; si se
omitiera en silencio, el resultado sería opaco.

Este script NO usa el motor de comparación: lee la IR de A y de B
(`audd inspect`) y, para cada conflicto de la verdad, verifica que la
diferencia exista en la IR. Marca cada conflicto con `status`:

    observable        la diferencia está en la IR -> cuenta para PCD
    not_observable    el adaptador perdió la diferencia -> se excluye y se reporta
    label_review      hay diferencia, pero de otro tipo que la etiqueta -> revisar a mano

Uso:
    python evaluation/suite/validate_truth.py --pair <dir_par> --ir-a <a.ir.json> --ir-b <b.ir.json>
Escribe <dir_par>/truth.validated.json, que score.py usa en lugar de truth.json.
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path
from typing import Any, Dict, Optional

sys.path.insert(0, str(Path(__file__).parent))
from common import load_json, norm, norm_type, write_json  # noqa: E402

# Tipos canónicos que la especificación trata como equivalentes (no generan conflicto).
EQUIVALENT = [{"string", "text"}, {"datetime", "timestamp"}]


def _ctype(field: Dict[str, Any]) -> str:
    """Tipo canónico normalizado: la IR serializa `date_time`; la especificación dice DateTime."""
    t = field.get("canonical_type")
    if isinstance(t, dict):
        base = norm(str(t.get("type", "")))
        if base == "decimal":
            return f"decimal({t.get('precision')},{t.get('scale')})"
        if base == "unknown":
            return f"unknown({t.get('original_type')})"
        return base
    return norm(str(t))


def _constraints(field: Dict[str, Any]) -> Dict[str, Any]:
    out: Dict[str, Any] = {}
    for c in field.get("constraints", []) or []:
        kind = str(c.get("constraint_type", "")).lower()
        out[kind] = c.get("value", c.get("expression", True))
    return out


def _find(ir: Dict[str, Any], entity: str, field: Optional[str]):
    for e in ir.get("entities", []):
        if norm(e.get("entity_name")) == norm(entity):
            if not field:
                return e, None
            for f in e.get("fields", []):
                if norm(f.get("field_name")) == norm(field):
                    return e, f
            return e, None
    return None, None


def _unique_in_keys(entity: Dict[str, Any], field_name: str) -> bool:
    for k in entity.get("keys", []) or []:
        cols = [norm(c) for c in k.get("field_names", k.get("fields", []))]
        # Solo claves UNIQUE (E3). La pertenencia a la PK es E4 y no se compara.
        if str(k.get("key_type", "")).lower() == "unique" and cols == [norm(field_name)]:
            return True
    return False


def check(conflict: Dict[str, Any], ir_a: Dict[str, Any], ir_b: Dict[str, Any]) -> str:
    t = norm_type(conflict.get("type"))
    ea, fa = _find(ir_a, conflict.get("entity", ""), conflict.get("field"))
    eb, fb = _find(ir_b, conflict.get("entity_b") or conflict.get("entity", ""),
                   conflict.get("field_b") or conflict.get("field"))
    if t == "normalizationcollision":
        if eb is None:
            return "not_observable"
        names = [norm(f.get("field_name")) for f in eb.get("fields", [])]
        return "observable" if len(names) != len(set(names)) else "not_observable"
    if fa is None or fb is None:
        return "not_observable"

    ta, tb = _ctype(fa), _ctype(fb)
    type_diff = ta != tb and not any({ta, tb} <= eq for eq in EQUIVALENT)
    null_diff = bool(fa.get("nullable")) != bool(fb.get("nullable"))
    ca, cb = _constraints(fa), _constraints(fb)
    len_diff = "maxlength" in ca and "maxlength" in cb and ca["maxlength"] != cb["maxlength"]
    uniq_a = "unique" in ca or _unique_in_keys(ea, fa["field_name"])
    uniq_b = "unique" in cb or _unique_in_keys(eb, fb["field_name"])
    cons_diff = (uniq_a != uniq_b) or (ca.get("check") != cb.get("check"))

    observed = {
        "typeincompatible": type_diff,
        "nullabilitymismatch": null_diff,
        "lengthmismatch": len_diff,
        "constraintmismatch": cons_diff,
    }
    if observed.get(t):
        return "observable"
    if any(observed.values()):
        return "label_review"
    return "not_observable"


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--pair", required=True, type=Path)
    ap.add_argument("--ir-a", required=True, type=Path)
    ap.add_argument("--ir-b", required=True, type=Path)
    args = ap.parse_args()

    truth = load_json(args.pair / "truth.json")
    ir_a, ir_b = load_json(args.ir_a), load_json(args.ir_b)
    counts: Dict[str, int] = {}
    for c in truth.get("conflicts", []):
        c["status"] = check(c, ir_a, ir_b)
        counts[c["status"]] = counts.get(c["status"], 0) + 1
    truth["validation"] = {"method": "IR de entrada (audd inspect), sin motor de comparación",
                           "counts": counts}
    write_json(args.pair / "truth.validated.json", truth)
    print(f"{args.pair.name}: {counts}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
