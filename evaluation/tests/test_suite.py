#!/usr/bin/env python3
"""
Tests de la suite de evaluación (E07), sin servidor MySQL (backend `ddl`).

Requieren el binario de release: cargo build --release -p audd-cli

Ejecutar:
    python evaluation/tests/test_suite.py
"""

import json
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

EVAL_DIR = Path(__file__).parent.parent.resolve()
PROJECT_ROOT = EVAL_DIR.parent
MUTATE_PY = EVAL_DIR / "mutate.py"
MUTATIONS_PY = EVAL_DIR / "suite" / "mutations.py"


def _import_validate_truth():
    """La suite tiene su propio `common`; evaluation/common.py (suite anterior) usa el mismo nombre."""
    previous = sys.modules.pop("common", None)
    sys.path.insert(0, str(EVAL_DIR / "suite"))
    try:
        import validate_truth
        return validate_truth.check
    finally:
        sys.path.pop(0)
        sys.modules.pop("common", None)
        if previous is not None:
            sys.modules["common"] = previous


check = _import_validate_truth()

HAS_BINARY = any((PROJECT_ROOT / "target" / "release" / n).exists() for n in ("audd.exe", "audd"))


def mutate(out: Path, base: str, seed: int) -> Path:
    subprocess.run([sys.executable, str(MUTATE_PY), "--base", base, "--seed", str(seed),
                    "--backend", "ddl", "--out", str(out)], check=True, capture_output=True)
    return out / f"{base}_s{seed}"


def run_block_b(pairs: Path, out: Path) -> dict:
    subprocess.run([sys.executable, str(MUTATIONS_PY), "--pairs", str(pairs), "--out", str(out)],
                   check=True, capture_output=True)
    return json.loads((out / "mutations_indicators.json").read_text(encoding="utf-8"))


def ir_field(name, ctype, nullable=True, constraints=None):
    f = {"field_name": name, "canonical_type": ctype, "nullable": nullable}
    if constraints:
        f["constraints"] = constraints
    return f


def ir(fields, keys=None):
    return {"entities": [{"entity_name": "t", "fields": fields, "keys": keys or []}]}


class ValidateTruthTests(unittest.TestCase):
    def test_equivalent_types_are_not_observable_type_changes(self):
        c = {"entity": "t", "field": "v", "type": "TypeIncompatible"}
        a = ir([ir_field("v", {"type": "date_time"})])
        b = ir([ir_field("v", {"type": "timestamp"})])
        self.assertEqual(check(c, a, b), "not_observable")

    def test_widening_is_observable(self):
        c = {"entity": "t", "field": "v", "type": "TypeIncompatible"}
        a = ir([ir_field("v", {"type": "int32"})])
        b = ir([ir_field("v", {"type": "int64"})])
        self.assertEqual(check(c, a, b), "observable")

    def test_unique_key_counts_but_primary_key_does_not(self):
        c = {"entity": "t", "field": "v", "type": "ConstraintMismatch"}
        plain = ir([ir_field("v", {"type": "int32"}, False)])
        unique = ir([ir_field("v", {"type": "int32"}, False)], [{"key_type": "unique", "field_names": ["v"]}])
        primary = ir([ir_field("v", {"type": "int32"}, False)], [{"key_type": "primary", "field_names": ["v"]}])
        self.assertEqual(check(c, plain, unique), "observable")
        self.assertEqual(check(c, plain, primary), "not_observable")

    def test_wrong_label_goes_to_review(self):
        c = {"entity": "t", "field": "v", "type": "LengthMismatch"}
        a = ir([ir_field("v", {"type": "string"}, True)])
        b = ir([ir_field("v", {"type": "string"}, False)])
        self.assertEqual(check(c, a, b), "label_review")


@unittest.skipUnless(HAS_BINARY, "requiere target/release/audd")
class BlockBTests(unittest.TestCase):
    def test_generation_is_deterministic(self):
        with tempfile.TemporaryDirectory() as t1, tempfile.TemporaryDirectory() as t2:
            p1 = mutate(Path(t1), "chinook", 1000)
            p2 = mutate(Path(t2), "chinook", 1000)
            for name in ("a.sql", "b.sql", "truth.json"):
                self.assertEqual((p1 / name).read_bytes(), (p2 / name).read_bytes(), name)

    def test_identity_pair_has_no_conflicts_or_exclusives(self):
        """A frente a A: cualquier conflicto o exclusivo sería un falso positivo."""
        with tempfile.TemporaryDirectory() as tmp:
            pair = mutate(Path(tmp) / "pairs", "employees", 1000)
            shutil.copy(pair / "a.sql", pair / "b.sql")
            truth = json.loads((pair / "truth.json").read_text(encoding="utf-8"))
            truth.update(conflicts=[], exclusives=[])
            truth["matches"] = [{"a": m["a"], "b": m["a"]} for m in truth["matches"]]
            dropped = {m["table"] for m in truth["mutations"] if m["operator"] == "drop_table"}
            for t in dropped:
                truth["matches"].append({"a": t, "b": t})
            (pair / "truth.json").write_text(json.dumps(truth), encoding="utf-8")
            ind = run_block_b(Path(tmp) / "pairs", Path(tmp) / "out")
            self.assertEqual(ind["totals"].get("FP", 0), 0)
            self.assertEqual(ind["ROB"], 100.0)
            self.assertEqual(ind["REP"], 100.0)

    def test_mutated_pairs_are_observable_and_detected(self):
        with tempfile.TemporaryDirectory() as tmp:
            for seed in (1000, 1001, 1002):
                mutate(Path(tmp) / "pairs", "unicenta", seed)
            ind = run_block_b(Path(tmp) / "pairs", Path(tmp) / "out")
            self.assertEqual(ind["ROB"], 100.0)
            self.assertEqual(ind["REP"], 100.0)
            self.assertGreater(ind["truth_validation"].get("observable", 0), 0)
            self.assertNotIn("label_review", ind["truth_validation"])
            self.assertGreater(ind["PCD"], 0)


if __name__ == "__main__":
    unittest.main()
