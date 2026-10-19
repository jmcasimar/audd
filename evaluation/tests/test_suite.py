#!/usr/bin/env python3
"""
Tests de la propia suite de evaluación (E07).

Ejecutar:
    python evaluation/tests/test_suite.py
"""

import csv
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

EVAL_DIR = Path(__file__).parent.parent.resolve()
PROJECT_ROOT = EVAL_DIR.parent
MUTATE_PY = EVAL_DIR / "mutate.py"
RUN_PY = EVAL_DIR / "run.py"
BASES_DIR = EVAL_DIR / "bases"


def run_mutate(base: Path, pair_id: str, level: int, seed: int, out: Path) -> None:
    subprocess.run(
        [sys.executable, str(MUTATE_PY), "--base", str(base), "--pair-id", pair_id,
         "--level", str(level), "--seed", str(seed), "--out", str(out)],
        check=True,
    )


def run_eval_pair(pair_dir: Path) -> Path:
    subprocess.run(
        [sys.executable, str(RUN_PY), "--pair", str(pair_dir)],
        check=True,
    )
    return EVAL_DIR / "results" / "results.csv"


class SuiteTests(unittest.TestCase):
    def test_trivial_pair_pre_100_and_f1_1(self):
        """Un par trivial (A == B) debe tener PRE = 100 % y F1 = 1."""
        with tempfile.TemporaryDirectory() as tmp:
            tmp_path = Path(tmp)
            pair_dir = tmp_path / "identity"
            base = BASES_DIR / "growventa" / "grow_venta.sql"
            run_mutate(base, "identity_test", 1, 1, pair_dir)
            # Forzar A == B
            shutil.copy(pair_dir / "a.sqlite", pair_dir / "b.sqlite")
            results_csv = run_eval_pair(pair_dir)
            with open(results_csv, newline="", encoding="utf-8") as f:
                reader = csv.DictReader(f)
                row = next(reader)
            self.assertEqual(row["pre"], "100.0", f"Expected PRE=100, got {row['pre']}")
            self.assertEqual(row["f1"], "1.0", f"Expected F1=1, got {row['f1']}")

    def test_known_conflict_pair_pcd_positive(self):
        """Un par con conflictos conocidos debe tener PCD > 0."""
        with tempfile.TemporaryDirectory() as tmp:
            tmp_path = Path(tmp)
            pair_dir = tmp_path / "known_conflict"
            base = BASES_DIR / "growventa" / "grow_venta.sql"
            # La semilla 99 produce al menos un conflicto detectable desde SQLite
            run_mutate(base, "known_conflict_test", 2, 99, pair_dir)
            results_csv = run_eval_pair(pair_dir)
            with open(results_csv, newline="", encoding="utf-8") as f:
                reader = csv.DictReader(f)
                row = next(reader)
            self.assertGreater(int(row["truth_conflicts"]), 0,
                               "Expected the seed to produce at least one truth conflict")
            self.assertGreater(float(row["pcd"]), 0, f"Expected PCD > 0, got {row['pcd']}")


if __name__ == "__main__":
    unittest.main()
