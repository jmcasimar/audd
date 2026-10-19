#!/usr/bin/env python3
"""Bloque de mutaciones controladas: ejecuta, valida la verdad y puntúa.

Cada par vive en un directorio con:
    truth.json                  verdad generada por el mutador (registro de mutaciones)
    a.sqlite y b.sqlite         o bien pair.json con {"source_a": ..., "source_b": ...}

Pasos por par:
    1. audd inspect de A y B  -> IR de entrada
    2. validate_truth          -> marca cada conflicto como observable / no observable
    3. audd compare x runs     -> REP y ROB
    4. score                   -> VP, FP, FN, PSR, F1

Los indicadores se agregan en forma micro (se suman VP, FP y FN de todos los
pares antes de dividir), que es la forma estándar cuando los pares tienen
distinto número de conflictos.

Uso:
    python evaluation/suite/mutations.py --pairs evaluation/generated --out evaluation/results/<id>
"""

from __future__ import annotations

import argparse
import sys
from collections import Counter, defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from common import (OUTPUT_FILES, canonical_json_hash, expand_env, load_config,  # noqa: E402
                    load_json, run_compare, run_inspect, write_json)
from score import score_pair  # noqa: E402
from validate_truth import check  # noqa: E402


def sources(pair_dir: Path):
    if (pair_dir / "pair.json").exists():
        p = load_json(pair_dir / "pair.json")
        # pair.json guarda ${AUDD_MYSQL}, nunca la credencial
        return expand_env(p["source_a"]), expand_env(p["source_b"])
    return f"db:sqlite://{pair_dir / 'a.sqlite'}", f"db:sqlite://{pair_dir / 'b.sqlite'}"


def pct(n, d):
    return round(100 * n / d, 2) if d else None


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--pairs", required=True, type=Path)
    ap.add_argument("--out", required=True, type=Path)
    ap.add_argument("--skip-run", action="store_true", help="reutilizar salidas existentes (solo puntuar)")
    args = ap.parse_args()
    cfg = load_config()
    runs = int(cfg["audd"].get("runs", 3))

    rows, tot, per_type, validation = [], Counter(), defaultdict(Counter), Counter()
    psr_hit = psr_den = 0
    for pdir in sorted(d for d in args.pairs.iterdir() if (d / "truth.json").exists()):
        if "identity" in pdir.name:
            continue  # la identidad se evalúa en cross.py con la misma base en ambos lados
        work = args.out / "mutations" / pdir.name
        src_a, src_b = sources(pdir)
        ok_runs = []
        if not args.skip_run:
            run_inspect(cfg, src_a, work / "ir_a.json")
            run_inspect(cfg, src_b, work / "ir_b.json")
            for i in range(runs):
                ok_runs.append(run_compare(cfg, src_a, src_b, work / f"run_{i}")["valid_outputs"])
        else:
            work = pdir
            ok_runs = [(work / f"run_{i}" / "diff.json").exists() for i in range(runs)]

        truth = load_json(pdir / "truth.json")
        if (work / "ir_a.json").exists() and (work / "ir_b.json").exists():
            ir_a, ir_b = load_json(work / "ir_a.json"), load_json(work / "ir_b.json")
            for c in truth.get("conflicts", []):
                c["status"] = check(c, ir_a, ir_b)
                validation[c["status"]] += 1
            write_json(work / "truth.validated.json", truth)
        else:
            ir_b = None
            validation["sin_validar"] += len(truth.get("conflicts", []))

        robust = all(ok_runs)
        rep = robust and all(len({canonical_json_hash(work / f"run_{i}" / f) for i in range(runs)}) == 1
                             for f in OUTPUT_FILES)
        res = score_pair(truth, work / "run_0", ir_b) if robust else None
        if res:
            write_json(work / "score.json", res)
            for k in ("VP", "FP", "FN"):
                tot[k] += res["conflicts"][k]
            for t, c in res["per_type"].items():
                per_type[t].update(c)
            for k in ("VP", "FP", "FN"):
                tot["m_" + k] += res["matches"][k]
            n_psr = sum(1 for c in truth["conflicts"] if c.get("expected_suggestion")
                        and c.get("status", "observable") == "observable")
            psr_den_pair = n_psr - len([c for c in res["detail"]["false_negatives"] if c.get("expected_suggestion")])
            psr_den += max(psr_den_pair, 0)
            psr_hit += max(psr_den_pair, 0) - len(res["detail"]["psr_misses"])
        rows.append({"pair": pdir.name, "robust": robust, "reproducible": rep,
                     **({"PCD": res["PCD"], "PRE": res["PRE"], "PSR": res["PSR"], "F1": res["F1"],
                         **res["conflicts"]} if res else {})})

    mp = tot["m_VP"] / (tot["m_VP"] + tot["m_FP"]) if (tot["m_VP"] + tot["m_FP"]) else 0
    mr = tot["m_VP"] / (tot["m_VP"] + tot["m_FN"]) if (tot["m_VP"] + tot["m_FN"]) else 0
    indicators = {
        "pairs": len(rows),
        "PCD": pct(tot["VP"], tot["VP"] + tot["FN"]),
        "PRE": pct(tot["VP"], tot["VP"] + tot["FP"]),
        "PSR": pct(psr_hit, psr_den),
        "F1": round(2 * mp * mr / (mp + mr), 4) if (mp + mr) else 0.0,
        "ROB": pct(sum(r["robust"] for r in rows), len(rows)),
        "REP": pct(sum(bool(r["reproducible"]) for r in rows), len(rows)),
        "totals": dict(tot),
        "per_type": {t: {**dict(c), "PCD": pct(c["VP"], c["VP"] + c["FN"])} for t, c in per_type.items()},
        "truth_validation": dict(validation),
    }
    write_json(args.out / "mutations_rows.json", rows)
    write_json(args.out / "mutations_indicators.json", indicators)
    print(indicators)
    return 0


if __name__ == "__main__":
    sys.exit(main())
