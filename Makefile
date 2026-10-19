.PHONY: eval eval-suite setup-mysql tests acceptance clean

# Carpeta de la corrida (no se sobrescribe) y backend del bloque B (mysql | ddl)
RUN ?= evaluation/results/$(shell date +%Y%m%d-%H%M)_$(shell git rev-parse --short HEAD)
BACKEND ?= mysql

# Suite de evaluación completa (PROTOCOLO_EVALUACION.md, sección 10). Requiere AUDD_MYSQL.
eval-suite:
	cargo build --release -p audd-cli
	python evaluation/suite/cross.py --out $(RUN)
	python evaluation/mutate.py --all --backend $(BACKEND)
	python evaluation/suite/mutations.py --pairs evaluation/generated --out $(RUN)
	python evaluation/requirements_coverage.py
	python evaluation/adoption.py
	python evaluation/suite/report.py --run $(RUN)
	@echo "Resultados en $(RUN)"

# Alias de eval-suite (la suite anterior, evaluation/run.py, está obsoleta)
eval: eval-suite

# Preparación única: cargar las bases en MySQL y generar bases/<id>/schema.sql
setup-mysql:
	python evaluation/suite/setup_mysql.py

# Ejecuta solo los tests de Rust y de la suite
tests:
	cargo test --workspace
	cargo build --release -p audd-cli
	python -m unittest discover -s evaluation/tests

# Ejecuta solo las pruebas de aceptación de Rust
acceptance:
	cargo test acceptance --workspace

# Limpia pares generados (las corridas en evaluation/results/ se conservan)
clean:
	rm -rf evaluation/generated
