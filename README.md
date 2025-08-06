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
    net.add_synapse(input, hidden, 1.0).unwrap();
    net.add_synapse(hidden, output, 1.0).unwrap();
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
net.add_synapse(input, hidden, 0.5).unwrap();
net.add_synapse(hidden, output, 1.0).unwrap();

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
net.add_synapse(a, out, 1.0).unwrap();
net.add_synapse(b, out, 1.0).unwrap();

net.set_inputs(&[("a", 1.0), ("b", 0.0)]);
net.propagate_inputs().unwrap();
let result = net.get_outputs();
println!("Result: {:?}", result.get("out"));
```

## Random Neuron Addition

Grow the network by inserting a neuron with a random activation and automatic
connections:

```rust
use aei_framework::Network;

let mut net = Network::new();
let new_neuron_id = net.add_random_neuron();
println!("Added neuron: {new_neuron_id}");
```

## Random Neuron Removal

Shrink the network by deleting a random internal neuron along with all its
connections:

```rust
use aei_framework::Network;

let mut net = Network::new();
// ... initialize the network ...
if let Some(removed_id) = net.remove_random_neuron() {
    println!("Removed neuron: {removed_id}");
}
```

## Random Synapse Addition

Create a synapse between two randomly chosen neurons using the event-sourced
handler:

```rust
use aei_framework::{
    application::{AddRandomSynapseCommand, AddRandomSynapseHandler},
    infrastructure::FileEventStore,
};
use std::path::PathBuf;

let store = FileEventStore::new(PathBuf::from("events.log"));
let mut handler = AddRandomSynapseHandler::new(store, rand::thread_rng()).unwrap();
let synapse_id = handler.handle(AddRandomSynapseCommand).unwrap();
println!("Created synapse: {synapse_id}");
```

## Serialization

Persist networks to disk and load them back later using JSON:

```rust
use aei_framework::{Activation, Network};
use std::path::Path;

let mut net = Network::new();
let a = net.add_input_neuron("a", Activation::Identity);
let b = net.add_output_neuron("b", Activation::Identity);
net.add_synapse(a, b, 1.0).unwrap();

let path = Path::new("network.json");
net.save_json(path).unwrap();
let restored = Network::load_json(path).unwrap();
```

## Identifiers

Every neuron and synapse receives a random [`Uuid`](https://docs.rs/uuid) when
created. These globally unique identifiers improve serialization and allow
merging networks without collisions. Constructors such as
`Neuron::with_id` and `Network::add_neuron_with_id` let you supply explicit
identifiers if needed:

```rust
use aei_framework::{Activation, Neuron};
use uuid::Uuid;

let id = Uuid::new_v4();
let neuron = Neuron::with_id(id, Activation::Sigmoid);
println!("Neuron {id} has value {}", neuron.value);
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

net.add_synapse(i1, h1, 0.5).unwrap();
net.add_synapse(i1, h2, -0.5).unwrap();
net.add_synapse(i2, h1, -0.5).unwrap();
net.add_synapse(i2, h2, 0.5).unwrap();
net.add_synapse(h1, o, 0.5).unwrap();
net.add_synapse(h2, o, 0.5).unwrap();

let dataset = [
    (vec![0.0, 0.0], vec![0.0]),
    (vec![0.0, 1.0], vec![1.0]),
    (vec![1.0, 0.0], vec![1.0]),
    (vec![1.0, 1.0], vec![0.0]),
];

net.train(&dataset, 10_000, 0.5).unwrap();

let output = net.predict(&[0.0, 1.0]).unwrap()[0];
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

let neuron = Neuron::new(Activation::Tanh);
println!("Neuron id: {}", neuron.id);
```

## Example: Multi-Activation Network

```rust
use aei_framework::{Activation, Network};

let mut net = Network::new();
let n1 = net.add_neuron_with_activation(Activation::Sigmoid);
let n2 = net.add_neuron_with_activation(Activation::ReLU);
net.add_synapse(n1, n2, 2.0).unwrap();

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

## Logging

The framework emits informational messages using the [`log`](https://docs.rs/log) crate. To see these logs, initialize a logger implementation such as [`env_logger`](https://docs.rs/env_logger) in your application:

```rust
env_logger::init();
```

With a logger configured, progress from functions like `Network::train` will be reported at the `info` level.

## Project Structure

```
src/
  core/        # activation, neuron, synapse primitives
  api/         # network implementation and public API
  domain/      # event-sourced aggregates
  events/      # domain events
  commands.rs  # write-side commands
  queries.rs   # read-side queries
  application/ # command and query handlers
  infrastructure/ # adapters such as the event store
examples/
tests/
docs/
  en/
  fr/
```

## Architecture Overview

AEIF follows Domain-Driven Design with Event Sourcing and CQRS. State-changing
operations are expressed as **commands** which are turned into immutable
**events** and appended to an event log. Aggregates such as the `domain::Network`
replay these events to rebuild their state. Read operations are served through
separate **queries** handled by lightweight projections. This separation keeps
the write path append-only and enables full traceability of the network's
evolution.

## Documentation

English and French guides are available under [docs/en](docs/en/README.md) and [docs/fr](docs/fr/README.md).

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

- Neuron and synapse identifiers use `Uuid`. Networks serialized with older
  numeric identifiers are not supported.
- JSON persistence is available via `save_json` and `load_json`.
- Layered abstractions are planned but not implemented.
