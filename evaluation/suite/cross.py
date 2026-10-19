#!/usr/bin/env python3
"""Matriz cruzada automatizada: todas las bases contra todas, en ambos sentidos.

Para N bases ejecuta:
    - N comparaciones de identidad (X contra X): no debe haber conflictos ni exclusivos.
    - N x (N-1) comparaciones dirigidas (X contra Y y Y contra X).
Cada comparación se repite `runs` veces (por defecto 3) con --deterministic.

Indicadores que produce (no requieren verdad de referencia):
    REP  % de comparaciones cuyas salidas son idénticas en las repeticiones
    ROB  % de comparaciones que terminan sin error y con JSON válidos
    SYM  % de pares no ordenados cuyo resultado es simétrico al invertir A y B
    FCI  conflictos falsos en identidad (debe ser 0)
    Tiempo mediano de ejecución por comparación

Uso:
    python evaluation/suite/cross.py                # todas las bases de config/suite.toml
    python evaluation/suite/cross.py --only chinook northwind
    python evaluation/suite/cross.py --runs 3 --out evaluation/results/<id>
"""

from __future__ import annotations

import argparse
import csv
import itertools
import statistics
import sys
from collections import Counter
from datetime import datetime
from pathlib import Path
from typing import Any, Dict, List

sys.path.insert(0, str(Path(__file__).parent))
from common import (EVAL_DIR, OUTPUT_FILES, PROJECT_ROOT, audd_cmd, canonical_json_hash,  # noqa: E402
                    compare_args, environment, git, load_config, load_json, norm, norm_type,
                    run_compare, run_inspect, sha256_dir, sha256_file, source_of, write_json)


def default_run_dir() -> Path:
    commit = (git("rev-parse", "--short", "HEAD") or "nocommit")
    return EVAL_DIR / "results" / f"{datetime.now():%Y%m%d-%H%M}_{commit}"


def snapshot_bases(cfg, bases, out: Path) -> Dict[str, Any]:
    """IR de cada base (audd inspect) + hash. Detecta si una base cambió entre corridas."""
    snaps = {}
    lock_path = EVAL_DIR / "config" / "snapshots.lock.json"
    lock = load_json(lock_path) if lock_path.exists() else {}
    for b in bases:
        f = out / "snapshots" / f"{b['id']}.ir.json"
        code = run_inspect(cfg, source_of(b), f)
        if code != 0:
            snaps[b["id"]] = {"error": f"inspect exit {code}"}
            continue
        h = canonical_json_hash(f)
        ir = load_json(f)
        snaps[b["id"]] = {
            "ir_sha256": h,
            "entities": len(ir.get("entities", [])),
            "fields": sum(len(e.get("fields", [])) for e in ir.get("entities", [])),
            "changed_vs_lock": bool(lock.get(b["id"])) and lock[b["id"]] != h,
        }
        if snaps[b["id"]]["changed_vs_lock"]:
            print(f"AVISO: la base {b['id']} cambió respecto de snapshots.lock.json", file=sys.stderr)
    return snaps


def summarize_runs(runs: List[Dict[str, Any]], dirs: List[Path]) -> Dict[str, Any]:
    ok = all(r["valid_outputs"] for r in runs)
    rep = None
    if ok:
        rep = all(len({canonical_json_hash(d / f) for d in dirs}) == 1 for f in OUTPUT_FILES)
    out = {"robust": ok, "reproducible": rep,
           "median_s": round(statistics.median(r["seconds"] for r in runs), 4)}
    if ok:
        s = load_json(dirs[0] / "diff.json").get("summary", {})
        out.update({k: s.get(k) for k in ("total_matches", "total_exclusives", "total_conflicts",
                                          "exclusives_a", "exclusives_b")})
    return out


def symmetry(dir_xy: Path, dir_yx: Path, ir_x: Dict[str, Any], ir_y: Dict[str, Any]) -> Dict[str, Any]:
    """Compara X->Y con Y->X. Las coincidencias y exclusivos deben invertirse
    exactamente; los conflictos se comparan por tipo."""
    d1, d2 = load_json(dir_xy / "diff.json"), load_json(dir_yx / "diff.json")

    def excl(d, side, ir_b):
        # En diff.json, un exclusivo de B dentro de una tabla emparejada lleva el nombre
        # de la tabla de A: se traduce al nombre propio de B con el índice de la coincidencia.
        ents_b = ir_b.get("entities", [])
        a_to_b = {m["entity_name"]: ents_b[m["index_b"]]["entity_name"]
                  for m in d.get("matches", [])
                  if m.get("field_name") is None and 0 <= m.get("index_b", -1) < len(ents_b)}
        out = set()
        for x in d.get("exclusives", []):
            if str(x.get("side")).lower() != side:
                continue
            entity = x.get("entity_name")
            if side == "b" and x.get("field_name") is not None:
                entity = a_to_b.get(entity, entity)
            out.add((norm(entity), norm(x.get("field_name"))))
        return out

    def ctypes(d):
        return Counter(norm_type(c.get("conflict_type")) for c in d.get("conflicts", []))

    m_ok = len(d1.get("matches", [])) == len(d2.get("matches", []))
    x_ok = excl(d1, "a", ir_y) == excl(d2, "b", ir_x) and excl(d1, "b", ir_y) == excl(d2, "a", ir_x)
    c1, c2 = ctypes(d1), ctypes(d2)
    c_ok = c1 == c2
    return {
        "matches_sym": m_ok, "exclusives_sym": x_ok, "conflicts_sym": c_ok,
        "symmetric": m_ok and x_ok and c_ok,
        "conflicts_xy": dict(c1), "conflicts_yx": dict(c2),
        "only_in_xy": dict(c1 - c2), "only_in_yx": dict(c2 - c1),
    }


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--only", nargs="*", help="limitar a estas bases (ids)")
    ap.add_argument("--runs", type=int)
    ap.add_argument("--out", type=Path)
    args = ap.parse_args()

    cfg = load_config()
    bases = [b for b in cfg["base"] if not args.only or b["id"] in args.only]
    runs = args.runs or int(cfg["audd"].get("runs", 3))
    out = args.out or default_run_dir()
    out.mkdir(parents=True, exist_ok=True)

    manifest = {
        "created": datetime.now().isoformat(timespec="seconds"),
        "env": environment(),
        "audd_binary_sha256": sha256_file(Path(audd_cmd(cfg)[0])),
        "compare_args": compare_args(cfg),
        "catalog_sha256": sha256_dir(PROJECT_ROOT / cfg["audd"]["catalog"]) if cfg["audd"].get("catalog") else None,
        "runs_per_comparison": runs,
        "bases": [b["id"] for b in bases],
    }
    manifest["snapshots"] = snapshot_bases(cfg, bases, out)
    write_json(out / "manifest.json", manifest)

    rows, results = [], {}
    pairs = [(b, b) for b in bases] + [(x, y) for x, y in itertools.permutations(bases, 2)]
    for x, y in pairs:
        cid = f"{x['id']}__{y['id']}"
        dirs, rr = [], []
        for i in range(runs):
            d = out / "cross" / cid / f"run_{i}"
            rr.append(run_compare(cfg, source_of(x), source_of(y), d))
            dirs.append(d)
        s = summarize_runs(rr, dirs)
        s.update({"comparison": cid, "a": x["id"], "b": y["id"], "identity": x["id"] == y["id"]})
        results[cid] = s
        rows.append(s)
        print(f"{cid:40s} robust={s['robust']} rep={s['reproducible']} "
              f"m={s.get('total_matches')} x={s.get('total_exclusives')} c={s.get('total_conflicts')}")

    sym_rows = []
    for x, y in itertools.combinations(bases, 2):
        a, b = f"{x['id']}__{y['id']}", f"{y['id']}__{x['id']}"
        ir_x = out / "snapshots" / f"{x['id']}.ir.json"
        ir_y = out / "snapshots" / f"{y['id']}.ir.json"
        if results[a]["robust"] and results[b]["robust"] and ir_x.exists() and ir_y.exists():
            s = symmetry(out / "cross" / a / "run_0", out / "cross" / b / "run_0",
                         load_json(ir_x), load_json(ir_y))
            s["pair"] = f"{x['id']}<->{y['id']}"
            sym_rows.append(s)

    with open(out / "cross.csv", "w", newline="", encoding="utf-8") as f:
        cols = ["comparison", "a", "b", "identity", "robust", "reproducible", "median_s",
                "total_matches", "total_exclusives", "exclusives_a", "exclusives_b", "total_conflicts"]
        w = csv.DictWriter(f, fieldnames=cols, extrasaction="ignore")
        w.writeheader()
        w.writerows(rows)
    write_json(out / "symmetry.json", sym_rows)

    n = len(rows)
    ident = [r for r in rows if r["identity"] and r["robust"]]
    indicators = {
        "comparisons": n,
        "ROB": round(100 * sum(r["robust"] for r in rows) / n, 2) if n else None,
        "REP": round(100 * sum(bool(r["reproducible"]) for r in rows) / n, 2) if n else None,
        "SYM": round(100 * sum(s["symmetric"] for s in sym_rows) / len(sym_rows), 2) if sym_rows else None,
        "SYM_matches": round(100 * sum(s["matches_sym"] for s in sym_rows) / len(sym_rows), 2) if sym_rows else None,
        "SYM_exclusives": round(100 * sum(s["exclusives_sym"] for s in sym_rows) / len(sym_rows), 2) if sym_rows else None,
        "SYM_conflicts": round(100 * sum(s["conflicts_sym"] for s in sym_rows) / len(sym_rows), 2) if sym_rows else None,
        "FCI_false_conflicts_identity": sum(r.get("total_conflicts") or 0 for r in ident),
        "FCI_false_exclusives_identity": sum(r.get("total_exclusives") or 0 for r in ident),
        "median_seconds": round(statistics.median(r["median_s"] for r in rows), 4) if rows else None,
    }
    write_json(out / "cross_indicators.json", indicators)
    print(indicators)
    print(f"Resultados en {out.relative_to(PROJECT_ROOT) if out.is_relative_to(PROJECT_ROOT) else out}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
