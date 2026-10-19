#!/usr/bin/env python3
"""
adoption.py

Recopila indicadores de transferencia (IAC-1 e IAC-2).

- IAC-1: descargas acumuladas del paquete audd-node en npm, desde su publicación
         hasta la fecha de corte (por defecto, hoy).
- IAC-2: interacciones externas (issues y PRs de cuentas distintas del autor)
         en el repositorio jmcasimar/AUDD.

Uso:
    python evaluation/adoption.py
    python evaluation/adoption.py --cutoff 2026-11-01

Salida:
    evaluation/results/adoption.csv
"""

import argparse
import csv
import json
import os
import sys
import urllib.error
import urllib.request
from datetime import date, datetime, timedelta
from pathlib import Path

RESULTS_DIR = Path(__file__).parent / "results"
NPM_PACKAGE = "audd-node"
GITHUB_REPO = "jmcasimar/AUDD"
GITHUB_TOKEN = os.environ.get("GITHUB_TOKEN")


def _get_json(url: str):
    with urllib.request.urlopen(url, timeout=30) as resp:
        return json.loads(resp.read().decode("utf-8"))


def fetch_npm_downloads(cutoff: date) -> int:
    """Descargas acumuladas desde la publicación del paquete hasta `cutoff`.

    La API de rangos de npm acepta hasta 18 meses por consulta, así que se suma
    por tramos de un año.
    """
    try:
        created = _get_json(f"https://registry.npmjs.org/{NPM_PACKAGE}")["time"]["created"]
        start = datetime.fromisoformat(created.replace("Z", "+00:00")).date()
        total = 0
        while start <= cutoff:
            end = min(start + timedelta(days=364), cutoff)
            data = _get_json(f"https://api.npmjs.org/downloads/range/{start}:{end}/{NPM_PACKAGE}")
            total += sum(d.get("downloads", 0) for d in data.get("downloads", []))
            start = end + timedelta(days=1)
        return total
    except Exception as e:
        print(f"Warning: could not fetch npm downloads: {e}", file=sys.stderr)
        return -1


def fetch_github_interactions() -> int:
    """Obtiene número de issues/PRs creados por usuarios externos."""
    headers = {"Accept": "application/vnd.github+json"}
    if GITHUB_TOKEN:
        headers["Authorization"] = f"Bearer {GITHUB_TOKEN}"
    url = f"https://api.github.com/repos/{GITHUB_REPO}/issues?state=all&per_page=100"
    req = urllib.request.Request(url, headers=headers)
    try:
        with urllib.request.urlopen(req, timeout=30) as resp:
            data = json.loads(resp.read().decode("utf-8"))
            # Filtrar por usuarios distintos del owner del repo
            owner = GITHUB_REPO.split("/")[0]
            count = sum(1 for item in data if item.get("user", {}).get("login") != owner)
            return count
    except urllib.error.HTTPError as e:
        if e.code == 403:
            print("Warning: GitHub API rate limit exceeded. Set GITHUB_TOKEN.", file=sys.stderr)
        else:
            print(f"Warning: could not fetch GitHub interactions: {e}", file=sys.stderr)
        return -1
    except Exception as e:
        print(f"Warning: could not fetch GitHub interactions: {e}", file=sys.stderr)
        return -1


def main() -> int:
    parser = argparse.ArgumentParser(description="Indicadores de adopción IAC-1 e IAC-2.")
    parser.add_argument("--cutoff", type=date.fromisoformat, default=date.today(),
                        help="fecha de corte AAAA-MM-DD (por defecto, hoy)")
    args = parser.parse_args()
    RESULTS_DIR.mkdir(parents=True, exist_ok=True)
    npm_downloads = fetch_npm_downloads(args.cutoff)
    gh_interactions = fetch_github_interactions()

    path = RESULTS_DIR / "adoption.csv"
    with open(path, "w", newline="", encoding="utf-8") as f:
        writer = csv.writer(f)
        writer.writerow(["date", "indicator", "value", "note"])
        writer.writerow([
            datetime.utcnow().isoformat(),
            "IAC-1",
            npm_downloads,
            f"npm downloads cumulative to {args.cutoff}" if npm_downloads >= 0 else "failed",
        ])
        writer.writerow([
            datetime.utcnow().isoformat(),
            "IAC-2",
            gh_interactions,
            "external issues/PRs" if gh_interactions >= 0 else "failed",
        ])

    print(f"Adoption data written to {path}")
    print(f"  IAC-1 (npm downloads): {npm_downloads}")
    print(f"  IAC-2 (GitHub interactions): {gh_interactions}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
