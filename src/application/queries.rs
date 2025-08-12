//! Read-side queries executed against projections of the domain state.

use uuid::Uuid;

/// Query operations handled by the [`QueryHandler`].
#[derive(Debug, Clone)]
pub enum Query {
    /// Fetch a neuron by identifier.
    GetNeuron { id: Uuid },
    /// Return all known neurons.
    ListNeurons,
    /// Return all known synapses.
    ListSynapses,
    /// Fetch a synapse by identifier.
    GetSynapse { id: Uuid },
    /// Fetch the activation function of a neuron by identifier.
    GetNeuronActivation { id: Uuid },
    /// Fetch the curiosity score for a neuron or synapse by identifier.
    GetCuriosityScore { id: Uuid },
}
