#!/usr/bin/env python3
"""Reúne todos los bloques en un solo reporte con el contraste contra las metas.

Fuentes que lee (las que existan):
    <run>/manifest.json, <run>/cross_indicators.json        bloque A (cross.py)
    <run>/mutations_indicators.json                          bloque B (mutations.py)
    evaluation/manual/*/score.json, evaluation/manual/tri.json   bloque C (manual.py)
    evaluation/results/requirements_coverage.json            PRM
    evaluation/results/adoption.csv                          IAC

Fuente oficial de cada indicador (definida antes de ejecutar; ver PROTOCOLO):
    PCD, PRE, PSR, F1  -> bloque B (entorno controlado, criterio de TRL 4); el bloque C se
                          reporta aparte como evidencia en esquemas reales
    REP, ROB           -> bloques A y B
    SYM, FCI           -> bloque A
    TRI                -> bloque C
    PRM                -> pruebas de aceptación
    IAC                -> estadísticas públicas de npm y GitHub

Uso:
    python evaluation/suite/report.py --run evaluation/results/<id>
"""

from __future__ import annotations

import argparse
import csv
import sys
from pathlib import Path
from typing import Any, Dict, Optional

sys.path.insert(0, str(Path(__file__).parent))
from common import EVAL_DIR, load_json, write_json  # noqa: E402

GOALS = {  # Tabla 18 de la tesis
    "PCD": (">=", 70), "PRE": (">=", 90), "PSR": (">=", 50), "TRI": (">=", 50),
    "REP": (">=", 100), "ROB": (">=", 90), "PRM": (">=", 80), "IAC-1": (">=", 20),
}


def opt(path: Path) -> Optional[Any]:
    return load_json(path) if path.exists() else None


def verdict(name: str, value) -> str:
    if value is None or name not in GOALS:
        return "—"
    op, goal = GOALS[name]
    return "Cumple" if value >= goal else "No cumple"


def fmt(v) -> str:
    if v is None:
        return "—"
    return f"{v:.2f}" if isinstance(v, float) else str(v)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--run", required=True, type=Path)
    args = ap.parse_args()
    run = args.run

    man = opt(run / "manifest.json") or {}
    a = opt(run / "cross_indicators.json") or {}
    b = opt(run / "mutations_indicators.json") or {}
    manual = []
    for d in sorted((EVAL_DIR / "manual").glob("M*")):
        s = opt(d / "score.json")
        if s:
            manual.append(s)
    tri = opt(EVAL_DIR / "manual" / "tri.json") or []
    prm_doc = opt(EVAL_DIR / "results" / "requirements_coverage.json") or {}
    prm = (prm_doc.get("summary", {}).get("total", {}) or {}).get("percentage")
    iac1 = iac2 = None
    adoption = EVAL_DIR / "results" / "adoption.csv"
    if adoption.exists():
        with open(adoption, encoding="utf-8") as f:
            rows = list(csv.DictReader(f))
        for r in rows:  # formato de adoption.py: date, indicator, value, note
            if r.get("indicator") == "IAC-1":
                iac1 = int(float(r["value"]))
            if r.get("indicator") == "IAC-2":
                iac2 = int(float(r["value"]))

    # Bloque C agregado en forma micro
    vp = sum(s["conflicts"]["VP"] for s in manual)
    fp = sum(s["conflicts"]["FP"] for s in manual)
    fn = sum(s["conflicts"]["FN"] for s in manual)
    c_pcd = round(100 * vp / (vp + fn), 2) if vp + fn else None
    c_pre = round(100 * vp / (vp + fp), 2) if vp + fp else None
    tri_vals = [t["TRI"] for t in tri]
    tri_mean = round(sum(tri_vals) / len(tri_vals), 2) if tri_vals else None

    rep_vals = [v for v in (a.get("REP"), b.get("REP")) if v is not None]
    rob_vals = [v for v in (a.get("ROB"), b.get("ROB")) if v is not None]
    official = {
        "PCD": b.get("PCD"), "PRE": b.get("PRE"), "PSR": b.get("PSR"), "F1": b.get("F1"),
        "TRI": tri_mean, "REP": min(rep_vals) if rep_vals else None,
        "ROB": min(rob_vals) if rob_vals else None, "SYM": a.get("SYM"),
        "PRM": prm, "IAC-1": iac1, "IAC-2": iac2,
    }
    write_json(run / "indicators.json", {"official": official, "block_A": a, "block_B": b,
                                         "block_C": {"pairs": manual and [s["pair_id"] for s in manual],
                                                     "PCD": c_pcd, "PRE": c_pre, "TRI": tri}})

    env = man.get("env", {})
    L = []
    L.append("# Resultados de la evaluación de AUDD\n")
    L.append(f"- Commit: `{env.get('commit', '—')}`  etiqueta: `{env.get('tag') or '—'}`  "
             f"cambios sin commit: {env.get('dirty')}")
    L.append(f"- Binario sha256: `{man.get('audd_binary_sha256', '—')}`")
    L.append(f"- Catálogo sha256: `{man.get('catalog_sha256', '—')}`  opciones: `{' '.join(man.get('compare_args', []))}`")
    L.append(f"- Entorno: {env.get('os')} · Python {env.get('python')} · {env.get('rustc')}")
    L.append(f"- Fecha: {man.get('created', '—')}\n")

    L.append("## Contraste con las metas (Tabla 18)\n")
    L.append("| Indicador | Valor | Meta | Resultado | Fuente |")
    L.append("|---|---|---|---|---|")
    src = {"PCD": "Bloque B", "PRE": "Bloque B", "PSR": "Bloque B", "F1": "Bloque B", "TRI": "Bloque C",
           "REP": "Bloques A y B", "ROB": "Bloques A y B", "SYM": "Bloque A", "PRM": "Pruebas de aceptación",
           "IAC-1": "npm", "IAC-2": "GitHub"}
    for k, v in official.items():
        goal = f"{GOALS[k][0]} {GOALS[k][1]}" if k in GOALS else "Sin meta"
        L.append(f"| {k} | {fmt(v)} | {goal} | {verdict(k, v)} | {src[k]} |")

    L.append("\n## Bloque A. Matriz cruzada\n")
    for k in ("comparisons", "ROB", "REP", "SYM", "SYM_matches", "SYM_exclusives", "SYM_conflicts",
              "FCI_false_conflicts_identity", "FCI_false_exclusives_identity", "median_seconds"):
        L.append(f"- {k}: {fmt(a.get(k))}")

    L.append("\n## Bloque B. Mutaciones controladas\n")
    L.append(f"- Pares: {b.get('pairs', '—')} · Validación de la verdad: {b.get('truth_validation', {})}")
    L.append("\n| Tipo de conflicto | VP | FN | FP | PCD |")
    L.append("|---|---|---|---|---|")
    for t, c in (b.get("per_type") or {}).items():
        L.append(f"| {t} | {c.get('VP', 0)} | {c.get('FN', 0)} | {c.get('FP', 0)} | {fmt(c.get('PCD'))} |")

    L.append("\n## Bloque C. Pares con verdad manual\n")
    L.append("| Par | VP | FP | FN | PCD | PRE | PSR | F1 | Versiones de la verdad |")
    L.append("|---|---|---|---|---|---|---|---|---|")
    for s in manual:
        c = s["conflicts"]
        L.append(f"| {s['pair_id']} | {c['VP']} | {c['FP']} | {c['FN']} | {fmt(s['PCD'])} | "
                 f"{fmt(s['PRE'])} | {fmt(s['PSR'])} | {fmt(s['F1'])} | {s.get('truth_versions', 1)} |")
    L.append(f"| **Total (micro)** | {vp} | {fp} | {fn} | {fmt(c_pcd)} | {fmt(c_pre)} | | | |")
    if tri:
        L.append("\n| Par | t manual (min) | t asistido (min) | TRI |")
        L.append("|---|---|---|---|")
        for t in tri:
            L.append(f"| {t['pair_id']} | {t['t_manual_min']} | {t['t_asistido_min']} | {t['TRI']} |")

    (run / "summary.md").write_text("\n".join(L) + "\n", encoding="utf-8")
    print("\n".join(L))
    return 0


if __name__ == "__main__":
    sys.exit(main())
