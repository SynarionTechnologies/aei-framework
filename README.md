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

## Advanced Example

Create a small network with heterogeneous activations and observe the
propagation of a value through the chain of neurons.

```rust
use aei_framework::{Activation, Network};

let mut net = Network::new();
let input = net.add_neuron_with_activation(Activation::Identity);
let hidden = net.add_neuron_with_activation(Activation::ReLU);
let output = net.add_neuron_with_activation(Activation::Tanh);
net.add_synapse(input, hidden, 0.5);
net.add_synapse(hidden, output, 1.0);

// Propagate once from the input neuron.
net.propagate(input, 1.0);

println!("Hidden neuron value: {}", net.value(hidden).unwrap());
println!("Output neuron value: {}", net.value(output).unwrap());
```

## Named Inputs and Outputs

Explicitly assign neurons as inputs or outputs and interact with them by name:

```rust
use aei_framework::{Activation, Network};

let mut net = Network::new();
let a = net.add_input_neuron("a", Activation::Identity);
let b = net.add_input_neuron("b", Activation::Identity);
let out = net.add_output_neuron("out", Activation::Sigmoid);
net.add_synapse(a, out, 1.0);
net.add_synapse(b, out, 1.0);

net.set_inputs(&[("a", 1.0), ("b", 0.0)]);
net.propagate_inputs();
let result = net.get_outputs();
println!("Result: {:?}", result.get("out"));
```

## Learning XOR

Train a small network to approximate the XOR truth table using
backpropagation:

```rust
use aei_framework::{Activation, Network};

let mut net = Network::new();
let i1 = net.add_neuron_with_activation(Activation::Identity);
let i2 = net.add_neuron_with_activation(Activation::Identity);
let h1 = net.add_neuron_with_activation(Activation::Sigmoid);
let h2 = net.add_neuron_with_activation(Activation::Sigmoid);
let o = net.add_neuron_with_activation(Activation::Sigmoid);

net.add_synapse(i1, h1, 0.5);
net.add_synapse(i1, h2, -0.5);
net.add_synapse(i2, h1, -0.5);
net.add_synapse(i2, h2, 0.5);
net.add_synapse(h1, o, 0.5);
net.add_synapse(h2, o, 0.5);

let dataset = [
    (vec![0.0, 0.0], vec![0.0]),
    (vec![0.0, 1.0], vec![1.0]),
    (vec![1.0, 0.0], vec![1.0]),
    (vec![1.0, 1.0], vec![0.0]),
];

net.train(&dataset, 10_000, 0.5);

let output = net.predict(&[0.0, 1.0])[0];
println!("XOR(0,1) ≈ {output}");
```

## Step-by-Step Propagation

`Network::propagate` performs four ordered phases:

1. **Reset** – every neuron's value is cleared to `0.0`.
2. **Source activation** – the input value is passed through the source
   neuron's activation function.
3. **Weighted propagation** – synapses contribute `from_value * weight` to
   their targets.
4. **Activation** – each target neuron applies its activation function once all
   inputs have been received.

This deterministic sequence ensures that repeated calls do not accumulate
state and that activations are applied only after all inputs are processed.

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

## Example: Multi-Activation Network

```rust
use aei_framework::{activation::Activation, network::Network};

let mut net = Network::new();
let n1 = net.add_neuron_with_activation(Activation::Sigmoid);
let n2 = net.add_neuron_with_activation(Activation::ReLU);
net.add_synapse(n1, n2, 2.0);

net.propagate(n1, 1.0);
println!("Value of neuron {n1} (Sigmoid): {}", net.value(n1).unwrap());
println!("Value of neuron {n2} (ReLU): {}", net.value(n2).unwrap());
```

Using varied activations lets each neuron process data differently. Combining
smooth functions like `Sigmoid` with piecewise-linear ones like `ReLU` increases
the representational power of the network.

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

## Known Limitations

- Neuron identifiers are numeric and local to a network. Each neuron now also
  stores a `Uuid` but it is not yet used as the primary key.
- No persistence or serialization layer is currently provided.
- Layered abstractions and neuron removal are planned but not implemented.
