//! Command and handler for removing a random neuron.

use rand::{seq::SliceRandom, Rng};
use uuid::Uuid;

use crate::domain::{Event, Network, RandomNeuronRemoved};
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
    /// Event store used for persistence.
    pub store: S,
    /// Current network state derived from applied events.
    pub network: Network,
    rng: R,
}

impl<S: EventStore, R: Rng> RemoveRandomNeuronHandler<S, R> {
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

    /// Handles the command and returns the identifier of the removed neuron.
    pub fn handle(
        &mut self,
        _cmd: RemoveRandomNeuronCommand,
    ) -> Result<Uuid, RemoveRandomNeuronError> {
        let ids: Vec<Uuid> = self.network.neurons.keys().copied().collect();
        if ids.is_empty() {
            return Err(RemoveRandomNeuronError::NoNeuronAvailable);
        }
        let neuron_id = *ids
            .choose(&mut self.rng)
            .expect("candidate list is non-empty");
        let event = Event::RandomNeuronRemoved(RandomNeuronRemoved { neuron_id });
        self.store
            .append(&event)
            .map_err(|_| RemoveRandomNeuronError::StorageError)?;
        self.network.apply(&event);
        Ok(neuron_id)
    }
}
