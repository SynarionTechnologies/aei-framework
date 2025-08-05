# Contributing to AEI Framework

Thank you for your interest in AEI Framework! Community contributions are essential to move the project forward.

## Contribution process

1. Fork the repository and create a branch for your feature.
2. Ensure the code is formatted, linted, and tested.
3. Submit a pull request describing the proposed change.

## Commit conventions

This project follows the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification.

Examples:

- `feat: add network module`
- `fix: correct propagation`

## Code style

- Format Rust code with `cargo fmt`.
- Run static analysis with `cargo clippy -- -D warnings`.
- Tests must pass via `cargo test`.

## Tests

Every new feature should include unit or integration tests. Run the full suite before submitting your PR:

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```

Thank you for contributing!
