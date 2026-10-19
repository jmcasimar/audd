#!/usr/bin/env python3
"""
requirements_coverage.py

Calcula el indicador PRM: porcentaje de requerimientos funcionales y no funcionales
con pruebas de aceptación (acceptance_<id>_*) APROBADAS. Un requerimiento cuenta como
cubierto solo si tiene al menos una prueba y todas sus pruebas pasan.

Uso:
    python evaluation/requirements_coverage.py              # ejecuta cargo test acceptance
    python evaluation/requirements_coverage.py --no-run     # solo lista las pruebas (sin PRM)

Salida:
    evaluation/results/requirements_coverage.json
"""

import argparse
import json
import re
import subprocess
import sys
from datetime import datetime
from pathlib import Path

EVAL_DIR = Path(__file__).parent.resolve()
PROJECT_ROOT = EVAL_DIR.parent
RESULTS_DIR = EVAL_DIR / "results"

REQUIREMENTS = {
    "functional": ["RF1", "RF2", "RF3", "RF4", "RF5", "RF6"],
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


def run_acceptance_tests() -> dict:
    """Ejecuta `cargo test acceptance` y devuelve {nombre_de_prueba: aprobada}."""
    print("Running cargo test acceptance...")
    result = subprocess.run(
        ["cargo", "test", "acceptance", "--workspace", "--no-fail-fast"],
        cwd=PROJECT_ROOT,
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
    )
    outcomes = {}
    for m in re.finditer(r"^test (\S+) \.\.\. (ok|FAILED|ignored)", result.stdout, re.MULTILINE):
        name = m.group(1).split("::")[-1]
        outcomes[name] = m.group(2) == "ok"
    if result.returncode != 0:
        print(result.stderr[-2000:], file=sys.stderr)
    return outcomes


def main() -> int:
    parser = argparse.ArgumentParser(description="Calcula PRM: requerimientos con pruebas de aceptación aprobadas.")
    parser.add_argument("--no-run", action="store_true", help="No ejecutar las pruebas; solo listarlas")
    parser.add_argument("--run-tests", action="store_true", help=argparse.SUPPRESS)  # compatibilidad
    args = parser.parse_args()

    coverage = find_acceptance_tests()
    outcomes = None if args.no_run else run_acceptance_tests()

    def covered(req: str) -> bool:
        tests = coverage[req]
        if not tests or outcomes is None:
            return False
        return all(outcomes.get(t.split("::")[-1], False) for t in tests)

    status = {
        req: {
            "tests": [{"test": t, "passed": None if outcomes is None else outcomes.get(t.split("::")[-1], False)}
                      for t in coverage[req]],
            "covered": covered(req),
        }
        for req in coverage
    }

    def summary(kind: str) -> dict:
        reqs = REQUIREMENTS[kind]
        n = sum(1 for r in reqs if status[r]["covered"])
        return {"covered": n, "total": len(reqs), "percentage": round(100.0 * n / len(reqs), 2)}

    total_n = sum(1 for r in status if status[r]["covered"])
    report = {
        "date": datetime.now().isoformat(timespec="seconds"),
        "tests_run": outcomes is not None,
        "coverage": status,
        "summary": {
            "functional": summary("functional"),
            "non_functional": summary("non_functional"),
            "total": {"covered": total_n, "total": len(status),
                      "percentage": round(100.0 * total_n / len(status), 2) if outcomes is not None else None},
        },
    }

    RESULTS_DIR.mkdir(parents=True, exist_ok=True)
    path = RESULTS_DIR / "requirements_coverage.json"
    path.write_text(json.dumps(report, indent=2, ensure_ascii=False), encoding="utf-8")

    print(f"Requirements coverage written to {path}")
    for req, st in status.items():
        mark = "OK " if st["covered"] else "-- "
        print(f"  {mark}{req}: {len(st['tests'])} prueba(s)")
    if outcomes is None:
        print("  PRM no calculado (--no-run)")
    else:
        print(f"  Total PRM: {report['summary']['total']['percentage']:.1f}%")
    return 0


if __name__ == "__main__":
    sys.exit(main())
