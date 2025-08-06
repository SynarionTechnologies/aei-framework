//! Handles write-side commands and persists resulting events.

use crate::commands::Command;
use crate::domain::Network;
use crate::events::Event;
use crate::infrastructure::EventStore;

/// Processes commands, emitting events and updating the in-memory state.
pub struct CommandHandler<S: EventStore> {
    /// Event store used for persistence.
    pub store: S,
    /// Current network state derived from applied events.
    pub network: Network,
}

impl<S: EventStore> CommandHandler<S> {
    /// Loads all events from the store and constructs a handler.
    pub fn new(mut store: S) -> Result<Self, S::Error> {
        let events = store.load()?;
        let network = Network::hydrate(&events);
        Ok(Self { store, network })
    }

    /// Handles a command by converting it to an event and applying it.
    pub fn handle(&mut self, command: Command) -> Result<(), S::Error> {
        let event = match command {
            Command::AddNeuron { id, activation } => Event::NeuronAdded { id, activation },
            Command::RemoveNeuron { id } => Event::NeuronRemoved { id },
            Command::CreateSynapse { id, from, to, weight } => {
                Event::SynapseCreated { id, from, to, weight }
            }
            Command::RemoveSynapse { id } => Event::SynapseRemoved { id },
        };
        self.store.append(&event)?;
        self.network.apply(&event);
        Ok(())
    }
}

