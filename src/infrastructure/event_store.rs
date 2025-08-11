//! Append-only event storage.
//!
//! [`FileEventStore`] persists domain events as JSON Lines using
//! [`JsonlEventStore`].

use std::io;

use crate::domain::Event;

use super::JsonlEventStore;

/// Storage backend for domain events.
pub trait EventStore {
    /// The error type produced by this event store.
    type Error;
    /// Persist an event to the underlying storage.
    fn append(&mut self, event: &Event) -> Result<(), Self::Error>;
    /// Load all events in chronological order.
    fn load(&mut self) -> Result<Vec<Event>, Self::Error>;
}

/// JSON-lines file based implementation of [`EventStore`].
pub type FileEventStore = JsonlEventStore<Event>;

impl EventStore for JsonlEventStore<Event> {
    type Error = io::Error;

    fn append(&mut self, event: &Event) -> Result<(), Self::Error> {
        JsonlEventStore::append(self, event)
    }

    fn load(&mut self) -> Result<Vec<Event>, Self::Error> {
        JsonlEventStore::load(self)
    }
}
