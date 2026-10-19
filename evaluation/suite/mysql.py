"""Acceso a MySQL/MariaDB para la suite, mediante los clientes `mysql` y `mysqldump`.

Las credenciales se leen de la variable de entorno AUDD_MYSQL
("usuario:clave@host:puerto"); nunca se escriben en disco. La clave se pasa a
los clientes por MYSQL_PWD para que no aparezca en la línea de comandos.
"""

from __future__ import annotations

import os
import shutil
import subprocess
from pathlib import Path
from typing import Dict, List, Optional

ENV_VAR = "AUDD_MYSQL"


def credentials() -> Dict[str, str]:
    raw = os.environ.get(ENV_VAR)
    if not raw:
        raise SystemExit(f"Falta la variable de entorno {ENV_VAR} (usuario:clave@host:puerto)")
    userpass, _, hostport = raw.rpartition("@")
    if not userpass:
        raise SystemExit(f"{ENV_VAR} debe tener la forma usuario:clave@host:puerto")
    user, _, password = userpass.partition(":")
    host, _, port = hostport.partition(":")
    return {"user": user, "password": password, "host": host or "127.0.0.1", "port": port or "3306"}


def _client(name: str) -> str:
    exe = shutil.which(name)
    if not exe:
        raise SystemExit(f"No se encontró `{name}` en el PATH (cliente de MySQL/MariaDB)")
    return exe


def _run(args: List[str], stdin: Optional[str] = None) -> str:
    c = credentials()
    env = dict(os.environ, MYSQL_PWD=c["password"])
    proc = subprocess.run(
        args + ["-h", c["host"], "-P", c["port"], "-u", c["user"]],
        input=stdin, capture_output=True, text=True, encoding="utf-8", env=env,
    )
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr.strip() or f"{args[0]} terminó con código {proc.returncode}")
    return proc.stdout


def recreate_database(name: str) -> None:
    _run([_client("mysql"), "-e",
          f"DROP DATABASE IF EXISTS `{name}`; "
          f"CREATE DATABASE `{name}` CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci"])


def load_sql(database: str, sql: str) -> None:
    """Ejecuta un script DDL dentro de `database` (sin revisar claves foráneas)."""
    _run([_client("mysql"), "--database", database],
         stdin="SET FOREIGN_KEY_CHECKS=0;\n" + sql + "\nSET FOREIGN_KEY_CHECKS=1;\n")


def dump_schema(database: str, out: Path) -> None:
    """`mysqldump --no-data`: solo estructura, sin datos."""
    text = _run([_client("mysqldump"), "--no-data", "--skip-comments", "--skip-dump-date",
                 "--routines=false", "--triggers=false", database])
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(text, encoding="utf-8")


def source_url(database: str) -> str:
    """URL de AUDD con la credencial como variable, para guardarla en disco sin la clave."""
    return f"db:mysql://${{{ENV_VAR}}}/{database}"
