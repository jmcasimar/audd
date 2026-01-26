# Contribuir a AUDD

## Primeros Pasos

1. Haga un fork del repositorio
2. Clone su fork: `git clone https://github.com/SU_USUARIO/AUDD.git`
3. Cree una rama: `git checkout -b feature/su-funcionalidad`

## Desarrollo

### Prerequisitos
- Rust 1.70+ (se recomienda `rustup`)
- Cargo

### Compilar y Probar
```bash
cargo build
cargo test
cargo fmt
cargo clippy
```

### Lineamientos para Commits
- Use commits convencionales: `feat:`, `fix:`, `docs:`, etc.
- Mantenga los commits atómicos y enfocados
- Escriba mensajes de commit claros

## Pull Requests

1. Actualice las pruebas para sus cambios
2. Ejecute `cargo fmt` y `cargo clippy`
3. Asegúrese de que todas las pruebas pasen
4. Actualice la documentación si es necesario
5. Complete la plantilla de PR

## Estilo de Código

- Siga las convenciones de Rust
- Ejecute `cargo fmt` antes de hacer commit
- Corrija las advertencias de `cargo clippy`

## ¿Preguntas?

Abra un issue o una discusión en GitHub.
