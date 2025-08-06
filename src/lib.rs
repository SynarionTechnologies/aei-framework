//! Core of the AEI Framework, a Rust framework for building dynamic and
//! modular neural networks.

pub mod api;
pub mod application;
pub mod commands;
pub mod core;
pub mod domain;
pub mod events;
pub mod infrastructure;
pub mod queries;

pub use api::{Network, NetworkError};
pub use application::{
    AddRandomNeuronCommand, AddRandomNeuronError, AddRandomNeuronHandler, AddRandomSynapseCommand,
    AddRandomSynapseError, AddRandomSynapseHandler, CommandHandler, QueryHandler, QueryResult,
    RemoveRandomNeuronCommand, RemoveRandomNeuronError, RemoveRandomNeuronHandler,
};
pub use core::{Activation, Neuron, Synapse};
pub use domain::Network as DomainNetwork;
pub use events::{Event, RandomNeuronAdded, RandomNeuronRemoved, RandomSynapseAdded};
pub use infrastructure::{EventStore, FileEventStore};
pub use queries::Query;
