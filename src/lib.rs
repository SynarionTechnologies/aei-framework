//! Core of the AEI Framework, a Rust framework for building dynamic and
//! modular neural networks.

pub mod application;
pub mod domain;
pub mod infrastructure;

pub use application::{
    AddRandomNeuronCommand, AddRandomNeuronError, AddRandomNeuronHandler, AddRandomSynapseCommand,
    AddRandomSynapseError, AddRandomSynapseHandler, Command, CommandHandler, Query, QueryHandler,
    QueryResult, RemoveRandomNeuronCommand, RemoveRandomNeuronError, RemoveRandomNeuronHandler,
};
pub use domain::{
    Activation, Event, Network as DomainNetwork, Neuron, RandomNeuronAdded, RandomNeuronRemoved,
    RandomSynapseAdded, Synapse,
};
pub use infrastructure::{EventStore, FileEventStore};
