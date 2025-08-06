//! Commands describing intent to change the domain state.

use crate::core::Activation;
use uuid::Uuid;

/// Write-side operations handled by the [`CommandHandler`].
#[derive(Debug, Clone)]
pub enum Command {
    /// Add a neuron with a specific activation function.
    AddNeuron { id: Uuid, activation: Activation },
    /// Remove a neuron and all attached synapses.
    RemoveNeuron { id: Uuid },
    /// Create a synapse between two existing neurons.
    CreateSynapse {
        id: Uuid,
        from: Uuid,
        to: Uuid,
        weight: f64,
    },
    /// Delete a synapse by its identifier.
    RemoveSynapse { id: Uuid },
}

