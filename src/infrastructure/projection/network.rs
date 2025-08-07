//! Read model reflecting the current network state via applied events.

use uuid::Uuid;

use crate::domain::{Event, Network, Neuron, Synapse};

/// In-memory projection of the [`Network`] aggregate.
#[derive(Debug, Default)]
pub struct NetworkProjection {
    network: Network,
}

impl NetworkProjection {
    /// Builds the projection by replaying the provided events.
    #[must_use]
    pub fn from_events(events: &[Event]) -> Self {
        Self {
            network: Network::hydrate(events),
        }
    }

    /// Applies a new event to update the projection.
    pub fn apply(&mut self, event: &Event) {
        self.network.apply(event);
    }

    /// Fetches a neuron by its identifier.
    pub fn neuron(&self, id: Uuid) -> Option<&Neuron> {
        self.network.neurons.get(&id)
    }

    /// Returns all neurons contained in the projection.
    #[must_use]
    pub fn neurons(&self) -> Vec<&Neuron> {
        self.network.neurons()
    }

    /// Returns all synapses contained in the projection.
    #[must_use]
    pub fn synapses(&self) -> Vec<&Synapse> {
        self.network.synapses()
    }

    /// Fetches a synapse by its identifier.
    pub fn synapse(&self, id: Uuid) -> Option<&Synapse> {
        self.network.synapses.get(&id)
    }
}
