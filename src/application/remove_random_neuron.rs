//! Command and handler for removing a random neuron.

use rand::{seq::SliceRandom, Rng};
use uuid::Uuid;

use super::NetworkHandlerBase;
use crate::domain::{Event, RandomNeuronRemoved};
use crate::infrastructure::EventStore;

/// Command requesting the removal of a random neuron.
#[derive(Debug, Clone, Copy)]
pub struct RemoveRandomNeuronCommand;

/// Errors that can occur when removing a neuron.
#[derive(Debug, Clone, PartialEq)]
pub enum RemoveRandomNeuronError {
    /// The network does not contain a removable neuron.
    NoNeuronAvailable,
    /// Persisting the event failed.
    StorageError,
}

/// Handles [`RemoveRandomNeuronCommand`], emitting events and updating state.
pub struct RemoveRandomNeuronHandler<S: EventStore, R: Rng> {
    /// Shared handler state including store, network and RNG.
    pub base: NetworkHandlerBase<S, R>,
}

impl<S: EventStore, R: Rng> RemoveRandomNeuronHandler<S, R> {
    /// Loads events from the store to initialize the handler.
    pub fn new(store: S, rng: R) -> Result<Self, S::Error> {
        Ok(Self {
            base: NetworkHandlerBase::new(store, rng)?,
        })
    }

    /// Handles the command and returns the identifier of the removed neuron.
    pub fn handle(
        &mut self,
        _cmd: RemoveRandomNeuronCommand,
    ) -> Result<Uuid, RemoveRandomNeuronError> {
        let base = &mut self.base;
        let ids: Vec<Uuid> = base.network.neurons.keys().copied().collect();
        if ids.is_empty() {
            return Err(RemoveRandomNeuronError::NoNeuronAvailable);
        }
        let neuron_id = *ids
            .choose(&mut base.rng)
            .expect("candidate list is non-empty");
        let event = Event::RandomNeuronRemoved(RandomNeuronRemoved { neuron_id });
        base.store
            .append(&event)
            .map_err(|_| RemoveRandomNeuronError::StorageError)?;
        base.network.apply(&event);
        Ok(neuron_id)
    }
}
