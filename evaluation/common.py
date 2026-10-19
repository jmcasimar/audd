#!/usr/bin/env python3
"""
common.py

Funciones compartidas entre run.py, real_pairs.py y otros scripts de evaluación.
"""

import hashlib
import json
import platform
import shutil
import subprocess
import sys
import time
from pathlib import Path
from typing import Dict, List

EVAL_DIR = Path(__file__).parent.resolve()
PROJECT_ROOT = EVAL_DIR.parent
RESULTS_DIR = EVAL_DIR / "results"
GENERATED_DIR = EVAL_DIR / "generated"
BASES_DIR = EVAL_DIR / "bases"


def audd_binary() -> List[str]:
    """Devuelve el comando para ejecutar audd compare."""
    release_exe = PROJECT_ROOT / "target" / "release" / "audd.exe"
    debug_exe = PROJECT_ROOT / "target" / "debug" / "audd.exe"
    unix_release = PROJECT_ROOT / "target" / "release" / "audd"
    unix_debug = PROJECT_ROOT / "target" / "debug" / "audd"

    candidates = [release_exe, debug_exe, unix_release, unix_debug]
    for c in candidates:
        if c.exists():
            return [str(c)]
    return ["cargo", "run", "--release", "--"]


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(8192), b""):
            h.update(chunk)
    return h.hexdigest()


def canonical_hash(path: Path) -> str:
    """Calcula hash canónico de un JSON (listas ordenadas, claves ordenadas)."""
    try:
        data = load_json(path)
    except Exception:
        return sha256_file(path)

    def sort_lists(obj):
        if isinstance(obj, dict):
            return {k: sort_lists(v) for k, v in sorted(obj.items())}
        if isinstance(obj, list):
            sorted_list = [sort_lists(x) for x in obj]
            try:
                return sorted(sorted_list, key=lambda x: json.dumps(x, sort_keys=True))
            except TypeError:
                return sorted_list
        return obj

    canonical = json.dumps(sort_lists(data), sort_keys=True, ensure_ascii=False)
    return hashlib.sha256(canonical.encode("utf-8")).hexdigest()


def load_json(path: Path) -> Dict:
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def get_rustc_version() -> str:
    try:
        return subprocess.run(["rustc", "--version"], capture_output=True, text=True).stdout.strip()
    except Exception:
        return "unknown"


def get_cargo_version() -> str:
    try:
        return subprocess.run(["cargo", "--version"], capture_output=True, text=True).stdout.strip()
    except Exception:
        return "unknown"


def write_env() -> None:
    env = {
        "os": platform.system(),
        "os_version": platform.release(),
        "python": sys.version,
        "rustc": get_rustc_version(),
        "cargo": get_cargo_version(),
    }
    (RESULTS_DIR / "env.json").write_text(json.dumps(env, indent=2), encoding="utf-8")
