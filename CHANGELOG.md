# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Backpropagation training with `Network::train` and `Network::predict`
- Activation function derivatives enabling gradient descent
- XOR example, tests, and README tutorial
- Makefile with common development tasks
- GitHub Actions workflow running `make ci`
- Initial changelog
- README.md
- `Neuron`, `Synapse`, and `Network` structures
- Propagation unit tests
- Documentation and guides in English
- Tests covering all activation functions and chained propagation
- Named input/output neurons with high-level `set_inputs`, `get_outputs`, and
  `propagate_inputs` APIs
- JSON serialization and loading for networks via `save_json` and `load_json`
- Structured logging of training progress via the `log` crate.
### Changed
- Propagation logic now applies activations after weighted sums and resets all
  neuron values between runs.
- Comprehensive rustdoc for modules and public APIs.
- Neuron and synapse identifiers now use `Uuid` instead of numeric indexes.
- Project structure now separates `core` primitives and `api` network module.
- Documentation mirrored in English and French under `docs/`.
