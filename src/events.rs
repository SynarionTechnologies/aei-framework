//! Domain events representing state changes in the network.
//!
//! Events are persisted in an append-only log and can be replayed to
//! reconstruct the state of the system.

use crate::core::Activation;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Business events emitted by command handlers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    /// A neuron was added to the network.
    NeuronAdded { id: Uuid, activation: Activation },
    /// A neuron was removed from the network.
    NeuronRemoved { id: Uuid },
    /// A synapse connecting two neurons was created.
    SynapseCreated {
        id: Uuid,
        from: Uuid,
        to: Uuid,
        weight: f64,
    },
    /// A synapse was removed from the network.
    SynapseRemoved { id: Uuid },
}

