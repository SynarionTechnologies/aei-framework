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
use aei_framework::Network;

fn main() {
    let mut net = Network::new();
    let a = net.add_neuron();
    let b = net.add_neuron();
    net.add_synapse(a, b, 0.5);
    net.propagate(a, 2.0);
    println!("Value of neuron b: {:?}", net.value(b));
}
```

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
