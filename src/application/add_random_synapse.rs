//! Command and handler for randomly adding a synapse between two neurons.

use rand::{seq::SliceRandom, Rng};
use uuid::Uuid;

use crate::domain::{Event, Network, RandomSynapseAdded};
use crate::infrastructure::EventStore;

/// Command requesting the creation of a random synapse.
#[derive(Debug, Clone, Copy)]
pub struct AddRandomSynapseCommand;

/// Possible errors when adding a random synapse.
#[derive(Debug, Clone, PartialEq)]
pub enum AddRandomSynapseError {
    /// The network does not contain at least two neurons.
    NotEnoughNeurons,
    /// All neuron pairs are already connected.
    NoAvailableConnection,
    /// Persisting the event failed.
    StorageError,
}

/// Handles [`AddRandomSynapseCommand`], emitting a [`RandomSynapseAdded`] event.
pub struct AddRandomSynapseHandler<S: EventStore, R: Rng> {
    /// Event store used for persistence.
    pub store: S,
    /// Current network state derived from applied events.
    pub network: Network,
    rng: R,
}

impl<S: EventStore, R: Rng> AddRandomSynapseHandler<S, R> {
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

    /// Handles the command and returns the identifier of the created synapse.
    pub fn handle(&mut self, _cmd: AddRandomSynapseCommand) -> Result<Uuid, AddRandomSynapseError> {
        let neuron_ids: Vec<Uuid> = self.network.neurons.keys().copied().collect();
        if neuron_ids.len() < 2 {
            return Err(AddRandomSynapseError::NotEnoughNeurons);
        }

        let mut pairs = Vec::new();
        for &from in &neuron_ids {
            for &to in &neuron_ids {
                if from == to {
                    continue;
                }
                if self
                    .network
                    .synapses
                    .values()
                    .any(|s| s.from == from && s.to == to)
                {
                    continue;
                }
                pairs.push((from, to));
            }
        }

        let (from, to) = *pairs
            .choose(&mut self.rng)
            .ok_or(AddRandomSynapseError::NoAvailableConnection)?;
        let weight = self.rng.gen_range(-1.0..=1.0);
        let synapse_id = Uuid::new_v4();
        let event = Event::RandomSynapseAdded(RandomSynapseAdded {
            synapse_id,
            from,
            to,
            weight,
        });
        self.store
            .append(&event)
            .map_err(|_| AddRandomSynapseError::StorageError)?;
        self.network.apply(&event);
        Ok(synapse_id)
    }
}
