//! Core of the AEI Framework, a Rust framework for building dynamic and
//! modular neural networks.

pub mod api;
pub mod application;
pub mod core;
pub mod domain;
pub mod infrastructure;

pub use api::{Network, NetworkError};
pub use application::{
    AddRandomNeuronCommand, AddRandomNeuronError, AddRandomNeuronHandler, AddRandomSynapseCommand,
    AddRandomSynapseError, AddRandomSynapseHandler, Command, CommandHandler, Query, QueryHandler,
    QueryResult, RemoveRandomNeuronCommand, RemoveRandomNeuronError, RemoveRandomNeuronHandler,
};
pub use core::{Activation, Neuron, Synapse};
pub use domain::{
    Event, Network as DomainNetwork, RandomNeuronAdded, RandomNeuronRemoved, RandomSynapseAdded,
};
pub use infrastructure::{EventStore, FileEventStore};
