"""Utilidades compartidas de la suite de evaluación de AUDD.

Todo lo que define *cómo se cuenta* (normalización de nombres, claves de
comparación, tipos de conflicto) vive aquí para que los scripts usen
exactamente la misma regla.
"""

from __future__ import annotations

import hashlib
import json
import os
import platform
import re
import subprocess
import sys
import time
import tomllib
from pathlib import Path
from typing import Any, Dict, Iterable, List, Optional, Tuple

SUITE_DIR = Path(__file__).parent.resolve()
EVAL_DIR = SUITE_DIR.parent
PROJECT_ROOT = EVAL_DIR.parent
CONFIG_PATH = EVAL_DIR / "config" / "suite.toml"

# Los cinco tipos estructurales que mide PCD. Cualquier otro tipo en la
# verdad de referencia (p. ej. "Semantic") se reporta aparte como limitación.
STRUCTURAL_TYPES = {
    "typeincompatible",
    "nullabilitymismatch",
    "constraintmismatch",
    "lengthmismatch",
    "normalizationcollision",
}

OUTPUT_FILES = ["diff.json", "unified_schema.json", "suggestions.json", "decision_log.json"]


# --------------------------------------------------------------------------
# Configuración
# --------------------------------------------------------------------------

_ENV_RE = re.compile(r"\$\{([A-Z0-9_]+)\}")


def expand_env(value: str) -> str:
    """Sustituye ${VAR} por variables de entorno (las credenciales no van al repo)."""

    def repl(m: re.Match) -> str:
        var = m.group(1)
        if var not in os.environ:
            raise SystemExit(f"Falta la variable de entorno {var} (ver config/suite.toml)")
        return os.environ[var]

    return _ENV_RE.sub(repl, value)


def load_config(path: Path = CONFIG_PATH) -> Dict[str, Any]:
    with open(path, "rb") as f:
        cfg = tomllib.load(f)
    cfg.setdefault("audd", {})
    cfg.setdefault("base", [])
    cfg.setdefault("manual", [])
    return cfg


def bases_by_id(cfg: Dict[str, Any]) -> Dict[str, Dict[str, Any]]:
    return {b["id"]: b for b in cfg["base"]}


def source_of(base: Dict[str, Any]) -> str:
    src = expand_env(base["source"])
    if src.startswith("db:sqlite://"):
        p = src[len("db:sqlite://"):]
        if not os.path.isabs(p):
            src = "db:sqlite://" + str((EVAL_DIR / p).resolve())
    return src


# --------------------------------------------------------------------------
# Ejecución de AUDD
# --------------------------------------------------------------------------


def audd_cmd(cfg: Dict[str, Any]) -> List[str]:
    configured = cfg["audd"].get("binary")
    candidates = []
    if configured:
        candidates.append(PROJECT_ROOT / configured)
    for name in ("audd.exe", "audd"):
        candidates.append(PROJECT_ROOT / "target" / "release" / name)
    for c in candidates:
        if c.exists():
            return [str(c)]
    raise SystemExit(
        "No se encontró el binario de AUDD. Compila con `cargo build --release -p audd-cli` "
        "o indica la ruta en [audd].binary de config/suite.toml"
    )


def compare_args(cfg: Dict[str, Any]) -> List[str]:
    """Opciones fijas de `audd compare` para toda la evaluación."""
    a = cfg["audd"]
    args = ["--deterministic"]
    if a.get("catalog"):
        args += ["--catalog", str(PROJECT_ROOT / a["catalog"])]
    if a.get("domain"):
        args += ["--domain", a["domain"]]
    return args


def run_compare(cfg: Dict[str, Any], src_a: str, src_b: str, out_dir: Path) -> Dict[str, Any]:
    out_dir.mkdir(parents=True, exist_ok=True)
    cmd = audd_cmd(cfg) + ["compare", "--source-a", src_a, "--source-b", src_b,
                           "--out", str(out_dir)] + compare_args(cfg)
    t0 = time.perf_counter()
    proc = subprocess.run(cmd, cwd=PROJECT_ROOT, capture_output=True, text=True,
                          encoding="utf-8", errors="replace")
    elapsed = time.perf_counter() - t0
    (out_dir / "stdout.log").write_text(proc.stdout or "", encoding="utf-8")
    (out_dir / "stderr.log").write_text(proc.stderr or "", encoding="utf-8")
    valid = proc.returncode == 0 and all(_valid_json(out_dir / f) for f in OUTPUT_FILES)
    return {"exit_code": proc.returncode, "seconds": round(elapsed, 4), "valid_outputs": valid}


def run_inspect(cfg: Dict[str, Any], src: str, out_file: Path) -> int:
    out_file.parent.mkdir(parents=True, exist_ok=True)
    cmd = audd_cmd(cfg) + ["inspect", "--source", src, "--out", str(out_file)]
    proc = subprocess.run(cmd, cwd=PROJECT_ROOT, capture_output=True, text=True,
                          encoding="utf-8", errors="replace")
    if proc.returncode != 0:
        print(proc.stderr, file=sys.stderr)
    return proc.returncode


def _valid_json(p: Path) -> bool:
    try:
        with open(p, encoding="utf-8") as f:
            json.load(f)
        return True
    except Exception:
        return False


# --------------------------------------------------------------------------
# Hashes y entorno (reproducibilidad)
# --------------------------------------------------------------------------


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(65536), b""):
            h.update(chunk)
    return h.hexdigest()


def sha256_dir(path: Path, pattern: str = "**/*") -> str:
    h = hashlib.sha256()
    for p in sorted(path.glob(pattern)):
        if p.is_file():
            h.update(str(p.relative_to(path)).replace("\\", "/").encode())
            h.update(sha256_file(p).encode())
    return h.hexdigest()


def canonical_json_hash(path: Path) -> str:
    """Hash del JSON con claves y listas ordenadas: ignora el orden, no el contenido."""
    with open(path, encoding="utf-8") as f:
        data = json.load(f)

    def canon(o):
        if isinstance(o, dict):
            return {k: canon(v) for k, v in sorted(o.items())}
        if isinstance(o, list):
            items = [canon(x) for x in o]
            return sorted(items, key=lambda x: json.dumps(x, sort_keys=True, ensure_ascii=False))
        return o

    s = json.dumps(canon(data), sort_keys=True, ensure_ascii=False)
    return hashlib.sha256(s.encode("utf-8")).hexdigest()


def git(*args: str) -> str:
    try:
        return subprocess.run(["git", *args], cwd=PROJECT_ROOT, capture_output=True,
                              text=True).stdout.strip()
    except Exception:
        return ""


def environment() -> Dict[str, str]:
    def ver(cmd):
        try:
            return subprocess.run(cmd, capture_output=True, text=True).stdout.strip()
        except Exception:
            return ""

    return {
        "os": f"{platform.system()} {platform.release()}",
        "python": sys.version.split()[0],
        "rustc": ver(["rustc", "--version"]),
        "cargo": ver(["cargo", "--version"]),
        "commit": git("rev-parse", "HEAD"),
        "tag": git("describe", "--tags", "--exact-match"),
        "dirty": bool(git("status", "--porcelain")),
    }


# --------------------------------------------------------------------------
# Normalización y lectura de salidas de AUDD
# --------------------------------------------------------------------------


def norm(name: Optional[str]) -> str:
    if not name:
        return ""
    return re.sub(r"[\s_\-`\"\[\]]", "", name).lower()


def norm_type(t: Optional[str]) -> str:
    return norm(t)


def load_json(path: Path) -> Any:
    with open(path, encoding="utf-8") as f:
        return json.load(f)


def write_json(path: Path, data: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        json.dump(data, f, ensure_ascii=False, indent=2)


def entity_scope_ok(entity: str, scope: Optional[Iterable[str]]) -> bool:
    if not scope:
        return True
    return norm(entity) in {norm(s) for s in scope}


def resolve_b_names(diff: Dict[str, Any], ir_b: Optional[Dict[str, Any]]) -> Dict[Tuple[str, str], Tuple[str, str]]:
    """Intenta recuperar los nombres del lado B de cada coincidencia.

    diff.json solo trae el nombre del lado A y los índices. Con la IR de B
    (snapshot de `audd inspect`) se resuelve: coincidencias de entidad ->
    entities[index_b]; coincidencias de campo -> fields[index_b] de la entidad
    B emparejada. Si no se puede resolver, se omite (el puntaje cae a la clave A).
    """
    if not ir_b:
        return {}
    ents_b = ir_b.get("entities", [])
    ent_map: Dict[str, str] = {}
    for m in diff.get("matches", []):
        if m.get("field_name") is None and 0 <= m.get("index_b", -1) < len(ents_b):
            ent_map[norm(m["entity_name"])] = ents_b[m["index_b"]]["entity_name"]
    out: Dict[Tuple[str, str], Tuple[str, str]] = {}
    by_name = {e["entity_name"]: e for e in ents_b}
    for m in diff.get("matches", []):
        ea = m["entity_name"]
        eb = ent_map.get(norm(ea))
        if eb is None:
            continue
        if m.get("field_name") is None:
            out[(norm(ea), "")] = (eb, "")
        else:
            fields = by_name.get(eb, {}).get("fields", [])
            ib = m.get("index_b", -1)
            if 0 <= ib < len(fields):
                out[(norm(ea), norm(m["field_name"]))] = (eb, fields[ib]["field_name"])
    return out
