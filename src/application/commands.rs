//! Commands describing intent to change the domain state.

use crate::domain::Activation;
use uuid::Uuid;

/// Write-side operations handled by the [`CommandHandler`].
#[derive(Debug, Clone)]
pub enum Command {
    /// Create a neuron with the specified identifier and activation.
    CreateNeuron {
        /// Identifier of the neuron to create.
        id: Uuid,
        /// Activation function assigned to the neuron.
        activation: Activation,
    },
    /// Remove a neuron by its identifier.
    RemoveNeuron {
        /// Identifier of the neuron to remove.
        id: Uuid,
    },
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
