//! Domain aggregates modelling the state of the network.

mod activation;
mod events;
mod network;
mod neuron;
mod synapse;

pub use activation::Activation;
pub use events::{
    Event, RandomNeuronAdded, RandomNeuronRemoved, RandomSynapseAdded, RandomSynapseRemoved,
    SynapseWeightMutated,
};
pub use network::Network;
pub use neuron::Neuron;
pub use synapse::Synapse;
