#!/usr/bin/env python3
"""
adoption.py

Recopila indicadores de transferencia (IAC-1 e IAC-2).

- IAC-1: descargas mensuales del paquete audd-node desde npm.
- IAC-2: interacciones externas (issues y PRs de cuentas distintas del autor)
         en el repositorio jmcasimar/AUDD.

Uso:
    python evaluation/adoption.py

Salida:
    evaluation/results/adoption.csv
"""

import csv
import json
import os
import sys
import urllib.error
import urllib.request
from datetime import datetime, timedelta
from pathlib import Path

RESULTS_DIR = Path(__file__).parent / "results"
NPM_PACKAGE = "audd-node"
GITHUB_REPO = "jmcasimar/AUDD"
GITHUB_TOKEN = os.environ.get("GITHUB_TOKEN")


def fetch_npm_downloads() -> int:
    """Obtiene descargas del último mes desde la API de npm."""
    end = datetime.utcnow().replace(day=1) - timedelta(days=1)
    start = end.replace(day=1)
    period = f"{start:%Y-%m-%d}:{end:%Y-%m-%d}"
    url = f"https://api.npmjs.org/downloads/point/{period}/{NPM_PACKAGE}"
    try:
        with urllib.request.urlopen(url, timeout=30) as resp:
            data = json.loads(resp.read().decode("utf-8"))
            return data.get("downloads", 0) or 0
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
    RESULTS_DIR.mkdir(parents=True, exist_ok=True)
    npm_downloads = fetch_npm_downloads()
    gh_interactions = fetch_github_interactions()

    path = RESULTS_DIR / "adoption.csv"
    with open(path, "w", newline="", encoding="utf-8") as f:
        writer = csv.writer(f)
        writer.writerow(["date", "indicator", "value", "note"])
        writer.writerow([
            datetime.utcnow().isoformat(),
            "IAC-1",
            npm_downloads,
            "npm downloads last month" if npm_downloads >= 0 else "failed",
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
