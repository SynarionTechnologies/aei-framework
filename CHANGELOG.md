# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Random neuron activation mutation through `MutateRandomNeuronActivationCommand` and `MutateRandomNeuronActivationHandler`.
- Random synapse weight mutation through `MutateRandomSynapseWeightCommand` and `MutateRandomSynapseWeightHandler`.
- Event-driven example and synapse command tests.
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
- Random synapse creation through `AddRandomSynapseCommand` and `AddRandomSynapseHandler`.
- Event-sourced random neuron addition via `AddRandomNeuronCommand` and
  `AddRandomNeuronHandler`.
- Event-sourced random neuron removal via `RemoveRandomNeuronCommand` and
  `RemoveRandomNeuronHandler`.
- Event-sourced random synapse removal via `RemoveRandomSynapseCommand` and
  `RemoveRandomSynapseHandler`.
### Changed
- Commands and queries now reside in the `application` module.
- Domain events moved under `domain` and exposed via `domain::events`.
- Read models implemented as projections in `infrastructure/projection`.
- Propagation logic now applies activations after weighted sums and resets all
  neuron values between runs.
- Comprehensive rustdoc for modules and public APIs.
- Neuron and synapse identifiers now use `Uuid` instead of numeric indexes.
- Removed legacy `api` and `core` modules; primitives moved into `domain`.
- Documentation mirrored in English and French under `docs/`.
### Removed
- Unused `NodeList` and `TopoOrder` type aliases in the network API.
- Empty `modules` crate from the workspace.
- Direct `Network::add_random_neuron` and `Network::remove_random_neuron`
  methods in favor of event-driven handlers.
