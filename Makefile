.PHONY: eval tests acceptance clean

# Ejecuta la suite de evaluación completa (E03, E05, E08, E09)
eval:
	python evaluation/mutate.py --base evaluation/bases/growventa/grow_venta.sql --pair-id grow_venta_l2_sample --level 2 --seed 42 --out evaluation/generated/grow_venta_l2_sample
	python evaluation/run.py --all
	python evaluation/adoption.py
	python evaluation/requirements_coverage.py --run-tests
	@echo "Results available in evaluation/results/"

# Ejecuta solo los tests de Rust y de la suite
tests:
	cargo test --workspace
	python evaluation/tests/test_suite.py

# Ejecuta solo las pruebas de aceptación de Rust
acceptance:
	cargo test acceptance --workspace

# Limpia resultados y pares generados
clean:
	rm -rf evaluation/generated
	rm -rf evaluation/results
