# AEI Framework (AEIF)

[![Build](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/owner/aei-framework/actions)
[![License: MPL-2.0](https://img.shields.io/badge/license-MPL%202.0-blue)](LICENSE)

AEI Framework is an open source Rust framework for building dynamic, modular, scalable, embeddable, and multi-agent neural networks.

## Goals

- Modify the network structure at runtime.
- Add or remove neurons and synapses on the fly.
- Provide a simple and well-documented API.

## Quick Example

```rust
use aei_framework::{Activation, Network};

fn main() {
    let mut net = Network::new();
    let input = net.add_neuron(); // Uses the default identity activation
    let hidden = net.add_neuron_with_activation(Activation::ReLU);
    let output = net.add_neuron_with_activation(Activation::Sigmoid);
    net.add_synapse(input, hidden, 1.0);
    net.add_synapse(hidden, output, 1.0);
    net.propagate(input, -0.5);
    println!("Value of output neuron: {:?}", net.value(output));
}
```

## Activation Functions

Neurons support several activation functions:

- `Identity`
- `Sigmoid`
- `ReLU`
- `Tanh`

By default, neurons use `Identity`. To create a neuron with a specific
activation, either instantiate a [`Neuron`] directly or use
`Network::add_neuron_with_activation`:

```rust
use aei_framework::{Activation, Neuron};

let neuron = Neuron::new(1, Activation::Tanh);
```

## Propagation Flow

`Network::propagate` performs a forward pass through the network in four
ordered steps:

1. **Reset** – all neuron values are set to `0.0`, ensuring that consecutive
   propagations do not interfere with each other.
2. **Source activation** – the input value is passed through the activation
   function of the source neuron.
3. **Weighted propagation** – each synapse adds `from_value * weight` to the
   target neuron's pending sum.
4. **Activation** – after all sums have been collected, every neuron applies
   its activation function to produce the new output.

This sequence guarantees that each neuron is activated exactly once per
propagation and that previous runs do not leak into the next one.

## Development

```bash
cargo build    # Build the project
cargo test     # Run the test suite
```

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) before submitting changes.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for the list of changes.

## License

Distributed under the Mozilla Public License 2.0. See [LICENSE](LICENSE) for more information.
