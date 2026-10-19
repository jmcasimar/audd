#!/usr/bin/env python3
"""Flujo de evaluación manual (3 a 5 pares con verdad de referencia elaborada a mano).

Orden obligatorio (garantiza que la verdad se construye sin ver la salida de AUDD):

    1. init    crea manual/<par>/ con las plantillas
    2. (a mano) llenar verdad.csv y registrar el tiempo en tiempos.csv (condición "manual")
    3. freeze  valida verdad.csv, genera truth.json y lo sella con su hash
               -> hacer `git commit` en este momento: el commit prueba que la verdad es previa
    4. run     ejecuta AUDD 3 veces sobre el par (se niega si la verdad no está sellada)
    5. (a mano) condición "asistido": producir el documento de correspondencias a partir
               del reporte de AUDD y registrar el tiempo en tiempos.csv
    6. score   calcula PCD, PRE, PSR y F1 y lista FP/FN para la adjudicación
    7. (a mano) adjudicacion.csv: para cada FP/FN decidir si el error es de AUDD o de la verdad
    8. tri     calcula TRI de todos los pares con tiempos.csv

Uso:
    python evaluation/suite/manual.py init  M2_unicenta_growventa
    python evaluation/suite/manual.py freeze M2_unicenta_growventa
    python evaluation/suite/manual.py run    M2_unicenta_growventa
    python evaluation/suite/manual.py score  M2_unicenta_growventa
    python evaluation/suite/manual.py tri
"""

from __future__ import annotations

import argparse
import csv
import shutil
import sys
from datetime import datetime
from pathlib import Path
from typing import Any, Dict, List

sys.path.insert(0, str(Path(__file__).parent))
from common import (EVAL_DIR, bases_by_id, load_config, load_json, run_compare,  # noqa: E402
                    run_inspect, sha256_file, source_of, write_json)
from score import score_pair  # noqa: E402

MANUAL_DIR = EVAL_DIR / "manual"
TEMPLATE_DIR = MANUAL_DIR / "_plantilla"

KINDS = {"coincidencia", "exclusivo_a", "exclusivo_b", "conflicto"}
CONFLICT_TYPES = {"TypeIncompatible", "NullabilityMismatch", "ConstraintMismatch",
                  "LengthMismatch", "NormalizationCollision", "Semantic"}
SUGGESTIONS = {"", "CastSafe", "CastRisky", "RenameField", "PreferType", "NoSuggestion"}


def pair_cfg(cfg: Dict[str, Any], pid: str) -> Dict[str, Any]:
    for p in cfg["manual"]:
        if p["id"] == pid:
            return p
    raise SystemExit(f"El par {pid} no está en [[manual]] de config/suite.toml")


def cmd_init(cfg, pid: str) -> None:
    p = pair_cfg(cfg, pid)
    d = MANUAL_DIR / pid
    if d.exists():
        raise SystemExit(f"{d} ya existe")
    d.mkdir(parents=True)
    for name in ("verdad.csv", "tiempos.csv", "adjudicacion.csv", "INSTRUCCIONES.md"):
        shutil.copy(TEMPLATE_DIR / name, d / name)
    print(f"Creado {d}. A = {p['a']}, B = {p['b']}. Alcance A: {p.get('scope_a')}, B: {p.get('scope_b')}")


def read_truth_csv(path: Path) -> List[Dict[str, str]]:
    rows = []
    with open(path, encoding="utf-8-sig", newline="") as f:
        for i, r in enumerate(csv.DictReader(f), start=2):
            r = {k.strip(): (v or "").strip() for k, v in r.items() if k}
            if not r.get("tipo") or r["tipo"].startswith("#"):
                continue
            errs = []
            if r["tipo"] not in KINDS:
                errs.append(f"tipo inválido '{r['tipo']}'")
            if r["tipo"] in ("coincidencia", "conflicto", "exclusivo_a") and not r.get("entidad_a"):
                errs.append("falta entidad_a")
            if r["tipo"] in ("coincidencia", "conflicto", "exclusivo_b") and not r.get("entidad_b"):
                errs.append("falta entidad_b")
            if r["tipo"] == "conflicto":
                if r.get("tipo_conflicto") not in CONFLICT_TYPES:
                    errs.append(f"tipo_conflicto inválido '{r.get('tipo_conflicto')}'")
                if r.get("sugerencia_esperada", "") not in SUGGESTIONS:
                    errs.append(f"sugerencia_esperada inválida '{r.get('sugerencia_esperada')}'")
            if errs:
                raise SystemExit(f"verdad.csv línea {i}: " + "; ".join(errs))
            rows.append(r)
    return rows


def to_truth_json(pid: str, p: Dict[str, Any], rows: List[Dict[str, str]]) -> Dict[str, Any]:
    def ref(e, f):
        return f"{e}.{f}" if f else e

    matches, excl, confl = [], [], []
    seen = set()
    for r in rows:
        if r["tipo"] in ("coincidencia", "conflicto"):
            k = (r["entidad_a"].lower(), r.get("campo_a", "").lower())
            if k not in seen:
                seen.add(k)
                matches.append({"a": ref(r["entidad_a"], r.get("campo_a")),
                                "b": ref(r["entidad_b"], r.get("campo_b"))})
        if r["tipo"] == "exclusivo_a":
            excl.append({"side": "a", "entity": r["entidad_a"], "field": r.get("campo_a") or None})
        if r["tipo"] == "exclusivo_b":
            excl.append({"side": "b", "entity": r["entidad_b"], "field": r.get("campo_b") or None})
        if r["tipo"] == "conflicto":
            confl.append({"entity": r["entidad_a"], "field": r.get("campo_a") or None,
                          "entity_b": r["entidad_b"], "field_b": r.get("campo_b") or None,
                          "type": r["tipo_conflicto"],
                          "expected_suggestion": r.get("sugerencia_esperada") or None,
                          "note": r.get("nota", "")})
    return {"pair_id": pid, "block": "manual", "a": p["a"], "b": p["b"],
            "scope": {"a": p.get("scope_a"), "b": p.get("scope_b")},
            "matches": matches, "exclusives": excl, "conflicts": confl}


def cmd_freeze(cfg, pid: str) -> None:
    p = pair_cfg(cfg, pid)
    d = MANUAL_DIR / pid
    if (d / "run_0").exists() and not (d / "truth.lock.json").exists():
        raise SystemExit("Ya hay salidas de AUDD y la verdad no estaba sellada: la verdad dejó de ser ciega.")
    rows = read_truth_csv(d / "verdad.csv")
    truth = to_truth_json(pid, p, rows)
    write_json(d / "truth.json", truth)
    lock_path = d / "truth.lock.json"
    history = load_json(lock_path)["history"] if lock_path.exists() else []
    history.append({"sealed_at": datetime.now().isoformat(timespec="seconds"),
                    "verdad_csv_sha256": sha256_file(d / "verdad.csv"),
                    "after_run": (d / "run_0").exists(),
                    "n": {"matches": len(truth["matches"]), "exclusives": len(truth["exclusives"]),
                          "conflicts": len(truth["conflicts"])}})
    write_json(lock_path, {"pair_id": pid, "history": history})
    print(f"Verdad sellada (versión {len(history)}): {history[-1]['n']}")
    if len(history) == 1:
        print(f'Ahora: git add evaluation/manual/{pid} && git commit -m "Verdad de referencia {pid} (ciega)"')
    else:
        print("Versión posterior a la ejecución: registra el motivo de cada cambio en adjudicacion.csv")


def cmd_run(cfg, pid: str) -> None:
    p = pair_cfg(cfg, pid)
    d = MANUAL_DIR / pid
    if not (d / "truth.lock.json").exists():
        raise SystemExit("Primero sella la verdad: manual.py freeze " + pid)
    bases = bases_by_id(cfg)
    a, b = bases[p["a"]], bases[p["b"]]
    for i in range(int(cfg["audd"].get("runs", 3))):
        r = run_compare(cfg, source_of(a), source_of(b), d / f"run_{i}")
        print(f"run_{i}: {r}")
    run_inspect(cfg, source_of(b), d / "ir_b.json")
    print("Siguiente: condición asistida (registrar en tiempos.csv) y luego manual.py score " + pid)


def cmd_score(cfg, pid: str) -> None:
    d = MANUAL_DIR / pid
    truth = load_json(d / "truth.json")
    ir_b = load_json(d / "ir_b.json") if (d / "ir_b.json").exists() else None
    res = score_pair(truth, d / "run_0", ir_b)
    res["truth_versions"] = len(load_json(d / "truth.lock.json")["history"])
    write_json(d / "score.json", res)
    print({k: res[k] for k in ("PCD", "PRE", "PSR", "F1", "conflicts")})
    print(f"Falsos negativos: {len(res['detail']['false_negatives'])}, "
          f"falsos positivos: {len(res['detail']['false_positives'])} -> adjudicar en adjudicacion.csv")


def _parse(ts: str) -> datetime:
    return datetime.fromisoformat(ts.replace(" ", "T"))


def cmd_tri(cfg) -> None:
    out = []
    for d in sorted(MANUAL_DIR.glob("M*")):
        f = d / "tiempos.csv"
        if not f.exists():
            continue
        t = {"manual": 0.0, "asistido": 0.0}
        with open(f, encoding="utf-8-sig", newline="") as fh:
            for r in csv.DictReader(fh):
                if not r.get("condicion") or r["condicion"].startswith("#"):
                    continue
                secs = (_parse(r["fin"]) - _parse(r["inicio"])).total_seconds()
                secs += float(r.get("segundos_audd") or 0)
                t[r["condicion"].strip()] += secs
        if t["manual"] and t["asistido"]:
            tri = round(100 * (t["manual"] - t["asistido"]) / t["manual"], 2)
            out.append({"pair_id": d.name, "t_manual_min": round(t["manual"] / 60, 1),
                        "t_asistido_min": round(t["asistido"] / 60, 1), "TRI": tri})
    write_json(MANUAL_DIR / "tri.json", out)
    for r in out:
        print(r)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("command", choices=["init", "freeze", "run", "score", "tri"])
    ap.add_argument("pair", nargs="?")
    args = ap.parse_args()
    cfg = load_config()
    if args.command == "tri":
        cmd_tri(cfg)
    else:
        if not args.pair:
            raise SystemExit("Indica el id del par")
        {"init": cmd_init, "freeze": cmd_freeze, "run": cmd_run, "score": cmd_score}[args.command](cfg, args.pair)
    return 0


if __name__ == "__main__":
    sys.exit(main())
