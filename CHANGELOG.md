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
### Changed
- Propagation logic now applies activations after weighted sums and resets all
  neuron values between runs.
- Comprehensive rustdoc for modules and public APIs.
- Neurons carry a `Uuid` preparing for global identifiers.
