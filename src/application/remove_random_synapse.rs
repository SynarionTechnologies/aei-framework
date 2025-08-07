//! Command and handler for removing a random synapse.

use rand::{seq::SliceRandom, Rng};
use uuid::Uuid;

use crate::domain::{Event, Network, RandomSynapseRemoved};
use crate::infrastructure::EventStore;

/// Command requesting the removal of a random synapse.
///
/// # Examples
/// ```
/// use aei_framework::RemoveRandomSynapseCommand;
/// let cmd = RemoveRandomSynapseCommand;
/// println!("{:?}", cmd);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct RemoveRandomSynapseCommand;

/// Errors that can occur when removing a synapse.
#[derive(Debug, Clone, PartialEq)]
pub enum RemoveRandomSynapseError {
    /// The network does not contain any synapse to remove.
    NoSynapseAvailable,
    /// Persisting the event failed.
    StorageError,
}

/// Handles [`RemoveRandomSynapseCommand`], emitting events and updating state.
pub struct RemoveRandomSynapseHandler<S: EventStore, R: Rng> {
    /// Event store used for persistence.
    pub store: S,
    /// Current network state derived from applied events.
    pub network: Network,
    rng: R,
}

impl<S: EventStore, R: Rng> RemoveRandomSynapseHandler<S, R> {
    /// Loads events from the store to initialize the handler.
    ///
    /// # Errors
    /// Propagates storage backend errors.
    pub fn new(mut store: S, rng: R) -> Result<Self, S::Error> {
        let events = store.load()?;
        let network = Network::hydrate(&events);
        Ok(Self {
            store,
            network,
            rng,
        })
    }

    /// Handles the command and returns the identifier of the removed synapse.
    ///
    /// # Errors
    /// Returns [`RemoveRandomSynapseError::NoSynapseAvailable`] if the network
    /// does not contain any synapse and
    /// [`RemoveRandomSynapseError::StorageError`] if persisting the event
    /// fails.
    ///
    /// # Examples
    /// ```
    /// use aei_framework::{
    ///     RemoveRandomSynapseCommand, RemoveRandomSynapseHandler, FileEventStore,
    /// };
    /// use rand::thread_rng;
    /// use std::path::PathBuf;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let store = FileEventStore::new(PathBuf::from("events.log"));
    /// let mut handler = RemoveRandomSynapseHandler::new(store, thread_rng())?;
    /// let _ = handler.handle(RemoveRandomSynapseCommand);
    /// # Ok(()) }
    /// ```
    pub fn handle(
        &mut self,
        _cmd: RemoveRandomSynapseCommand,
    ) -> Result<Uuid, RemoveRandomSynapseError> {
        let ids: Vec<Uuid> = self.network.synapses.keys().copied().collect();
        if ids.is_empty() {
            return Err(RemoveRandomSynapseError::NoSynapseAvailable);
        }
        let synapse_id = *ids
            .choose(&mut self.rng)
            .expect("candidate list is non-empty");
        let event = Event::RandomSynapseRemoved(RandomSynapseRemoved { synapse_id });
        self.store
            .append(&event)
            .map_err(|_| RemoveRandomSynapseError::StorageError)?;
        self.network.apply(&event);
        Ok(synapse_id)
    }
}
