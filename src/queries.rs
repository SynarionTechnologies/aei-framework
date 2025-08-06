//! Read-side queries executed against projections of the domain state.

use uuid::Uuid;

/// Query operations handled by the [`QueryHandler`].
#[derive(Debug, Clone)]
pub enum Query {
    /// Fetch a neuron by identifier.
    GetNeuron { id: Uuid },
    /// Return all known neurons.
    ListNeurons,
}

