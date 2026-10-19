#!/usr/bin/env python3
"""
Tests de pares reales (E07 extensión).

Verifica que AUDD puede cargar y comparar bases de datos reales depositadas
en evaluation/bases/ 

Ejecutar:
    python evaluation/tests/test_real_pairs.py
"""

import sys
import unittest
from pathlib import Path

EVAL_DIR = Path(__file__).parent.parent.resolve()
sys.path.insert(0, str(EVAL_DIR))

from real_pairs import REAL_PAIRS, evaluate_real_pair


class RealPairTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.results = {p["pair_id"]: evaluate_real_pair(p) for p in REAL_PAIRS}

    def test_all_pairs_exit_success(self):
        for pair_id, result in self.results.items():
            self.assertEqual(
                result.exit_code, 0,
                f"{pair_id}: AUDD terminó con error (exit_code={result.exit_code})"
            )

    def test_reproducibility_metrics_perfect(self):
        for pair_id, result in self.results.items():
            self.assertEqual(
                result.pre, 100.0,
                f"{pair_id}: PRE no es 100% ({result.pre})"
            )
            self.assertEqual(
                result.rep, 100.0,
                f"{pair_id}: REP no es 100% ({result.rep})"
            )
            self.assertEqual(
                result.rob, 100.0,
                f"{pair_id}: ROB no es 100% ({result.rob})"
            )

    def test_chinook_cross_engine_expected_counts(self):
        r = self.results["chinook_cross_engine"]
        self.assertEqual(r.matches, 71, f"matches={r.matches}")
        self.assertEqual(r.exclusives, 2, f"exclusives={r.exclusives}")
        self.assertEqual(r.conflicts, 0, f"conflicts={r.conflicts}")

    def test_grow_real_case_expected_counts(self):
        r = self.results["grow_travel_vs_venta"]
        self.assertEqual(r.matches, 17, f"matches={r.matches}")
        self.assertEqual(r.conflicts, 4, f"conflicts={r.conflicts}")
        self.assertGreater(r.exclusives, 0, f"exclusives={r.exclusives}")

    def test_unicenta_vs_growventa_has_matches(self):
        r = self.results["unicenta_vs_growventa"]
        self.assertGreater(r.matches, 0, f"matches={r.matches}")
        self.assertEqual(r.conflicts, 0, f"conflicts={r.conflicts}")

    def test_northwind_loads_with_quoted_identifiers(self):
        # Esta prueba fallaría antes de soportar espacios en identificadores SQLite.
        r = self.results["northwind_vs_chinook"]
        self.assertEqual(r.exit_code, 0)
        self.assertGreater(r.matches, 0)


if __name__ == "__main__":
    unittest.main()
