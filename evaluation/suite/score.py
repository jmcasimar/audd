#!/usr/bin/env python3
"""Calcula los indicadores de eficacia de un par contra su verdad de referencia.

Uso:
    python evaluation/suite/score.py --truth <truth.json> --run <dir_salida_audd> [--ir-b <ir_b.json>]

Indicadores (definiciones del apartado 5.3 de la tesis):
    PCD = VP / (VP + FN)          solo tipos estructurales
    PRE = VP / (VP + FP)
    PSR = VP con sugerencia del tipo esperado / VP con sugerencia esperada definida
    F1  = 2PR / (P + R) de las coincidencias

Regla de conteo: un conflicto es VP cuando coinciden entidad, campo y tipo
(nombres normalizados: minúsculas, sin guiones bajos ni espacios). Se acepta
el nombre del lado A o el del lado B. Cada conflicto de la verdad se empareja
una sola vez.
"""

from __future__ import annotations

import argparse
import json
import sys
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any, Dict, List, Optional

sys.path.insert(0, str(Path(__file__).parent))
from common import (STRUCTURAL_TYPES, entity_scope_ok, load_json, norm, norm_type,  # noqa: E402
                    resolve_b_names)


def _split(ref: str):
    if "." in ref:
        e, f = ref.split(".", 1)
        return norm(e), norm(f)
    return norm(ref), ""


def _ratio(num: int, den: int) -> Optional[float]:
    return round(100.0 * num / den, 2) if den else None


def score_pair(truth: Dict[str, Any], run_dir: Path, ir_b: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
    diff = load_json(run_dir / "diff.json")
    sugg_path = run_dir / "suggestions.json"
    suggestions = load_json(sugg_path) if sugg_path.exists() else []

    scope = truth.get("scope") or {}
    scope_a, scope_b = scope.get("a"), scope.get("b")

    # ---------------- Conflictos ----------------
    det = []
    for c in diff.get("conflicts", []):
        if not entity_scope_ok(c.get("entity_name", ""), scope_a):
            continue
        det.append({"entity": norm(c.get("entity_name")), "field": norm(c.get("field_name")),
                    "type": norm_type(c.get("conflict_type")), "raw": c})

    truth_c, excluded = [], []
    for c in truth.get("conflicts", []):
        if not entity_scope_ok(c.get("entity", ""), scope_a):
            continue
        if c.get("status") in ("not_observable", "label_review", "discarded"):
            excluded.append({**c, "reason": c.get("status")})
            continue
        if norm_type(c.get("type")) not in STRUCTURAL_TYPES:
            excluded.append({**c, "reason": "no_estructural"})
            continue
        truth_c.append(c)

    used = set()
    per_type = defaultdict(Counter)
    vp_list, fn_list = [], []
    for c in truth_c:
        t = norm_type(c["type"])
        keys = {(norm(c.get("entity")), norm(c.get("field")))}
        if c.get("entity_b"):
            keys.add((norm(c.get("entity_b")), norm(c.get("field_b"))))
        hit = None
        for i, d in enumerate(det):
            if i in used or d["type"] != t:
                continue
            if (d["entity"], d["field"]) in keys:
                hit = i
                break
        if hit is None:
            per_type[t]["FN"] += 1
            fn_list.append(c)
        else:
            used.add(hit)
            per_type[t]["VP"] += 1
            vp_list.append((c, det[hit]))
    fp_list = [d["raw"] for i, d in enumerate(det) if i not in used]
    for d in (det[i] for i in range(len(det)) if i not in used):
        per_type[d["type"]]["FP"] += 1

    VP, FN, FP = len(vp_list), len(fn_list), len(fp_list)

    # ---------------- Sugerencias (PSR) ----------------
    sugg_idx = defaultdict(set)
    for s in suggestions:
        k = (norm(s.get("entity_name")), norm(s.get("field_name")), norm_type(s.get("conflict_type")))
        sugg_idx[k].add(norm(s.get("suggestion_kind")))
    psr_hit, psr_den, psr_miss = 0, 0, []
    for c, d in vp_list:
        exp = c.get("expected_suggestion")
        if not exp:
            continue
        psr_den += 1
        kinds = sugg_idx.get((d["entity"], d["field"], d["type"]), set())
        ok = norm(exp) in kinds or (norm(exp) == "nosuggestion" and not (kinds - {"nosuggestion"}))
        if ok:
            psr_hit += 1
        else:
            psr_miss.append({"entity": c.get("entity"), "field": c.get("field"), "type": c["type"],
                             "expected": exp, "got": sorted(kinds)})

    # ---------------- Coincidencias (F1) ----------------
    bmap = resolve_b_names(diff, ir_b)
    det_m = {}
    for m in diff.get("matches", []):
        if not entity_scope_ok(m.get("entity_name", ""), scope_a):
            continue
        ka = (norm(m.get("entity_name")), norm(m.get("field_name")))
        det_m[ka] = bmap.get(ka)
    truth_m, truth_cat = {}, {}
    for m in truth.get("matches", []):
        ka = _split(m["a"])
        if scope_a and not entity_scope_ok(ka[0], scope_a):
            continue
        truth_m[ka] = _split(m["b"]) if m.get("b") else None
        truth_cat[ka] = m.get("category", "unchanged")
    # Elementos de A que NO deben emparejarse (distractores, renombrados sin relación)
    no_match_cat = {(norm(x.get("entity")), norm(x.get("field"))): x["category"]
                    for x in truth.get("exclusives", [])
                    if str(x.get("side")).lower() == "a" and x.get("category")}

    # Conteos por categoría: un VP exige el campo correcto de B; emparejar con el
    # campo equivocado cuenta como FP y FN; emparejar algo que no debía, como FP
    by_cat = defaultdict(Counter)
    m_vp = 0
    for ka, tb in truth_m.items():
        cat = truth_cat[ka]
        db = det_m.get(ka, "missing")
        if db == "missing":
            by_cat[cat]["FN"] += 1
            continue
        if tb is not None and db is not None and (norm(db[0]), norm(db[1])) != tb:
            by_cat[cat]["FN"] += 1  # coincidencia con el campo equivocado del lado B
            by_cat[cat]["FP"] += 1
            continue
        by_cat[cat]["VP"] += 1
        m_vp += 1
    for ka in det_m:
        if ka not in truth_m:
            by_cat[no_match_cat.get(ka, "other")]["FP"] += 1
    m_fp = len(det_m) - m_vp
    m_fn = len(truth_m) - m_vp

    def prf(vp: int, fp: int, fn: int) -> Dict[str, Any]:
        p = vp / (vp + fp) if (vp + fp) else 0.0
        r = vp / (vp + fn) if (vp + fn) else 0.0
        return {"VP": vp, "FP": fp, "FN": fn, "P": round(p, 4), "R": round(r, 4),
                "F1": round(2 * p * r / (p + r), 4) if (p + r) else 0.0}

    overall = prf(m_vp, m_fp, m_fn)
    p, r, f1 = overall["P"], overall["R"], overall["F1"]
    # F1 de los elementos afectados: todo menos los campos que no cambiaron
    affected = Counter()
    for cat, c in by_cat.items():
        if cat != "unchanged":
            affected.update(c)
    f1_affected = prf(affected["VP"], affected["FP"], affected["FN"])

    # PCD por nivel de dificultad de la mutación
    per_level = defaultdict(Counter)
    for c, _ in vp_list:
        per_level[c.get("level", "—")]["VP"] += 1
    for c in fn_list:
        per_level[c.get("level", "—")]["FN"] += 1

    # ---------------- Exclusivos ----------------
    def ex_key(side, e, f):
        return (str(side).lower(), norm(e), norm(f))

    def own_entity(x):
        # diff.json nombra con la tabla de A los exclusivos de B dentro de una tabla
        # emparejada; la verdad usa el nombre propio de B
        e = x.get("entity_name", "")
        if str(x.get("side")).lower() == "b" and x.get("field_name") is not None:
            e = bmap.get((norm(e), ""), (e, ""))[0]
        return e

    det_x = {ex_key(x.get("side"), own_entity(x), x.get("field_name"))
             for x in diff.get("exclusives", [])
             if entity_scope_ok(own_entity(x), scope_a if str(x.get("side")).lower() == "a" else scope_b)}
    truth_x = {ex_key(x.get("side"), x.get("entity"), x.get("field"))
               for x in truth.get("exclusives", [])
               if entity_scope_ok(x.get("entity", ""), scope_a if str(x.get("side")).lower() == "a" else scope_b)}
    x_vp = len(det_x & truth_x)

    return {
        "pair_id": truth.get("pair_id"),
        "block": truth.get("block", truth.get("level")),
        "conflicts": {"VP": VP, "FP": FP, "FN": FN, "excluded": len(excluded)},
        "PCD": _ratio(VP, VP + FN),
        "PRE": _ratio(VP, VP + FP),
        "PSR": _ratio(psr_hit, psr_den),
        "matches": {"VP": m_vp, "FP": m_fp, "FN": m_fn, "P": round(p, 4), "R": round(r, 4)},
        "F1": f1,
        "F1_affected": f1_affected,
        "matches_by_category": {cat: dict(c) for cat, c in sorted(by_cat.items())},
        "conflicts_by_level": {lv: dict(c) for lv, c in sorted(per_level.items())},
        "exclusives": {"VP": x_vp, "FP": len(det_x - truth_x), "FN": len(truth_x - det_x)},
        "per_type": {t: dict(c) for t, c in sorted(per_type.items())},
        "detail": {
            "false_negatives": fn_list,
            "false_positives": fp_list,
            "psr_misses": psr_miss,
            "excluded": excluded,
        },
    }


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--truth", required=True, type=Path)
    ap.add_argument("--run", required=True, type=Path, help="directorio con diff.json y suggestions.json")
    ap.add_argument("--ir-b", type=Path, help="IR de la fuente B (audd inspect) para verificar el lado B")
    ap.add_argument("--out", type=Path, help="guardar el resultado en JSON")
    args = ap.parse_args()
    ir_b = load_json(args.ir_b) if args.ir_b else None
    res = score_pair(load_json(args.truth), args.run, ir_b)
    text = json.dumps(res, ensure_ascii=False, indent=2)
    if args.out:
        args.out.write_text(text, encoding="utf-8")
    summary = {k: res[k] for k in ("pair_id", "conflicts", "PCD", "PRE", "PSR", "F1")}
    print(json.dumps(summary, ensure_ascii=False, indent=2))
    return 0


if __name__ == "__main__":
    sys.exit(main())
