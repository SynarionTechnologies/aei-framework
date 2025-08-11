//! Command and handler for randomly adding a synapse between two neurons.

use rand::{seq::SliceRandom, Rng};
use uuid::Uuid;

use super::NetworkHandlerBase;
use crate::domain::{Event, RandomSynapseAdded};
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
    /// Shared handler state including store, network and RNG.
    pub base: NetworkHandlerBase<S, R>,
}

impl<S: EventStore, R: Rng> AddRandomSynapseHandler<S, R> {
    /// Loads events from the store to initialize the handler.
    pub fn new(store: S, rng: R) -> Result<Self, S::Error> {
        Ok(Self {
            base: NetworkHandlerBase::new(store, rng)?,
        })
    }

    /// Handles the command and returns the identifier of the created synapse.
    pub fn handle(&mut self, _cmd: AddRandomSynapseCommand) -> Result<Uuid, AddRandomSynapseError> {
        let base = &mut self.base;
        let neuron_ids: Vec<Uuid> = base.network.neurons.keys().copied().collect();
        if neuron_ids.len() < 2 {
            return Err(AddRandomSynapseError::NotEnoughNeurons);
        }

        let mut pairs = Vec::new();
        for &from in &neuron_ids {
            for &to in &neuron_ids {
                if from == to {
                    continue;
                }
                if base
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
            .choose(&mut base.rng)
            .ok_or(AddRandomSynapseError::NoAvailableConnection)?;
        let weight = base.rng.gen_range(-1.0..=1.0);
        let synapse_id = Uuid::new_v4();
        let event = Event::RandomSynapseAdded(RandomSynapseAdded {
            synapse_id,
            from,
            to,
            weight,
        });
        base.store
            .append(&event)
            .map_err(|_| AddRandomSynapseError::StorageError)?;
        base.network.apply(&event);
        Ok(synapse_id)
    }
}
