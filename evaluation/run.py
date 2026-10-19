#!/usr/bin/env python3
"""
run.py

Orquesta la suite de evaluación de AUDD.

Uso:
    python evaluation/run.py --all
    python evaluation/run.py --pair evaluation/generated/grow_venta_l2_s42
    python evaluation/run.py --level 2 --count 5
    python evaluation/run.py --real-pairs

Salidas:
    evaluation/results/results.csv                    métricas por par sintético
    evaluation/results/summary.md                     resumen por nivel
    evaluation/results/env.json                       entorno de ejecución
    evaluation/results/real_pairs_results.csv         métricas de pares reales
    evaluation/results/real_pairs_summary.md          resumen de pares reales
"""

import argparse
import csv
import json
import os
import shutil
import subprocess
import sys
import time
from dataclasses import dataclass, asdict
from pathlib import Path
from typing import Dict, List, Optional, Tuple

from real_pairs import REAL_PAIRS, evaluate_real_pair, write_real_results_csv, write_real_summary
from common import (
    audd_binary,
    canonical_hash,
    load_json,
    write_env,
    RESULTS_DIR,
    GENERATED_DIR,
    BASES_DIR,
    PROJECT_ROOT,
    EVAL_DIR,
)


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
        print(f"AUDD failed: {result.stderr}", file=sys.stderr)
    return result.returncode, elapsed


def load_truth(pair_dir: Path) -> Dict:
    return load_json(pair_dir / "truth.json")


def normalize_name(name: str) -> str:
    return name.lower().replace("_", "").replace(" ", "")


def match_key(entity: str, field: Optional[str] = None) -> str:
    if field:
        return f"{normalize_name(entity)}.{normalize_name(field)}"
    return normalize_name(entity)


def compute_match_f1(detected_matches: List[Dict], truth_matches: List[Dict]) -> Tuple[float, float, float]:
    truth_set = set()
    for m in truth_matches:
        a = m.get("a", "")
        b = m.get("b", "")
        if "." in a and "." in b:
            truth_set.add(match_key(a.split(".")[0], a.split(".")[1]))
        else:
            truth_set.add(match_key(a))

    detected_set = set()
    for m in detected_matches:
        # AUDD diff.json no tiene matches directos; unified_schema.json tiene origin
        pass

    # Mejor aproximación: contar coincidencias en unified_schema por entidad/campo
    # Esto se hace en evaluate_pair
    return 0.0, 0.0, 0.0


@dataclass
class PairResult:
    pair_id: str
    level: int
    seed: int
    exit_code: int
    runs: int
    time_ms: float
    matches: int
    exclusives: int
    conflicts: int
    truth_matches: int
    truth_exclusives: int
    truth_conflicts: int
    pcd: float
    pre: float
    psr: float
    f1: float
    rep: float
    rob: float


def evaluate_pair(pair_dir: Path) -> PairResult:
    truth = load_truth(pair_dir)
    source_a = f"db:sqlite://{pair_dir / 'a.sqlite'}"
    source_b = f"db:sqlite://{pair_dir / 'b.sqlite'}"

    run_outputs: List[Path] = []
    times = []
    exit_codes = []

    for i in range(3):
        out_dir = pair_dir / f"run_{i}"
        if out_dir.exists():
            shutil.rmtree(out_dir)
        out_dir.mkdir(parents=True, exist_ok=True)
        code, elapsed = run_audd_compare(source_a, source_b, out_dir)
        exit_codes.append(code)
        times.append(elapsed)
        run_outputs.append(out_dir)

    exit_code = max(exit_codes)
    time_ms = sum(times) * 1000 / len(times)

    # Cargar diff.json del primer run
    diff = load_json(run_outputs[0] / "diff.json")
    detected_matches = diff.get("matches", [])
    detected_exclusives = diff.get("exclusive_to_a", []) + diff.get("exclusive_to_b", [])
    detected_conflicts = diff.get("conflicts", [])

    # Calcular REP: hash canónico de diff.json y unified_schema.json entre runs
    files_to_check = ["diff.json", "unified_schema.json"]
    rep_hashes = []
    for fname in files_to_check:
        hashes = [canonical_hash(run / fname) for run in run_outputs]
        rep_hashes.append(all(h == hashes[0] for h in hashes))
    rep = 100.0 if all(rep_hashes) else 0.0

    # ROB: % de archivos estables entre runs
    all_files = ["diff.json", "unified_schema.json", "report.json"]
    stable = 0
    for fname in all_files:
        hashes = [canonical_hash(run / fname) for run in run_outputs]
        if all(h == hashes[0] for h in hashes):
            stable += 1
    rob = (stable / len(all_files)) * 100.0

    # PRE: reproducibilidad total (todos los outputs idénticos)
    all_outputs = ["diff.json", "unified_schema.json", "report.json", "suggestions.json"]
    pre_per_file = []
    for fname in all_outputs:
        hashes = [canonical_hash(run / fname) for run in run_outputs]
        pre_per_file.append(all(h == hashes[0] for h in hashes))
    pre = 100.0 if all(pre_per_file) else 0.0

    # PCD: conflictos detectados vs truth
    truth_conflicts = truth.get("conflicts", [])
    def norm_conflict_type(t: str) -> str:
        return t.lower().replace("_", "").replace(" ", "")

    detected_conflict_keys = set()
    for c in detected_conflicts:
        entity = c.get("entity_name", "") or c.get("entity", "")
        field = c.get("field_name") or c.get("field")
        ctype = norm_conflict_type(c.get("conflict_type", ""))
        key = (match_key(entity, field), ctype)
        detected_conflict_keys.add(key)

    truth_conflict_keys = set()
    for c in truth_conflicts:
        entity = c.get("entity", "")
        field = c.get("field")
        ctype = norm_conflict_type(c.get("type", ""))
        key = (match_key(entity, field), ctype)
        truth_conflict_keys.add(key)

    if truth_conflict_keys:
        pcd = len(detected_conflict_keys & truth_conflict_keys) / len(truth_conflict_keys) * 100.0
    else:
        pcd = 100.0 if not detected_conflicts else 0.0

    # F1 de coincidencias
    truth_match_keys = set()
    for m in truth.get("matches", []):
        a = m.get("a", "")
        if "." in a:
            truth_match_keys.add(match_key(a.split(".")[0], a.split(".")[1]))
        else:
            truth_match_keys.add(match_key(a))

    detected_match_keys = set()
    for m in detected_matches:
        # diff.json matches no tienen entidad/field explícito en todas las versiones
        # Intentar extraer de unified_schema.json
        pass

    # Aproximación simple: comparar entidades de unified_schema.json con origin Both
    unified = load_json(run_outputs[0] / "unified_schema.json")
    for entity in unified.get("entities", []):
        origin = entity.get("origin", "").upper()
        if origin == "BOTH":
            detected_match_keys.add(match_key(entity.get("entity_name", "")))
        for field in entity.get("fields", []):
            if field.get("origin", "").upper() == "BOTH":
                detected_match_keys.add(
                    match_key(entity.get("entity_name", ""), field.get("field", {}).get("field_name", ""))
                )

    tp = len(detected_match_keys & truth_match_keys)
    fp = len(detected_match_keys - truth_match_keys)
    fn = len(truth_match_keys - detected_match_keys)

    precision = tp / (tp + fp) if (tp + fp) > 0 else 0.0
    recall = tp / (tp + fn) if (tp + fn) > 0 else 0.0
    f1 = 2 * precision * recall / (precision + recall) if (precision + recall) > 0 else 0.0

    # PSR: sugerencias correctas vs conflictos detectados
    suggestions = load_json(run_outputs[0] / "suggestions.json")
    correct_suggestions = 0
    for sug in suggestions:
        # No tenemos expected directo; usamos que existe sugerencia distinta de NoSuggestion
        if sug.get("suggestion_kind") != "NoSuggestion":
            correct_suggestions += 1

    if detected_conflicts:
        psr = correct_suggestions / len(detected_conflicts) * 100.0
    else:
        psr = 100.0

    return PairResult(
        pair_id=truth.get("pair_id", pair_dir.name),
        level=truth.get("level", 0),
        seed=truth.get("seed", 0),
        exit_code=exit_code,
        runs=3,
        time_ms=round(time_ms, 2),
        matches=len(detected_matches),
        exclusives=len(detected_exclusives),
        conflicts=len(detected_conflicts),
        truth_matches=len(truth_match_keys),
        truth_exclusives=len(truth.get("exclusives", [])),
        truth_conflicts=len(truth_conflicts),
        pcd=round(pcd, 2),
        pre=round(pre, 2),
        psr=round(psr, 2),
        f1=round(f1, 4),
        rep=round(rep, 2),
        rob=round(rob, 2),
    )


def generate_corpus_level(level: int, count: int, base: Path) -> List[Path]:
    pairs = []
    for i in range(count):
        pair_id = f"{base.stem}_l{level}_s{i + 1000}"
        out_dir = GENERATED_DIR / pair_id
        cmd = [
            sys.executable,
            str(EVAL_DIR / "mutate.py"),
            "--base", str(base),
            "--pair-id", pair_id,
            "--level", str(level),
            "--seed", str(i + 1000),
            "--out", str(out_dir),
        ]
        subprocess.run(cmd, check=True)
        pairs.append(out_dir)
    return pairs


def generate_all_corpus() -> List[Path]:
    pairs: List[Path] = []
    # Nivel 1: identidad entre motores — par a=a para cada base
    for base in BASES_DIR.rglob("*.sql"):
        # Saltar archivos que solo contienen datos o que no son adecuados para mutación
        if "-data" in base.name.lower():
            continue
        pair_id = f"{base.parent.name}_{base.stem}_identity"
        out_dir = GENERATED_DIR / pair_id
        if out_dir.exists():
            shutil.rmtree(out_dir)
        # Copiar el DDL base como A y B (sin mutaciones)
        subprocess.run([
            sys.executable,
            str(EVAL_DIR / "mutate.py"),
            "--base", str(base),
            "--pair-id", pair_id,
            "--level", "1",
            "--seed", "1",
            "--out", str(out_dir),
        ], check=True, stdout=subprocess.DEVNULL)
        # Forzar A == B reemplazando b.sqlite por a.sqlite
        shutil.copy(out_dir / "a.sqlite", out_dir / "b.sqlite")
        pairs.append(out_dir)

    # Nivel 2: 10 pares sintéticos por base
    for base in BASES_DIR.rglob("*.sql"):
        if "-data" in base.name.lower():
            continue
        pairs.extend(generate_corpus_level(2, 10, base))

    # Nivel 3: versiones n contra n+1 de Grow Venta
    base = BASES_DIR / "growventa" / "grow_venta.sql"
    pairs.extend(generate_corpus_level(3, 2, base))

    # Nivel 4: caso de aplicación con truth manual (placeholder)
    base = BASES_DIR / "grow_venta.sql"
    pairs.extend(generate_corpus_level(4, 1, base))

    return pairs


def write_results_csv(results: List[PairResult]) -> None:
    RESULTS_DIR.mkdir(parents=True, exist_ok=True)
    path = RESULTS_DIR / "results.csv"
    with open(path, "w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=[
            "pair_id", "level", "seed", "exit_code", "runs", "time_ms",
            "matches", "exclusives", "conflicts",
            "truth_matches", "truth_exclusives", "truth_conflicts",
            "pcd", "pre", "psr", "f1", "rep", "rob",
        ])
        writer.writeheader()
        for r in results:
            writer.writerow(asdict(r))


def write_summary(results: List[PairResult]) -> None:
    path = RESULTS_DIR / "summary.md"
    by_level: Dict[int, List[PairResult]] = {}
    for r in results:
        by_level.setdefault(r.level, []).append(r)

    lines = [
        "# Resumen de evaluación AUDD",
        "",
        f"**Fecha:** {time.strftime('%Y-%m-%d %H:%M:%S')}",
        f"**Pares evaluados:** {len(results)}",
        "",
        "## Metas",
        "",
        "| Indicador | Meta |",
        "|---|---|",
        "| PCD | ≥ 70 % |",
        "| PRE | ≥ 90 % |",
        "| PSR | ≥ 50 % |",
        "| REP | 100 % |",
        "| ROB | ≥ 90 % |",
        "",
        "## Resultados por nivel",
        "",
    ]

    for level in sorted(by_level):
        rs = by_level[level]
        avg = lambda attr: round(sum(getattr(r, attr) for r in rs) / len(rs), 2)
        lines.append(f"### Nivel {level}")
        lines.append("")
        lines.append(f"- Pares: {len(rs)}")
        lines.append(f"- PCD: {avg('pcd')} %")
        lines.append(f"- PRE: {avg('pre')} %")
        lines.append(f"- PSR: {avg('psr')} %")
        lines.append(f"- F1: {avg('f1')}")
        lines.append(f"- REP: {avg('rep')} %")
        lines.append(f"- ROB: {avg('rob')} %")
        lines.append("")

    lines.append("## Resultados globales")
    lines.append("")
    avg = lambda attr: round(sum(getattr(r, attr) for r in results) / len(results), 2)
    lines.append(f"- PCD: {avg('pcd')} %")
    lines.append(f"- PRE: {avg('pre')} %")
    lines.append(f"- PSR: {avg('psr')} %")
    lines.append(f"- F1: {avg('f1')}")
    lines.append(f"- REP: {avg('rep')} %")
    lines.append(f"- ROB: {avg('rob')} %")
    lines.append("")

    path.write_text("\n".join(lines), encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description="Suite de evaluación AUDD")
    parser.add_argument("--all", action="store_true", help="Generar corpus completo y evaluar")
    parser.add_argument("--pair", type=Path, help="Evaluar un par ya generado")
    parser.add_argument("--level", type=int, default=2, choices=[1, 2, 3, 4])
    parser.add_argument("--count", type=int, default=5, help="Número de pares a generar")
    parser.add_argument("--base", type=Path, default=BASES_DIR / "growventa" / "grow_venta.sql")
    parser.add_argument("--real-pairs", action="store_true",
                        help="Evaluar únicamente los pares de bases de datos reales")
    args = parser.parse_args()

    if not shutil.which("cargo") and not audd_binary()[0].endswith("audd"):
        print("Error: no se encontró cargo ni el binario audd.", file=sys.stderr)
        return 1

    RESULTS_DIR.mkdir(parents=True, exist_ok=True)
    write_env()

    if args.real_pairs:
        real_results = [evaluate_real_pair(p) for p in REAL_PAIRS]
        csv_path = write_real_results_csv(real_results)
        md_path = write_real_summary(real_results)
        print(f"Real-pair results written to {csv_path} and {md_path}")
        return 0

    if args.pair:
        results = [evaluate_pair(args.pair)]
    elif args.all:
        pairs = generate_all_corpus()
        results = [evaluate_pair(p) for p in pairs]
    else:
        pairs = generate_corpus_level(args.level, args.count, args.base)
        results = [evaluate_pair(p) for p in pairs]

    write_results_csv(results)
    write_summary(results)
    print(f"Results written to {RESULTS_DIR}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
