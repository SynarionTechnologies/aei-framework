//! Handles read-side queries against the current state.

use crate::application::Query;
use crate::domain::{Activation, Neuron, Synapse};
use crate::infrastructure::projection::{CuriosityScoreProjection, NetworkProjection};
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
    /// Curiosity score lookup.
    CuriosityScore(Option<f64>),
}

/// Provides read-only access to the network state.
pub struct QueryHandler<'a> {
    network: &'a NetworkProjection,
    curiosity: Option<&'a CuriosityScoreProjection>,
}

impl<'a> QueryHandler<'a> {
    /// Creates a new query handler from the given network projection reference.
    pub fn new(projection: &'a NetworkProjection) -> Self {
        Self {
            network: projection,
            curiosity: None,
        }
    }

    /// Attaches a curiosity score projection for score lookups.
    ///
    /// # Arguments
    ///
    /// * `projection` - Projection containing curiosity scores.
    ///
    /// # Examples
    ///
    /// ```
    /// use aei_framework::application::{Query, QueryHandler, QueryResult};
    /// use aei_framework::domain::{Activation, CuriosityScoreUpdated, Event, RandomNeuronAdded};
    /// use aei_framework::infrastructure::projection::{CuriosityScoreProjection, NetworkProjection};
    /// use uuid::Uuid;
    ///
    /// let id = Uuid::new_v4();
    /// let events = vec![
    ///     Event::RandomNeuronAdded(RandomNeuronAdded { neuron_id: id, activation: Activation::ReLU }),
    ///     Event::CuriosityScoreUpdated(CuriosityScoreUpdated { target_id: id, old_score: 0.0, new_score: 0.42 }),
    /// ];
    /// let network = NetworkProjection::from_events(&events);
    /// let curiosity = CuriosityScoreProjection::from_events(&events);
    /// let handler = QueryHandler::new(&network).with_curiosity_projection(&curiosity);
    /// if let QueryResult::CuriosityScore(Some(score)) = handler.handle(Query::GetCuriosityScore { id }) {
    ///     assert_eq!(score, 0.42);
    /// }
    /// ```
    pub fn with_curiosity_projection(mut self, projection: &'a CuriosityScoreProjection) -> Self {
        self.curiosity = Some(projection);
        self
    }

    /// Executes a query and returns a projection of the state.
    pub fn handle(&self, query: Query) -> QueryResult<'a> {
        match query {
            Query::GetNeuron { id } => QueryResult::Neuron(self.network.neuron(id)),
            Query::ListNeurons => QueryResult::Neurons(self.network.neurons()),
            Query::ListSynapses => QueryResult::Synapses(self.network.synapses()),
            Query::GetSynapse { id } => QueryResult::Synapse(self.network.synapse(id)),
            Query::GetNeuronActivation { id } => {
                QueryResult::Activation(self.network.activation(id))
            }
            Query::GetCuriosityScore { id } => QueryResult::CuriosityScore(
                self.curiosity.and_then(|c| c.get(id)),
            ),
        }
    }

    /// Convenience method to fetch a neuron directly.
    #[must_use]
    pub fn neuron(&self, id: Uuid) -> Option<&'a Neuron> {
        self.network.neuron(id)
    }

    /// Convenience method to fetch a synapse directly.
    #[must_use]
    pub fn synapse(&self, id: Uuid) -> Option<&'a Synapse> {
        self.network.synapse(id)
    }

    /// Convenience method to fetch a neuron's activation directly.
    #[must_use]
    pub fn activation(&self, id: Uuid) -> Option<Activation> {
        self.network.activation(id)
    }

    /// Convenience method to fetch a curiosity score directly.
    ///
    /// # Arguments
    ///
    /// * `id` - Identifier of the target neuron or synapse.
    ///
    /// # Returns
    ///
    /// * `Option<f64>` - The curiosity score if present.
    ///
    /// # Examples
    ///
    /// ```
    /// use aei_framework::application::QueryHandler;
    /// use aei_framework::domain::{Activation, CuriosityScoreUpdated, Event, RandomNeuronAdded};
    /// use aei_framework::infrastructure::projection::{CuriosityScoreProjection, NetworkProjection};
    /// use uuid::Uuid;
    ///
    /// let id = Uuid::new_v4();
    /// let events = vec![
    ///     Event::RandomNeuronAdded(RandomNeuronAdded { neuron_id: id, activation: Activation::ReLU }),
    ///     Event::CuriosityScoreUpdated(CuriosityScoreUpdated { target_id: id, old_score: 0.0, new_score: 0.5 }),
    /// ];
    /// let network = NetworkProjection::from_events(&events);
    /// let curiosity = CuriosityScoreProjection::from_events(&events);
    /// let handler = QueryHandler::new(&network).with_curiosity_projection(&curiosity);
    /// assert_eq!(handler.curiosity_score(id), Some(0.5));
    /// ```
    #[must_use]
    pub fn curiosity_score(&self, id: Uuid) -> Option<f64> {
        self.curiosity.and_then(|c| c.get(id))
    }
}
