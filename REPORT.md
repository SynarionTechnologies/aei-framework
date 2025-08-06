# Refactor Report

## Removed / Refactored
- Moved `activation.rs`, `neuron.rs`, and `synapse.rs` into `src/core`.
- Moved `network.rs` into `src/api`.
- Updated imports and tests to match the new module layout.

## Architecture Choices
- `core` contains primitive types (`Activation`, `Neuron`, `Synapse`).
- `api` exposes the `Network` type and future public interfaces.
- Documentation is mirrored in English and French under `docs/en` and `docs/fr`.

## TODO / Future Work
- Review and refactor the crates under `crates/`.
- Expand examples and user guides.
- Improve French translations and keep documentation in sync.
