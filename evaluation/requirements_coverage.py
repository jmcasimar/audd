#!/usr/bin/env python3
"""
requirements_coverage.py

Calcula el indicador PRM: porcentaje de requerimientos funcionales y no funcionales
cubiertos por pruebas de aceptación con nombre acceptance_<id>_*.

Uso:
    python evaluation/requirements_coverage.py
    python evaluation/requirements_coverage.py --run-tests

Salida:
    evaluation/results/requirements_coverage.json
"""

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path

EVAL_DIR = Path(__file__).parent.resolve()
PROJECT_ROOT = EVAL_DIR.parent
RESULTS_DIR = EVAL_DIR / "results"

REQUIREMENTS = {
    "functional": ["RF1", "RF2", "RF3", "RF4", "RF5"],
    "non_functional": ["RNF1", "RNF2", "RNF3", "RNF4"],
}


def find_acceptance_tests() -> dict:
    """Escanea crates/**/tests/acceptance_*.rs y extrae funciones de prueba."""
    coverage = {req: [] for req in REQUIREMENTS["functional"] + REQUIREMENTS["non_functional"]}
    pattern = re.compile(r"fn\s+(acceptance_([a-zA-Z0-9]+)_\w+)")

    test_files = list(PROJECT_ROOT.rglob("tests/acceptance_*.rs"))
    for path in test_files:
        text = path.read_text(encoding="utf-8")
        for match in pattern.finditer(text):
            test_name = match.group(1)
            req_id = match.group(2).upper()
            if req_id in coverage:
                coverage[req_id].append(f"{path.relative_to(PROJECT_ROOT)}::{test_name}")

    return coverage


def run_acceptance_tests() -> bool:
    print("Running cargo test acceptance...")
    result = subprocess.run(
        ["cargo", "test", "acceptance", "--workspace"],
        cwd=PROJECT_ROOT,
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
    )
    print(result.stdout)
    if result.returncode != 0:
        print(result.stderr, file=sys.stderr)
    return result.returncode == 0


def main() -> int:
    parser = argparse.ArgumentParser(description="Calcula cobertura de requerimientos por pruebas acceptance.")
    parser.add_argument("--run-tests", action="store_true", help="Ejecuta cargo test acceptance antes de reportar")
    args = parser.parse_args()

    if args.run_tests:
        if not run_acceptance_tests():
            print("acceptance tests failed", file=sys.stderr)

    coverage = find_acceptance_tests()

    total_functional = len(REQUIREMENTS["functional"])
    covered_functional = sum(1 for req in REQUIREMENTS["functional"] if coverage[req])
    total_non_functional = len(REQUIREMENTS["non_functional"])
    covered_non_functional = sum(1 for req in REQUIREMENTS["non_functional"] if coverage[req])

    prm_functional = covered_functional / total_functional * 100.0
    prm_non_functional = covered_non_functional / total_non_functional * 100.0
    prm_total = (covered_functional + covered_non_functional) / (total_functional + total_non_functional) * 100.0

    report = {
        "date": subprocess.run(["date", "/t"], capture_output=True, text=True, shell=False).stdout.strip()
        if sys.platform == "win32"
        else subprocess.run(["date", "-Iseconds"], capture_output=True, text=True).stdout.strip(),
        "coverage": coverage,
        "summary": {
            "functional": {"covered": covered_functional, "total": total_functional, "percentage": round(prm_functional, 2)},
            "non_functional": {"covered": covered_non_functional, "total": total_non_functional, "percentage": round(prm_non_functional, 2)},
            "total": {"covered": covered_functional + covered_non_functional, "total": total_functional + total_non_functional, "percentage": round(prm_total, 2)},
        },
    }

    RESULTS_DIR.mkdir(parents=True, exist_ok=True)
    path = RESULTS_DIR / "requirements_coverage.json"
    path.write_text(json.dumps(report, indent=2), encoding="utf-8")

    print(f"Requirements coverage written to {path}")
    print(f"  Functional: {covered_functional}/{total_functional} ({prm_functional:.1f}%)")
    print(f"  Non-functional: {covered_non_functional}/{total_non_functional} ({prm_non_functional:.1f}%)")
    print(f"  Total PRM: {prm_total:.1f}%")
    return 0


if __name__ == "__main__":
    sys.exit(main())
