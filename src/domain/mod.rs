//! Domain aggregates modelling the state of the network.

mod events;
mod network;

pub use events::{Event, RandomNeuronAdded, RandomNeuronRemoved, RandomSynapseAdded};
pub use network::Network;
