#!/usr/bin/env python3
"""Preparación única de las bases MySQL (PROTOCOLO_EVALUACION.md, sección 3.1).

Para cada base de config/suite.toml con `source = "db:mysql://..."` y un `ddl` versionado:
    1. recrea la base de datos indicada en `source` y carga el DDL;
    2. guarda `bases/<id>/schema.sql` con `mysqldump --no-data` (estructura exacta).

Requiere los clientes `mysql` y `mysqldump` en el PATH y la variable AUDD_MYSQL.

Uso:
    python evaluation/suite/setup_mysql.py               # todas las bases MySQL
    python evaluation/suite/setup_mysql.py --only chinook employees
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import mysql  # noqa: E402
from common import EVAL_DIR, load_config, sha256_file  # noqa: E402


# El DDL se carga en la base indicada en `source`: se quitan las sentencias que la cambian
_DB_STATEMENTS = re.compile(r"^\s*(CREATE|DROP)\s+(DATABASE|SCHEMA)\b[^;]*;|^\s*USE\s+[^;]*;",
                            re.IGNORECASE | re.MULTILINE)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--only", nargs="*", help="limitar a estas bases (ids)")
    args = ap.parse_args()

    cfg = load_config()
    for base in cfg["base"]:
        if args.only and base["id"] not in args.only:
            continue
        if not base["source"].startswith("db:mysql://"):
            print(f"{base['id']}: fuente no MySQL, se omite")
            continue
        if not base.get("ddl"):
            print(f"{base['id']}: sin `ddl` en suite.toml, se omite")
            continue
        database = base["source"].rstrip("/").rsplit("/", 1)[-1]
        ddl = (EVAL_DIR / base["ddl"]).read_text(encoding="utf-8", errors="replace")
        ddl = _DB_STATEMENTS.sub("", ddl)
        mysql.recreate_database(database)
        mysql.load_sql(database, ddl)
        out = EVAL_DIR / "bases" / base["id"] / "schema.sql"
        mysql.dump_schema(database, out)
        print(f"{base['id']}: cargada en `{database}`, estructura en {out.relative_to(EVAL_DIR)}")
        print(f"    sha256 {sha256_file(out)}  (registrar en datasets.toml)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
