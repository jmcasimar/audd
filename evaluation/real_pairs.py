#!/usr/bin/env python3
"""
real_pairs.py

Define y evalúa pares de bases de datos reales (sin mutaciones sintéticas).
Útil para validar que AUDD puede comparar el mismo esquema bajo distintos
motores y para probar casos de aplicación reales.

Uso directo:
    python evaluation/real_pairs.py
    python evaluation/real_pairs.py --pair chinook_cross_engine
"""

import argparse
import csv
import json
import shutil
import subprocess
import sys
import time
from dataclasses import dataclass, asdict
from pathlib import Path
from typing import Dict, List, Tuple

from common import audd_binary, canonical_hash, load_json, RESULTS_DIR, PROJECT_ROOT, EVAL_DIR


REAL_PAIRS: List[Dict[str, str]] = [
    {
        "pair_id": "chinook_cross_engine",
        "description": "Chinook en MySQL DDL vs Chinook en SQLite",
        "source_a": "evaluation/bases/chinohook/Chinook_MySql.sql",
        "source_b": "db:sqlite://evaluation/bases/chinohook/Chinook_Sqlite.sqlite",
    },
    {
        "pair_id": "grow_travel_vs_venta",
        "description": "Caso real Grow Travel (MySQL DDL) vs Grow Venta (SQLite)",
        "source_a": "evaluation/bases/growtravel/growvtravel.sql",
        "source_b": "db:sqlite://evaluation/bases/growventa/growventa.db",
    },
    {
        "pair_id": "unicenta_vs_growventa",
        "description": "UniCenta OPOS (MySQL DDL) vs Grow Venta (SQLite)",
        "source_a": "evaluation/bases/unicenta-opos/MySQL-create.sql",
        "source_b": "db:sqlite://evaluation/bases/growventa/growventa.db",
    },
    {
        "pair_id": "northwind_vs_chinook",
        "description": "Northwind (SQLite) vs Chinook (SQLite)",
        "source_a": "db:sqlite://evaluation/bases/northwind/northwind.db",
        "source_b": "db:sqlite://evaluation/bases/chinohook/Chinook_Sqlite.sqlite",
    },
    {
        "pair_id": "sakila_vs_chinook_mysql",
        "description": "Sakila (MySQL DDL) vs Chinook (MySQL DDL)",
        "source_a": "evaluation/bases/sakila-db/sakila-schema.sql",
        "source_b": "evaluation/bases/chinohook/Chinook_MySql.sql",
    },
]


@dataclass
class RealPairResult:
    pair_id: str
    description: str
    exit_code: int
    runs: int
    time_ms: float
    matches: int
    exclusives: int
    conflicts: int
    suggestions: int
    pre: float
    rep: float
    rob: float


def run_audd_compare(source_a: str, source_b: str, out_dir: Path) -> Tuple[int, float]:
    cmd = audd_binary() + [
        "compare",
        "--source-a", source_a,
        "--source-b", source_b,
        "--out", str(out_dir),
        "--deterministic",
    ]
    start = time.time()
    result = subprocess.run(
        cmd, cwd=PROJECT_ROOT, capture_output=True, text=True, encoding="utf-8", errors="replace"
    )
    elapsed = time.time() - start
    if result.returncode != 0:
        print(f"AUDD failed for {source_a} vs {source_b}:\n{result.stderr}", file=sys.stderr)
    return result.returncode, elapsed


def evaluate_real_pair(pair: Dict[str, str]) -> RealPairResult:
    pair_id = pair["pair_id"]
    description = pair["description"]
    source_a = pair["source_a"]
    source_b = pair["source_b"]

    out_base = RESULTS_DIR / "real_pairs" / pair_id
    if out_base.exists():
        shutil.rmtree(out_base)
    out_base.mkdir(parents=True, exist_ok=True)

    run_outputs: List[Path] = []
    times = []
    exit_codes = []

    for i in range(3):
        out_dir = out_base / f"run_{i}"
        out_dir.mkdir(parents=True, exist_ok=True)
        code, elapsed = run_audd_compare(source_a, source_b, out_dir)
        exit_codes.append(code)
        times.append(elapsed)
        run_outputs.append(out_dir)

    exit_code = max(exit_codes)
    time_ms = sum(times) * 1000 / len(times)

    diff = load_json(run_outputs[0] / "diff.json")
    summary = diff.get("summary", {})
    matches = summary.get("total_matches", len(diff.get("matches", [])))
    exclusives = summary.get("total_exclusives", len(diff.get("exclusives", [])))
    conflicts = summary.get("total_conflicts", len(diff.get("conflicts", [])))
    suggestions = load_json(run_outputs[0] / "suggestions.json")
    suggestion_count = sum(1 for s in suggestions if s.get("suggestion_kind") != "NoSuggestion")

    # PRE/REP/ROB: reproducibilidad entre runs
    all_outputs = ["diff.json", "unified_schema.json", "report.json", "suggestions.json"]
    pre_per_file = []
    rep_per_file = []
    for fname in all_outputs:
        hashes = [canonical_hash(run / fname) for run in run_outputs]
        all_equal = all(h == hashes[0] for h in hashes)
        pre_per_file.append(all_equal)
        # REP solo diff y unified_schema
        if fname in {"diff.json", "unified_schema.json"}:
            rep_per_file.append(all_equal)

    pre = 100.0 if all(pre_per_file) else 0.0
    rep = 100.0 if all(rep_per_file) else 0.0

    # ROB: porcentaje de archivos estables
    stable = sum(1 for eq in pre_per_file if eq)
    rob = (stable / len(all_outputs)) * 100.0

    return RealPairResult(
        pair_id=pair_id,
        description=description,
        exit_code=exit_code,
        runs=3,
        time_ms=round(time_ms, 2),
        matches=matches,
        exclusives=exclusives,
        conflicts=conflicts,
        suggestions=suggestion_count,
        pre=round(pre, 2),
        rep=round(rep, 2),
        rob=round(rob, 2),
    )


def write_real_results_csv(results: List[RealPairResult]) -> Path:
    RESULTS_DIR.mkdir(parents=True, exist_ok=True)
    path = RESULTS_DIR / "real_pairs_results.csv"
    with open(path, "w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=[
            "pair_id", "description", "exit_code", "runs", "time_ms",
            "matches", "exclusives", "conflicts", "suggestions",
            "pre", "rep", "rob",
        ])
        writer.writeheader()
        for r in results:
            writer.writerow(asdict(r))
    return path


def write_real_summary(results: List[RealPairResult]) -> Path:
    path = RESULTS_DIR / "real_pairs_summary.md"
    lines = [
        "# Resumen de pares reales",
        "",
        f"**Fecha:** {time.strftime('%Y-%m-%d %H:%M:%S')}",
        f"**Pares evaluados:** {len(results)}",
        "",
        "| Par | Descripción | Exit | Coincidencias | Exclusivos | Conflictos | Sugerencias | PRE | REP | ROB |",
        "|---|---|---:|---:|---:|---:|---:|---:|---:|---:|",
    ]
    for r in results:
        lines.append(
            f"| {r.pair_id} | {r.description} | {r.exit_code} | "
            f"{r.matches} | {r.exclusives} | {r.conflicts} | {r.suggestions} | "
            f"{r.pre}% | {r.rep}% | {r.rob}% |"
        )
    avg = lambda attr: round(sum(getattr(r, attr) for r in results) / len(results), 2) if results else 0.0
    lines.extend([
        "",
        "## Promedios",
        "",
        f"- PRE: {avg('pre')} %",
        f"- REP: {avg('rep')} %",
        f"- ROB: {avg('rob')} %",
        f"- Tiempo medio: {avg('time_ms')} ms",
    ])
    path.write_text("\n".join(lines), encoding="utf-8")
    return path


def main() -> int:
    parser = argparse.ArgumentParser(description="Evalúa pares reales de AUDD")
    parser.add_argument("--pair", help="ID del par a evaluar (por defecto todos)")
    args = parser.parse_args()

    pairs = REAL_PAIRS
    if args.pair:
        pairs = [p for p in pairs if p["pair_id"] == args.pair]
        if not pairs:
            print(f"Par desconocido: {args.pair}", file=sys.stderr)
            print(f"Disponibles: {', '.join(p['pair_id'] for p in REAL_PAIRS)}", file=sys.stderr)
            return 1

    results = []
    for pair in pairs:
        print(f"Evaluando {pair['pair_id']}...")
        results.append(evaluate_real_pair(pair))

    csv_path = write_real_results_csv(results)
    md_path = write_real_summary(results)
    print(f"Resultados escritos en {csv_path} y {md_path}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
