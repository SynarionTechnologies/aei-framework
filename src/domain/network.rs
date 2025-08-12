//! Event-sourced representation of a neural network.
//!
//! The [`Network`] aggregate stores neurons and synapses and evolves solely
//! through the application of [`Event`]s.

use std::collections::HashMap;

use super::events::{
    CuriosityScoreUpdated, Event, NeuronActivationMutated, NeuronAdded, NeuronRemoved,
    RandomNeuronAdded, RandomNeuronRemoved, RandomSynapseAdded, RandomSynapseRemoved,
    SynapseWeightMutated,
};
use super::{Neuron, Synapse};
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
            Event::RandomNeuronAdded(e) => {
                self.apply_random_neuron_added(e);
            }
            Event::RandomNeuronRemoved(e) => {
                self.apply_random_neuron_removed(e);
            }
            Event::NeuronAdded(e) => {
                self.apply_neuron_added(e);
            }
            Event::NeuronRemoved(e) => {
                self.apply_neuron_removed(e);
            }
            Event::SynapseCreated {
                id,
                from,
                to,
                weight,
            } => {
                if self.neurons.contains_key(from) && self.neurons.contains_key(to) {
                    self.synapses
                        .insert(*id, Synapse::with_id(*id, *from, *to, *weight));
                }
            }
            Event::SynapseRemoved { id } => {
                self.synapses.remove(id);
            }
            Event::RandomSynapseAdded(e) => {
                self.apply_random_synapse_added(e);
            }
            Event::RandomSynapseRemoved(e) => {
                self.apply_random_synapse_removed(e);
            }
            Event::SynapseWeightMutated(e) => {
                self.apply_synapse_weight_mutated(e);
            }
            Event::NeuronActivationMutated(e) => {
                self.apply_neuron_activation_mutated(e);
            }
            Event::CuriosityScoreUpdated(e) => {
                self.apply_curiosity_score_updated(e);
            }
        }
    }

    /// Applies a [`RandomNeuronAdded`] event to the network state.
    fn apply_random_neuron_added(&mut self, event: &RandomNeuronAdded) {
        self.neurons.insert(
            event.neuron_id,
            Neuron::with_id(event.neuron_id, event.activation),
        );
    }

    /// Applies a [`RandomNeuronRemoved`] event to the network state.
    fn apply_random_neuron_removed(&mut self, event: &RandomNeuronRemoved) {
        self.neurons.remove(&event.neuron_id);
        self.synapses
            .retain(|_, s| s.from != event.neuron_id && s.to != event.neuron_id);
    }

    /// Applies a [`NeuronAdded`] event to the network state.
    fn apply_neuron_added(&mut self, event: &NeuronAdded) {
        self.neurons.insert(
            event.neuron_id,
            Neuron::with_id(event.neuron_id, event.activation),
        );
    }

    /// Applies a [`NeuronRemoved`] event to the network state.
    fn apply_neuron_removed(&mut self, event: &NeuronRemoved) {
        self.neurons.remove(&event.neuron_id);
        self.synapses
            .retain(|_, s| s.from != event.neuron_id && s.to != event.neuron_id);
    }

    /// Applies a [`RandomSynapseAdded`] event to the network state.
    fn apply_random_synapse_added(&mut self, event: &RandomSynapseAdded) {
        if self.neurons.contains_key(&event.from)
            && self.neurons.contains_key(&event.to)
            && event.from != event.to
            && !self
                .synapses
                .values()
                .any(|s| s.from == event.from && s.to == event.to)
        {
            self.synapses.insert(
                event.synapse_id,
                Synapse::with_id(event.synapse_id, event.from, event.to, event.weight),
            );
        }
    }

    /// Applies a [`RandomSynapseRemoved`] event to the network state.
    fn apply_random_synapse_removed(&mut self, event: &RandomSynapseRemoved) {
        self.synapses.remove(&event.synapse_id);
    }

    /// Applies a [`SynapseWeightMutated`] event to the network state.
    fn apply_synapse_weight_mutated(&mut self, event: &SynapseWeightMutated) {
        if let Some(synapse) = self.synapses.get_mut(&event.synapse_id) {
            synapse.weight = event.new_weight;
        }
    }

    /// Applies a [`NeuronActivationMutated`] event to the network state.
    fn apply_neuron_activation_mutated(&mut self, event: &NeuronActivationMutated) {
        if let Some(neuron) = self.neurons.get_mut(&event.neuron_id) {
            neuron.activation = event.new_activation;
        }
    }

    /// Applies a [`CuriosityScoreUpdated`] event to the network state.
    fn apply_curiosity_score_updated(&mut self, event: &CuriosityScoreUpdated) {
        if let Some(neuron) = self.neurons.get_mut(&event.target_id) {
            neuron.curiosity_score = event.new_score;
        } else if let Some(synapse) = self.synapses.get_mut(&event.target_id) {
            synapse.curiosity_score = event.new_score;
        }
    }

    /// Convenience method to list all neurons.
    #[must_use]
    pub fn neurons(&self) -> Vec<&Neuron> {
        self.neurons.values().collect()
    }

    /// Convenience method to list all synapses.
    #[must_use]
    pub fn synapses(&self) -> Vec<&Synapse> {
        self.synapses.values().collect()
    }
}
