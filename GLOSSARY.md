# Glossary

Definitions of domain-specific and technical terms used across the AEI Framework.

### AEI Framework (AEIF)
The Autonomous & Evolutive Intelligence Framework, a Rust library for building dynamic multi-agent neural networks. See [README.md](README.md).

### Domain-Driven Design (DDD)
An approach to software development that models complex domains in terms of bounded contexts and ubiquitous language.

### Command
An intent to change state in the system. Commands are handled by dedicated command handlers and produce events when successful. See [src/application/commands.rs](src/application/commands.rs).

### Query
A read-only request for information handled separately from commands. See [src/application/queries.rs](src/application/queries.rs).

### Event
An immutable record describing a state change that occurred as a result of handling a command. Events are persisted to the event store and can be replayed to reconstruct state. See [src/domain/events.rs](src/domain/events.rs).

### Event Store
Persistent storage responsible for appending and loading domain events. Implementations live under [src/infrastructure/event_store.rs](src/infrastructure/event_store.rs).

### Event Sourcing
Architectural pattern in which state is derived from a log of events rather than being stored directly. The AEI Framework rebuilds aggregates by replaying events from the store.

### Command Handler
Component that validates and executes a command, emitting one or more events. Examples include [AddRandomNeuronHandler](src/application/add_random_neuron.rs) and [RemoveRandomNeuronHandler](src/application/remove_random_neuron.rs).

### Query Handler
Component that serves a query by reading from a projection or read model. See [src/application/query_handler.rs](src/application/query_handler.rs).

### Projection
Process that transforms events into a read model suited for queries. Projections reside under [src/infrastructure/projection](src/infrastructure/projection).

### Read Model
State optimized for serving queries, maintained by projections derived from the event stream.

### Aggregate
A domain object that enforces invariants and rebuilds its state by applying events, such as [Network](src/domain/network.rs).

### Neuron
Basic processing unit in the network. Defined in [src/domain/neuron.rs](src/domain/neuron.rs).

### Synapse
Connection between neurons that carries signals. Defined in [src/domain/synapse.rs](src/domain/synapse.rs).

### AddRandomNeuronCommand
Command that introduces a new neuron into the network at a random location. Implemented in [src/application/add_random_neuron.rs](src/application/add_random_neuron.rs).

### RemoveRandomNeuronCommand
Command that removes a randomly selected neuron from the network. Implemented in [src/application/remove_random_neuron.rs](src/application/remove_random_neuron.rs).

### AddRandomSynapseCommand
Command that creates a synapse between two randomly chosen neurons. Implemented in [src/application/add_random_synapse.rs](src/application/add_random_synapse.rs).

### RemoveRandomSynapseCommand
Command requesting the removal of a random synapse from the network. Implemented in [src/application/add_random_synapse.rs](src/application/remove_random_synapse.rs).
