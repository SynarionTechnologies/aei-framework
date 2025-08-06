//! Event-sourced representation of a neural network.
//!
//! The [`Network`] aggregate stores neurons and synapses and evolves solely
//! through the application of [`Event`]s.

use std::collections::HashMap;

use crate::core::{Neuron, Synapse};
use crate::events::Event;
use uuid::Uuid;

/// Aggregate root containing all neurons and synapses.
#[derive(Debug, Default, Clone)]
pub struct Network {
    /// Neurons indexed by their [`Uuid`].
    pub neurons: HashMap<Uuid, Neuron>,
    /// Synapses indexed by their [`Uuid`].
    pub synapses: HashMap<Uuid, Synapse>,
}

impl Network {
    /// Creates a network by replaying the provided events.
    #[must_use]
    pub fn hydrate(events: &[Event]) -> Self {
        let mut net = Self::default();
        for event in events {
            net.apply(event);
        }
        net
    }

    /// Applies a domain event to mutate the aggregate state.
    pub fn apply(&mut self, event: &Event) {
        match event {
            Event::NeuronAdded { id, activation } => {
                self.neurons
                    .insert(*id, Neuron::with_id(*id, *activation));
            }
            Event::NeuronRemoved { id } => {
                self.neurons.remove(id);
                self.synapses
                    .retain(|_, s| s.from != *id && s.to != *id);
            }
            Event::SynapseCreated {
                id,
                from,
                to,
                weight,
            } => {
                if self.neurons.contains_key(from) && self.neurons.contains_key(to) {
                    self.synapses.insert(
                        *id,
                        Synapse::with_id(*id, *from, *to, *weight),
                    );
                }
            }
            Event::SynapseRemoved { id } => {
                self.synapses.remove(id);
            }
        }
    }

    /// Convenience method to list all neurons.
    #[must_use]
    pub fn neurons(&self) -> Vec<&Neuron> {
        self.neurons.values().collect()
    }
}

