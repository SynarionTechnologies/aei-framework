# Autonomous & Evolutive Intelligence Framework (AEIF)

AEIF is an open, modular Rust framework for building evolutive, autonomous AI agents — **Autonomous Conscious Units (ACUs)**.  
It promotes transparent, reproducible research by allowing developers to mix and match memory layers, knowledge modules, and runtime environments.

## Project structure

```
aei-framework/
├── crates/
│   ├── core/        # Core traits and types for ACUs
│   ├── memory/      # Persistence and memory abstraction
│   ├── runtime/     # Agent orchestration and scheduling
│   ├── modules/     # Plug-and-play knowledge modules
│   └── utils/       # Shared utilities
├── examples/        # Example binaries
├── tests/           # Integration tests
├── README.md        # Project overview
├── Cargo.toml       # Workspace configuration
└── .gitignore
```

## Getting started

You can use Cargo directly or the provided Makefile for common development tasks.

```bash
# Build the entire workspace
cargo build

# Run tests
cargo test

# Run an example
cargo run --example basic

# Format, lint and test using the Makefile
make ci
```

## Crates

- `core`: foundational traits and abstractions for ACUs.
- `memory`: persistence and memory layer interfaces.
- `runtime`: agent runtime and scheduling infrastructure.
- `modules`: plug-and-play knowledge modules.
- `utils`: shared utilities used across the workspace.

## Contributing

AEIF welcomes contributions from researchers and developers. Feel free to open issues and pull requests.

GitHub Actions runs `make ci` on pushes and pull requests targeting `main` to ensure the workspace builds and tests successfully.

See [CHANGELOG.md](CHANGELOG.md) for a list of notable changes.

## License

Distributed under the terms of the Mozilla Public License 2.0 (MPL-2.0). See the [LICENSE](LICENSE) file for details.
