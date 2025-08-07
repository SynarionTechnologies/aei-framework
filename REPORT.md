# Refactor Report

## Removed / Refactored
- Removed legacy `src/api` module.
- Moved `activation.rs`, `neuron.rs`, and `synapse.rs` into `src/domain`.
- Updated imports, documentation, and tests to match the new layout.

## Architecture Choices
- `domain` now contains primitive types (`Activation`, `Neuron`, `Synapse`).
- Documentation is mirrored in English and French under `docs/en` and `docs/fr`.

## TODO / Future Work
- Review and refactor the crates under `crates/`.
- Expand examples and user guides.
- Improve French translations and keep documentation in sync.
