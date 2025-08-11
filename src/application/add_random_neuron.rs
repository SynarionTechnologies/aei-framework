//! Command and handler for adding a neuron with random properties.

use rand::{seq::SliceRandom, Rng};
use uuid::Uuid;

use super::NetworkHandlerBase;
use crate::domain::{Activation, Event, RandomNeuronAdded};
use crate::infrastructure::EventStore;

/// Command requesting the addition of a randomly configured neuron.
#[derive(Debug, Clone, Copy)]
pub struct AddRandomNeuronCommand;

/// Possible errors when adding a random neuron.
#[derive(Debug, Clone, PartialEq)]
pub enum AddRandomNeuronError {
    /// Persisting the event failed.
    StorageError,
}

/// Handles [`AddRandomNeuronCommand`], emitting events and updating state.
pub struct AddRandomNeuronHandler<S: EventStore, R: Rng> {
    /// Shared handler state including store, network and RNG.
    pub base: NetworkHandlerBase<S, R>,
}

impl<S: EventStore, R: Rng> AddRandomNeuronHandler<S, R> {
    /// Loads events from the store to initialize the handler.
    pub fn new(store: S, rng: R) -> Result<Self, S::Error> {
        Ok(Self {
            base: NetworkHandlerBase::new(store, rng)?,
        })
    }

    /// Handles the command and returns the identifier of the created neuron.
    pub fn handle(&mut self, _cmd: AddRandomNeuronCommand) -> Result<Uuid, AddRandomNeuronError> {
        let activations = [
            Activation::Identity,
            Activation::Sigmoid,
            Activation::ReLU,
            Activation::Tanh,
        ];
        let base = &mut self.base;
        let activation = *activations
            .choose(&mut base.rng)
            .expect("activation list is non-empty");
        let neuron_id = Uuid::new_v4();
        let event = Event::RandomNeuronAdded(RandomNeuronAdded {
            neuron_id,
            activation,
        });
        base.store
            .append(&event)
            .map_err(|_| AddRandomNeuronError::StorageError)?;
        base.network.apply(&event);

        // Attach at least one random synapse if other neurons exist.
        let mut others: Vec<Uuid> = base
            .network
            .neurons
            .keys()
            .copied()
            .filter(|id| *id != neuron_id)
            .collect();
        if !others.is_empty() {
            let count = base.rng.gen_range(1..=others.len());
            others.shuffle(&mut base.rng);
            for target in others.into_iter().take(count) {
                let weight = base.rng.gen_range(-1.0..=1.0);
                let syn_id = Uuid::new_v4();
                let event = if base.rng.gen_bool(0.5) {
                    Event::SynapseCreated {
                        id: syn_id,
                        from: target,
                        to: neuron_id,
                        weight,
                    }
                } else {
                    Event::SynapseCreated {
                        id: syn_id,
                        from: neuron_id,
                        to: target,
                        weight,
                    }
                };
                base.store
                    .append(&event)
                    .map_err(|_| AddRandomNeuronError::StorageError)?;
                base.network.apply(&event);
            }
        }

        Ok(neuron_id)
    }
}
