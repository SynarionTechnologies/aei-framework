//! Command and handler for adding a neuron with random properties.

use rand::{seq::SliceRandom, Rng};
use uuid::Uuid;

use crate::core::Activation;
use crate::domain::{Event, Network, RandomNeuronAdded};
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
    /// Event store used for persistence.
    pub store: S,
    /// Current network state derived from applied events.
    pub network: Network,
    rng: R,
}

impl<S: EventStore, R: Rng> AddRandomNeuronHandler<S, R> {
    /// Loads events from the store to initialize the handler.
    pub fn new(mut store: S, rng: R) -> Result<Self, S::Error> {
        let events = store.load()?;
        let network = Network::hydrate(&events);
        Ok(Self {
            store,
            network,
            rng,
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
        let activation = *activations
            .choose(&mut self.rng)
            .expect("activation list is non-empty");
        let neuron_id = Uuid::new_v4();
        let event = Event::RandomNeuronAdded(RandomNeuronAdded {
            neuron_id,
            activation,
        });
        self.store
            .append(&event)
            .map_err(|_| AddRandomNeuronError::StorageError)?;
        self.network.apply(&event);

        // Attach at least one random synapse if other neurons exist.
        let mut others: Vec<Uuid> = self
            .network
            .neurons
            .keys()
            .copied()
            .filter(|id| *id != neuron_id)
            .collect();
        if !others.is_empty() {
            let count = self.rng.gen_range(1..=others.len());
            others.shuffle(&mut self.rng);
            for target in others.into_iter().take(count) {
                let weight = self.rng.gen_range(-1.0..=1.0);
                let syn_id = Uuid::new_v4();
                let event = if self.rng.gen_bool(0.5) {
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
                self.store
                    .append(&event)
                    .map_err(|_| AddRandomNeuronError::StorageError)?;
                self.network.apply(&event);
            }
        }

        Ok(neuron_id)
    }
}
