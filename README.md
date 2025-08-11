# AEI Framework (AEIF)

[![Build](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/owner/aei-framework/actions)
[![License: MPL-2.0](https://img.shields.io/badge/license-MPL%202.0-blue)](LICENSE)

AEI Framework is an open source Rust framework for building dynamic, modular, scalable, embeddable, and multi-agent neural networks.

## Goals

- Modify the network structure at runtime.
- Add or remove neurons and synapses on the fly.
- Provide a simple and well-documented API.

## Event Storage

`JsonlEventStore<T>` persists any event implementing `Serialize` and `DeserializeOwned` as JSON Lines. The `FileEventStore` and `FileMemoryEventStore` type aliases offer ready-made stores for domain and memory events.

## Random Neuron Addition

Grow the network by issuing a command handled through the event-sourced
infrastructure:

```rust
use aei_framework::{
    AddRandomNeuronCommand, AddRandomNeuronHandler, FileEventStore,
};
use rand::thread_rng;
use std::path::PathBuf;

let store = FileEventStore::new(PathBuf::from("events.log"));
let mut handler = AddRandomNeuronHandler::new(store, thread_rng()).unwrap();
let new_neuron_id = handler.handle(AddRandomNeuronCommand).unwrap();
println!("Neuron added: {new_neuron_id}");
```

## Random Neuron Removal

Remove an internal neuron via a dedicated command handler:

```rust
use aei_framework::{
    RemoveRandomNeuronCommand, RemoveRandomNeuronHandler, FileEventStore,
};
use rand::thread_rng;
use std::path::PathBuf;

let store = FileEventStore::new(PathBuf::from("events.log"));
let mut handler = RemoveRandomNeuronHandler::new(store, thread_rng()).unwrap();
if let Ok(removed_id) = handler.handle(RemoveRandomNeuronCommand) {
    println!("Removed neuron: {removed_id}");
}
```

## Random Neuron Activation Mutation

Mutate the activation function of a randomly selected neuron:

```rust
use aei_framework::{
    MutateRandomNeuronActivationCommand, MutateRandomNeuronActivationHandler,
    FileEventStore,
};
use rand::thread_rng;
use std::path::PathBuf;

let store = FileEventStore::new(PathBuf::from("events.log"));
let mut handler =
    MutateRandomNeuronActivationHandler::new(store, thread_rng()).unwrap();
if let Ok(mutated_id) = handler.handle(MutateRandomNeuronActivationCommand { exclude_io: false }) {
    println!("Mutated neuron: {mutated_id}");
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

## Random Synapse Removal

Delete a randomly selected synapse and record the action as an event:

```rust
use aei_framework::{
    RemoveRandomSynapseCommand, RemoveRandomSynapseHandler, FileEventStore,
};
use rand::thread_rng;
use std::path::PathBuf;

let store = FileEventStore::new(PathBuf::from("events.log"));
let mut handler = RemoveRandomSynapseHandler::new(store, thread_rng()).unwrap();
if let Ok(removed_id) = handler.handle(RemoveRandomSynapseCommand) {
    println!("Removed synapse: {removed_id}");
}
```

## Random Synapse Weight Mutation

Adjust a synapse's weight by adding Gaussian noise:

```rust
use aei_framework::{
    MutateRandomSynapseWeightCommand, MutateRandomSynapseWeightHandler, FileEventStore,
};
use rand::thread_rng;
use std::path::PathBuf;

let store = FileEventStore::new(PathBuf::from("events.log"));
let mut handler = MutateRandomSynapseWeightHandler::new(store, thread_rng()).unwrap();
if let Ok(synapse_id) =
    handler.handle(MutateRandomSynapseWeightCommand { std_dev: 0.1 })
{
    println!("Mutated synapse: {synapse_id}");
}
```

## Adaptive Memory

Record and query past experiences through a bounded, scored buffer:

```rust
use aei_framework::{
    application::memory::{
        AddMemoryEntryCommand, AddMemoryEntryHandler, MemoryQuery, MemoryQueryHandler,
    },
    infrastructure::{projection::MemoryProjection, FileMemoryEventStore},
};
use serde_json::json;
use std::path::PathBuf;

let store = FileMemoryEventStore::new(PathBuf::from("memory.log"));
let mut handler = AddMemoryEntryHandler::new(store, 50).unwrap();
handler
    .handle(AddMemoryEntryCommand {
        event_type: "interaction".into(),
        payload: json!({"msg": "hello"}),
        score: 0.7,
    })
    .unwrap();
let mut store = handler.store;
let events = store.load().unwrap();
let projection = MemoryProjection::from_events(50, &events);
let qh = MemoryQueryHandler::new(&projection);
let _entries = qh.handle(MemoryQuery::GetMemoryState);
```

## Logging

The framework emits informational messages using the [`log`](https://docs.rs/log) crate. To see these logs, initialize a logger implementation such as [`env_logger`](https://docs.rs/env_logger) in your application:

```rust
env_logger::init();
```

With a logger configured, progress from command handlers will be reported at the `info` level.

## Example

A minimal command workflow is available in [examples/basic.rs](examples/basic.rs):

```bash
 cargo run --example basic
```

It adds neurons, connects them with a synapse, and queries the read model.

## Project Structure

```
src/
  domain/      # primitives, aggregates and domain events
  application/ # commands, queries and handlers
  infrastructure/
    event_store.rs  # event store implementations
    projection/     # read-model projections
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
separate **queries** handled by projections located under
`infrastructure/projection`. This separation keeps the write path append-only
and enables full traceability of the network's evolution.

**Command → Event → Apply → Projection**

1. A command expresses intent to mutate state.
2. The handler emits and persists a domain event.
3. The aggregate applies the event to update its state.
4. Projections consume the event to refresh read models.

## Documentation

English and French guides are available under [docs/en](docs/en/README.md) and [docs/fr](docs/fr/README.md).
A glossary of domain-specific and technical terms is provided in [GLOSSARY.md](GLOSSARY.md).

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
