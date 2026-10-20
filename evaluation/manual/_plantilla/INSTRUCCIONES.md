# Instrucciones para el par

Guía completa, con ejemplos y la tabla para decidir cada tipo de conflicto:
`evaluation/PROTOCOLO_EVALUACION.md`, sección 7.

Antes de empezar, revisa el alcance (`scope_a` y `scope_b`) del par en `config/suite.toml`. Solo entran a la verdad las tablas del alcance.

## Condición manual (evaluador 1)

1. Abre únicamente el DDL de ambas bases (`bases/<id>/schema.sql`). No abras salidas de AUDD.
2. Anota la hora de inicio en `tiempos.csv` (condicion = manual).
3. Llena `verdad.csv` con una fila por elemento:
   - `coincidencia`: un campo de A que corresponde a uno de B, aunque se llamen distinto.
   - `exclusivo_a` / `exclusivo_b`: un campo sin equivalente. Si toda la tabla no tiene equivalente, escribe una sola fila con el campo vacío.
   - `conflicto`: un campo coincidente cuya definición difiere. Indica `tipo_conflicto` y `sugerencia_esperada` según la tabla de la sección 5 del protocolo. Un conflicto también cuenta como coincidencia; no hace falta repetirlo.
   - `Semantic`: mismo nombre con distinto significado. Se registra, pero no cuenta para PCD.
4. Anota la hora de fin.
5. Ejecuta `manual.py freeze <par>` y haz `git commit` de inmediato.

## Condición asistida (evaluador 2)

1. Ejecuta `manual.py run <par>`.
2. Anota la hora de inicio en `tiempos.csv` (condicion = asistido).
3. Abre `run_0/report.md` y produce el mismo documento de correspondencias que en la condición manual: confirma, corrige o completa lo que reporta AUDD.
4. Anota la hora de fin. En `segundos_audd`, anota el tiempo de ejecución (está en `run_0/stdout.log` o lo imprime `run`).

## Adjudicación

1. Ejecuta `manual.py score <par>`. Los FP y FN quedan en `score.json` → `detail`.
2. Anota cada caso en `adjudicacion.csv`, con su decisión y una justificación de una línea:
   - `error_audd`: el núcleo se equivocó; se queda como error.
   - `error_verdad`: la verdad estaba mal. Corrige `verdad.csv` y vuelve a ejecutar `freeze`.
   - `ambiguo`: se conserva la verdad original.
3. Vuelve a ejecutar `score`.
