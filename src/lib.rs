//! Core of the AEI Framework, a Rust framework for building dynamic and
//! modular neural networks.

pub mod application;
pub mod domain;
pub mod infrastructure;

pub use application::memory::{
    AddMemoryEntryCommand, AddMemoryEntryError, AddMemoryEntryHandler, MemoryQuery,
    MemoryQueryHandler, MemoryQueryResult, PruneMemoryCommand, PruneMemoryError,
    PruneMemoryHandler, RemoveMemoryEntryCommand, RemoveMemoryEntryError, RemoveMemoryEntryHandler,
    UpdateMemoryScoreCommand, UpdateMemoryScoreError, UpdateMemoryScoreHandler,
};
pub use application::{
    AddRandomNeuronCommand, AddRandomNeuronError, AddRandomNeuronHandler, AddRandomSynapseCommand,
    AddRandomSynapseError, AddRandomSynapseHandler, Command, CommandHandler, CuriosityScope,
    MutateNeuronActivationError, MutateRandomNeuronActivationCommand,
    MutateRandomNeuronActivationHandler, MutateRandomSynapseWeightCommand,
    MutateRandomSynapseWeightError, MutateRandomSynapseWeightHandler, NetworkHandlerBase, Query,
    QueryHandler, QueryResult, RecalculateCuriosityScoreCommand, RecalculateCuriosityScoreHandler,
    RemoveRandomNeuronCommand, RemoveRandomNeuronError, RemoveRandomNeuronHandler,
    RemoveRandomSynapseCommand, RemoveRandomSynapseError, RemoveRandomSynapseHandler,
};
pub use domain::{
    Activation, AdaptiveMemory, CuriosityScoreUpdated, Event, MemoryEntry, MemoryEntryAdded,
    MemoryEntryRemoved, MemoryEvent, MemoryPruned, MemoryScoreUpdated, Network as DomainNetwork,
    Neuron, NeuronActivationMutated, RandomNeuronAdded, RandomNeuronRemoved, RandomSynapseAdded,
    RandomSynapseRemoved, Synapse, SynapseWeightMutated,
};
pub use infrastructure::{
    EventStore, FileEventStore, FileMemoryEventStore, JsonlEventStore, MemoryEventStore,
};
