//! Domain aggregates modelling the state of the network.

mod activation;
mod events;
mod memory;
mod network;
mod neuron;
mod synapse;

pub use activation::Activation;
pub use events::{
    CuriosityScoreUpdated, Event, NeuronActivationMutated, NeuronAdded, NeuronRemoved,
    RandomNeuronAdded, RandomNeuronRemoved, RandomSynapseAdded, RandomSynapseRemoved,
    SynapseWeightMutated,
};
pub use memory::{
    AdaptiveMemory, MemoryEntry, MemoryEntryAdded, MemoryEntryRemoved, MemoryEvent, MemoryPruned,
    MemoryScoreUpdated,
};
pub use network::Network;
pub use neuron::Neuron;
pub use synapse::Synapse;
