//! Command and handler for mutating the weight of a random synapse.
//!
//! The mutation adds Gaussian noise with a configurable standard deviation to
//! the existing weight. A corresponding [`SynapseWeightMutated`] event is
//! emitted, persisted, and applied to the domain.

use rand::{seq::SliceRandom, Rng};
use rand_distr::{Distribution, Normal};
use uuid::Uuid;

use super::NetworkHandlerBase;
use crate::domain::{Event, SynapseWeightMutated};
use crate::infrastructure::EventStore;

/// Command requesting mutation of a random synapse weight.
#[derive(Debug, Clone, Copy)]
pub struct MutateRandomSynapseWeightCommand {
    /// Standard deviation of the Gaussian noise to add to the weight.
    pub std_dev: f64,
}

/// Errors that can occur while mutating a synapse weight.
#[derive(Debug, Clone, PartialEq)]
pub enum MutateRandomSynapseWeightError {
    /// The network does not contain any synapse to mutate.
    NoSynapseAvailable,
    /// The provided standard deviation is not valid (must be positive).
    InvalidStdDev,
    /// Persisting the event failed.
    StorageError,
}

/// Handles [`MutateRandomSynapseWeightCommand`], emitting and applying
/// [`SynapseWeightMutated`] events.
pub struct MutateRandomSynapseWeightHandler<S: EventStore, R: Rng> {
    /// Shared handler state including store, network and RNG.
    pub base: NetworkHandlerBase<S, R>,
}

impl<S: EventStore, R: Rng> MutateRandomSynapseWeightHandler<S, R> {
    /// Loads events from the store to initialize the handler.
    pub fn new(store: S, rng: R) -> Result<Self, S::Error> {
        Ok(Self {
            base: NetworkHandlerBase::new(store, rng)?,
        })
    }

    /// Handles the command and returns the identifier of the mutated synapse.
    ///
    /// # Errors
    /// Returns [`MutateRandomSynapseWeightError::NoSynapseAvailable`] if the
    /// network contains no synapse, [`MutateRandomSynapseWeightError::InvalidStdDev`]
    /// if the provided standard deviation is non-positive, and
    /// [`MutateRandomSynapseWeightError::StorageError`] if persisting the event
    /// fails.
    ///
    /// # Examples
    /// ```
    /// use aei_framework::{
    ///     MutateRandomSynapseWeightCommand, MutateRandomSynapseWeightHandler, FileEventStore,
    /// };
    /// use rand::thread_rng;
    /// use std::path::PathBuf;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let store = FileEventStore::new(PathBuf::from("events.log"));
    /// let mut handler = MutateRandomSynapseWeightHandler::new(store, thread_rng())?;
    /// let _ = handler.handle(MutateRandomSynapseWeightCommand { std_dev: 0.1 });
    /// # Ok(()) }
    /// ```
    pub fn handle(
        &mut self,
        cmd: MutateRandomSynapseWeightCommand,
    ) -> Result<Uuid, MutateRandomSynapseWeightError> {
        if cmd.std_dev <= 0.0 {
            return Err(MutateRandomSynapseWeightError::InvalidStdDev);
        }
        let base = &mut self.base;
        let ids: Vec<Uuid> = base.network.synapses.keys().copied().collect();
        if ids.is_empty() {
            return Err(MutateRandomSynapseWeightError::NoSynapseAvailable);
        }
        let synapse_id = *ids
            .choose(&mut base.rng)
            .expect("candidate list is non-empty");
        let old_weight = base
            .network
            .synapses
            .get(&synapse_id)
            .expect("synapse exists")
            .weight;
        let normal = Normal::new(0.0, cmd.std_dev)
            .map_err(|_| MutateRandomSynapseWeightError::InvalidStdDev)?;
        let noise = normal.sample(&mut base.rng);
        let new_weight = old_weight + noise;
        let event = Event::SynapseWeightMutated(SynapseWeightMutated {
            synapse_id,
            old_weight,
            new_weight,
        });
        base.store
            .append(&event)
            .map_err(|_| MutateRandomSynapseWeightError::StorageError)?;
        base.network.apply(&event);
        Ok(synapse_id)
    }
}
