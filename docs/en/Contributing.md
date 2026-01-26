# Contributing to AUDD

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/AUDD.git`
3. Create a branch: `git checkout -b feature/your-feature`

## Development

### Prerequisites
- Rust 1.70+ (`rustup` recommended)
- Cargo

### Build and Test
```bash
cargo build
cargo test
cargo fmt
cargo clippy
```

### Commit Guidelines
- Use conventional commits: `feat:`, `fix:`, `docs:`, etc.
- Keep commits atomic and focused
- Write clear commit messages

## Pull Requests

1. Update tests for your changes
2. Run `cargo fmt` and `cargo clippy`
3. Ensure all tests pass
4. Update documentation if needed
5. Fill out the PR template

## Code Style

- Follow Rust conventions
- Run `cargo fmt` before committing
- Address `cargo clippy` warnings

## Questions?

Open an issue or discussion on GitHub.
