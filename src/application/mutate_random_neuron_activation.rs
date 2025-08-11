//! Command and handler for mutating the activation of a random neuron.
//!
//! The mutation replaces the current activation function of a randomly
//! selected neuron with a different one. The change is persisted as a
//! [`NeuronActivationMutated`] event and applied to the [`Network`] aggregate.

use rand::{seq::SliceRandom, Rng};
use uuid::Uuid;

use super::NetworkHandlerBase;
use crate::domain::{Activation, Event, NeuronActivationMutated};
use crate::infrastructure::EventStore;

/// Command requesting mutation of a random neuron's activation.
#[derive(Debug, Clone, Copy)]
pub struct MutateRandomNeuronActivationCommand {
    /// When true, input and output neurons are excluded from selection.
    pub exclude_io: bool,
}

/// Errors that can occur while mutating a neuron's activation.
#[derive(Debug, Clone, PartialEq)]
pub enum MutateNeuronActivationError {
    /// No neuron matched the selection criteria.
    NoEligibleNeuron,
    /// Persisting the event failed.
    StorageError,
}

/// Handles [`MutateRandomNeuronActivationCommand`], emitting and applying
/// [`NeuronActivationMutated`] events.
pub struct MutateRandomNeuronActivationHandler<S: EventStore, R: Rng> {
    /// Shared handler state including store, network and RNG.
    pub base: NetworkHandlerBase<S, R>,
}

impl<S: EventStore, R: Rng> MutateRandomNeuronActivationHandler<S, R> {
    /// Loads events from the store to initialize the handler.
    pub fn new(store: S, rng: R) -> Result<Self, S::Error> {
        Ok(Self {
            base: NetworkHandlerBase::new(store, rng)?,
        })
    }

    /// Handles the command and returns the identifier of the mutated neuron.
    ///
    /// # Errors
    /// Returns [`MutateNeuronActivationError::NoEligibleNeuron`] if no neuron
    /// satisfies the selection criteria and
    /// [`MutateNeuronActivationError::StorageError`] if persisting the event
    /// fails.
    ///
    /// # Examples
    /// ```
    /// use aei_framework::{
    ///     MutateRandomNeuronActivationCommand, MutateRandomNeuronActivationHandler,
    ///     FileEventStore,
    /// };
    /// use rand::thread_rng;
    /// use std::path::PathBuf;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let store = FileEventStore::new(PathBuf::from("events.log"));
    /// let mut handler = MutateRandomNeuronActivationHandler::new(store, thread_rng())?;
    /// let _ = handler.handle(MutateRandomNeuronActivationCommand { exclude_io: false });
    /// # Ok(()) }
    /// ```
    pub fn handle(
        &mut self,
        cmd: MutateRandomNeuronActivationCommand,
    ) -> Result<Uuid, MutateNeuronActivationError> {
        let base = &mut self.base;
        let mut candidates: Vec<Uuid> = base.network.neurons.keys().copied().collect();
        if cmd.exclude_io {
            candidates.retain(|id| {
                let has_in = base.network.synapses.values().any(|s| s.to == *id);
                let has_out = base.network.synapses.values().any(|s| s.from == *id);
                has_in && has_out
            });
        }
        let neuron_id = *candidates
            .choose(&mut base.rng)
            .ok_or(MutateNeuronActivationError::NoEligibleNeuron)?;

        let old_activation = base
            .network
            .neurons
            .get(&neuron_id)
            .expect("neuron exists")
            .activation;
        let mut activations = vec![
            Activation::Identity,
            Activation::Sigmoid,
            Activation::ReLU,
            Activation::Tanh,
        ];
        activations.retain(|a| *a != old_activation);
        let new_activation = *activations
            .choose(&mut base.rng)
            .expect("activation list is non-empty");

        let event = Event::NeuronActivationMutated(NeuronActivationMutated {
            neuron_id,
            old_activation,
            new_activation,
        });
        base.store
            .append(&event)
            .map_err(|_| MutateNeuronActivationError::StorageError)?;
        base.network.apply(&event);
        Ok(neuron_id)
    }
}
