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

    # Conteos con la especificación de unificación simétrica (PROTOCOLO, sección 5):
    # las conversiones seguras y con pérdida y la nulabilidad en ambos sentidos son conflictos.

    def test_chinook_cross_engine_expected_counts(self):
        # MySQL (INT = Int32) frente a SQLite (INTEGER = Int64): conflictos de baja severidad
        r = self.results["chinook_cross_engine"]
        self.assertEqual(r.matches, 75, f"matches={r.matches}")
        self.assertEqual(r.exclusives, 0, f"exclusives={r.exclusives}")
        self.assertEqual(r.conflicts, 24, f"conflicts={r.conflicts}")

    # Conteos con emparejamiento uno a uno (exacto > normalizado > semántico > similitud).

    def test_grow_real_case_expected_counts(self):
        # Sin cambios con el emparejamiento uno a uno: los pares del caso Grow ya eran únicos.
        r = self.results["grow_travel_vs_venta"]
        self.assertEqual(r.matches, 17, f"matches={r.matches}")
        self.assertEqual(r.conflicts, 12, f"conflicts={r.conflicts}")
        self.assertEqual(r.exclusives, 176, f"exclusives={r.exclusives}")

    def test_unicenta_vs_growventa_has_matches(self):
        # PRODUCTS ya no se empareja también con Product_Product: 12 -> 11 matches y
        # 183 -> 157 exclusivos (Product_Product pasa a exclusivo de entidad y se dejan
        # de duplicar los 21 campos de PRODUCTS y 6 de Product_Product).
        r = self.results["unicenta_vs_growventa"]
        self.assertEqual(r.matches, 11, f"matches={r.matches}")
        self.assertEqual(r.exclusives, 157, f"exclusives={r.exclusives}")
        self.assertEqual(r.conflicts, 2, f"conflicts={r.conflicts}")

    def test_northwind_loads_with_quoted_identifiers(self):
        # Esta prueba fallaría antes de soportar espacios en identificadores SQLite.
        r = self.results["northwind_vs_chinook"]
        self.assertEqual(r.exit_code, 0)
        self.assertGreater(r.matches, 0)


if __name__ == "__main__":
    unittest.main()
