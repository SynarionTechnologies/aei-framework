//! Command and handler for explicitly setting a synapse's weight.
//!
//! This operation emits a [`SynapseWeightSet`](crate::domain::SynapseWeightSet)
//! event, which is persisted and applied to the [`Network`](crate::domain::Network).

use crate::domain::{Event, Network, SynapseWeightSet};
use crate::infrastructure::EventStore;
use uuid::Uuid;

/// Command requesting to assign a new weight to a synapse.
#[derive(Debug, Clone)]
pub struct SetSynapseWeightCommand {
    /// Identifier of the synapse to update.
    pub synapse_id: Uuid,
    /// Desired weight value.
    pub new_weight: f64,
}

/// Errors that may occur while setting a synapse's weight.
#[derive(Debug, Clone, PartialEq)]
pub enum SetSynapseWeightError {
    /// The specified synapse does not exist in the network.
    SynapseNotFound,
    /// Persisting the event failed.
    StorageError,
}

/// Handles [`SetSynapseWeightCommand`] and applies the resulting event.
pub struct SetSynapseWeightHandler<S: EventStore> {
    /// Event store used for persistence.
    pub store: S,
    /// Current network state reconstructed from events.
    pub network: Network,
}

impl<S: EventStore> SetSynapseWeightHandler<S> {
    /// Loads events from the store to initialize the handler.
    pub fn new(mut store: S) -> Result<Self, S::Error> {
        let events = store.load()?;
        let network = Network::hydrate(&events);
        Ok(Self { store, network })
    }

    /// Handles the command by emitting and applying a [`SynapseWeightSet`] event.
    ///
    /// # Errors
    /// Returns [`SetSynapseWeightError::SynapseNotFound`] if the target synapse is
    /// missing, or [`SetSynapseWeightError::StorageError`] if persisting the event
    /// fails.
    ///
    /// # Examples
    /// ```
    /// use aei_framework::{SetSynapseWeightCommand, SetSynapseWeightHandler, FileEventStore};
    /// use uuid::Uuid;
    /// use std::path::PathBuf;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let store = FileEventStore::new(PathBuf::from("events.log"));
    /// let mut handler = SetSynapseWeightHandler::new(store)?;
    /// let synapse_id = Uuid::new_v4();
    /// // network must already contain `synapse_id`
    /// let _ = handler.handle(SetSynapseWeightCommand { synapse_id, new_weight: 0.5 });
    /// # Ok(()) }
    /// ```
    pub fn handle(&mut self, cmd: SetSynapseWeightCommand) -> Result<(), SetSynapseWeightError> {
        let old_weight = self
            .network
            .synapses
            .get(&cmd.synapse_id)
            .map(|s| s.weight)
            .ok_or(SetSynapseWeightError::SynapseNotFound)?;
        let event = Event::SynapseWeightSet(SynapseWeightSet {
            synapse_id: cmd.synapse_id,
            old_weight,
            new_weight: cmd.new_weight,
        });
        self.store
            .append(&event)
            .map_err(|_| SetSynapseWeightError::StorageError)?;
        self.network.apply(&event);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{RandomNeuronAdded, RandomSynapseAdded};
    use crate::infrastructure::FileEventStore;
    use std::path::PathBuf;
    use uuid::Uuid;

    fn temp_path() -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("set_weight_{}.log", Uuid::new_v4()));
        path
    }

    #[test]
    fn set_synapse_weight_updates_network() {
        let path = temp_path();
        let mut store = FileEventStore::new(path.clone());
        let n1 = Uuid::new_v4();
        let n2 = Uuid::new_v4();
        let syn_id = Uuid::new_v4();
        let events = [
            Event::RandomNeuronAdded(RandomNeuronAdded {
                neuron_id: n1,
                activation: crate::domain::Activation::Identity,
            }),
            Event::RandomNeuronAdded(RandomNeuronAdded {
                neuron_id: n2,
                activation: crate::domain::Activation::Identity,
            }),
            Event::RandomSynapseAdded(RandomSynapseAdded {
                synapse_id: syn_id,
                from: n1,
                to: n2,
                weight: 1.0,
            }),
        ];
        for e in &events {
            store.append(e).unwrap();
        }

        let mut handler = SetSynapseWeightHandler::new(FileEventStore::new(path)).unwrap();
        handler
            .handle(SetSynapseWeightCommand {
                synapse_id: syn_id,
                new_weight: 2.0,
            })
            .unwrap();
        assert_eq!(handler.network.synapses.get(&syn_id).unwrap().weight, 2.0);
    }
}
