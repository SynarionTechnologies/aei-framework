# Glossary

Definitions of domain-specific and technical terms used across the AEI Framework.

### AEI Framework (AEIF)
The Autonomous & Evolutive Intelligence Framework, a Rust library for building dynamic multi-agent neural networks. See [README.md](README.md).

### Domain-Driven Design (DDD)
An approach to software development that models complex domains in terms of bounded contexts and ubiquitous language.

### MemoryStore
Abstraction for CRUD operations on memory items.

### MemoryIndex
Component providing vector-based search over memory items.

### RetentionPolicy
Strategy determining whether memory items are kept, archived, or deleted.

### Compactor
Reduces storage usage by merging or removing memory items.

### Scheduler
Plans recurring tasks executed on manual ticks.

### EventBus
In-process publish/subscribe mechanism for events.

### Command
An intent to change state in the system. Commands are handled by dedicated command handlers and produce events when successful. See [src/application/commands.rs](src/application/commands.rs).

### Query
A read-only request for information handled separately from commands. See [src/application/queries.rs](src/application/queries.rs).

### Event
An immutable record describing a state change that occurred as a result of handling a command. Events are persisted to the event store and can be replayed to reconstruct state. See [src/domain/events.rs](src/domain/events.rs).

### Event Type
Label categorizing a memory entry, enabling queries for specific kinds of experiences.

### Event Store
Persistent storage responsible for appending and loading domain events. Implementations live under [src/infrastructure/event_store.rs](src/infrastructure/event_store.rs).

### JsonlEventStore
Generic event store persisting events as JSON Lines files. See [src/infrastructure/jsonl_event_store.rs](src/infrastructure/jsonl_event_store.rs).

### Event Sourcing
Architectural pattern in which state is derived from a log of events rather than being stored directly. The AEI Framework rebuilds aggregates by replaying events from the store.

### Command Handler
Component that validates and executes a command, emitting one or more events. Examples include [AddRandomNeuronHandler](src/application/add_random_neuron.rs) and [RemoveRandomNeuronHandler](src/application/remove_random_neuron.rs).

### NetworkHandlerBase
Shared structure bundling an event store, a hydrated [`Network`](src/domain/network.rs), and a random number generator for command handlers operating on networks.

### MemoryHandlerBase
Shared structure bundling a memory event store and hydrated [`AdaptiveMemory`](src/domain/memory) with helpers to persist events and prune entries.

### Query Handler
Component that serves a query by reading from a projection or read model. See [src/application/query_handler.rs](src/application/query_handler.rs).

### Projection
Process that transforms events into a read model suited for queries. Projections reside under [src/infrastructure/projection](src/infrastructure/projection).

### Read Model
State optimized for serving queries, maintained by projections derived from the event stream.

### Aggregate
A domain object that enforces invariants and rebuilds its state by applying events, such as [Network](src/domain/network.rs).

### Adaptive Memory
Bounded buffer storing scored experiences. Managed through event sourcing and queried via projections. See [src/domain/memory](src/domain/memory).

### Activation
The non-linear function a neuron applies to its input to produce an output.

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

### CreateNeuron Command
Command that inserts a neuron with a specific identifier and activation into the network. Handled by `CommandHandler`.

### RemoveNeuron Command
Command that deletes a neuron by its identifier and prunes connected synapses. Handled by `CommandHandler`.

### MutateRandomSynapseWeightCommand
Command that mutates the weight of a randomly selected synapse by adding Gaussian noise. Implemented in [src/application/mutate_random_synapse_weight.rs](src/application/mutate_random_synapse_weight.rs).

### SynapseWeightMutated
Domain event recording a change in a synapse's weight. Emitted by `MutateRandomSynapseWeightHandler`.

### SetSynapseWeightCommand
Command that assigns a specific weight to an existing synapse. Implemented in [src/application/set_synapse_weight.rs](src/application/set_synapse_weight.rs).

### SynapseWeightSet
Domain event recording an explicit update of a synapse's weight. Emitted by `SetSynapseWeightHandler`.

### NeuronAdded
Domain event emitted when a neuron is added to the network. Result of `CreateNeuron`.

### NeuronRemoved
Domain event emitted when a neuron is removed from the network. Result of `RemoveNeuron`.

### Curiosity Score
Metric representing the exploratory potential of a neuron or synapse. Recomputed via `RecalculateCuriosityScoreCommand` and stored through `CuriosityScoreUpdated` events.

### CuriosityScoreProjection
Read model mapping identifiers to curiosity scores. See [src/infrastructure/projection/curiosity.rs](src/infrastructure/projection/curiosity.rs).

### GetCuriosityScore Query
Query retrieving a curiosity score for a neuron or synapse via [`QueryHandler`](src/application/query_handler.rs).
