//! Handles read-side queries against the current state.

use crate::core::{Neuron, Synapse};
use crate::domain::Network;
use crate::queries::Query;
use uuid::Uuid;

/// Result returned by the [`QueryHandler`].
pub enum QueryResult<'a> {
    /// Single neuron lookup.
    Neuron(Option<&'a Neuron>),
    /// Listing of all neurons.
    Neurons(Vec<&'a Neuron>),
    /// Listing of all synapses.
    Synapses(Vec<&'a Synapse>),
}

/// Provides read-only access to the network state.
pub struct QueryHandler<'a> {
    network: &'a Network,
}

impl<'a> QueryHandler<'a> {
    /// Creates a new query handler from the given network reference.
    pub fn new(network: &'a Network) -> Self {
        Self { network }
    }

    /// Executes a query and returns a projection of the state.
    pub fn handle(&self, query: Query) -> QueryResult<'a> {
        match query {
            Query::GetNeuron { id } => QueryResult::Neuron(self.network.neurons.get(&id)),
            Query::ListNeurons => QueryResult::Neurons(self.network.neurons()),
            Query::ListSynapses => QueryResult::Synapses(self.network.synapses()),
        }
    }

    /// Convenience method to fetch a neuron directly.
    #[must_use]
    pub fn neuron(&self, id: Uuid) -> Option<&'a Neuron> {
        self.network.neurons.get(&id)
    }
}
