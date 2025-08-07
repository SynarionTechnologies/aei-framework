//! Domain events representing state changes in the network.
//!
//! Events are persisted in an append-only log and can be replayed to
//! reconstruct the state of the system.

use super::Activation;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Business events emitted by command handlers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    /// A neuron was added to the network with a random activation.
    RandomNeuronAdded(RandomNeuronAdded),
    /// A neuron was removed from the network.
    RandomNeuronRemoved(RandomNeuronRemoved),
    /// A synapse connecting two neurons was created.
    SynapseCreated {
        id: Uuid,
        from: Uuid,
        to: Uuid,
        weight: f64,
    },
    /// A synapse was removed from the network.
    SynapseRemoved { id: Uuid },
    /// A synapse between two randomly selected neurons was added.
    RandomSynapseAdded(RandomSynapseAdded),
}

/// Event emitted when a random neuron is added to the network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomNeuronAdded {
    /// Identifier of the created neuron.
    pub neuron_id: Uuid,
    /// Activation assigned to the neuron.
    pub activation: Activation,
}

/// Event emitted when a random neuron is removed from the network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomNeuronRemoved {
    /// Identifier of the removed neuron.
    pub neuron_id: Uuid,
}

/// Event emitted when a random synapse is created between two existing neurons.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomSynapseAdded {
    /// Identifier of the new synapse.
    pub synapse_id: Uuid,
    /// Source neuron of the synapse.
    pub from: Uuid,
    /// Target neuron of the synapse.
    pub to: Uuid,
    /// Weight associated with the synapse.
    pub weight: f64,
}
