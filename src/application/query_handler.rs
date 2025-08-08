//! Handles read-side queries against the current state.

use crate::application::Query;
use crate::domain::{Activation, Neuron, Synapse};
use crate::infrastructure::projection::NetworkProjection;
use uuid::Uuid;

/// Result returned by the [`QueryHandler`].
pub enum QueryResult<'a> {
    /// Single neuron lookup.
    Neuron(Option<&'a Neuron>),
    /// Listing of all neurons.
    Neurons(Vec<&'a Neuron>),
    /// Listing of all synapses.
    Synapses(Vec<&'a Synapse>),
    /// Single synapse lookup.
    Synapse(Option<&'a Synapse>),
    /// Activation lookup.
    Activation(Option<Activation>),
}

/// Provides read-only access to the network state.
pub struct QueryHandler<'a> {
    projection: &'a NetworkProjection,
}

impl<'a> QueryHandler<'a> {
    /// Creates a new query handler from the given projection reference.
    pub fn new(projection: &'a NetworkProjection) -> Self {
        Self { projection }
    }

    /// Executes a query and returns a projection of the state.
    pub fn handle(&self, query: Query) -> QueryResult<'a> {
        match query {
            Query::GetNeuron { id } => QueryResult::Neuron(self.projection.neuron(id)),
            Query::ListNeurons => QueryResult::Neurons(self.projection.neurons()),
            Query::ListSynapses => QueryResult::Synapses(self.projection.synapses()),
            Query::GetSynapse { id } => QueryResult::Synapse(self.projection.synapse(id)),
            Query::GetNeuronActivation { id } => {
                QueryResult::Activation(self.projection.activation(id))
            }
        }
    }

    /// Convenience method to fetch a neuron directly.
    #[must_use]
    pub fn neuron(&self, id: Uuid) -> Option<&'a Neuron> {
        self.projection.neuron(id)
    }

    /// Convenience method to fetch a synapse directly.
    #[must_use]
    pub fn synapse(&self, id: Uuid) -> Option<&'a Synapse> {
        self.projection.synapse(id)
    }

    /// Convenience method to fetch a neuron's activation directly.
    #[must_use]
    pub fn activation(&self, id: Uuid) -> Option<Activation> {
        self.projection.activation(id)
    }
}
