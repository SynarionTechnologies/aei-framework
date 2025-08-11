//! Append-only store for [`MemoryEvent`](crate::domain::MemoryEvent).
//!
//! [`FileMemoryEventStore`] is a type alias over [`JsonlEventStore`].

use std::io;

use crate::domain::MemoryEvent;

use super::JsonlEventStore;

/// Storage backend dedicated to memory events.
pub trait MemoryEventStore {
    /// Error type returned by the store.
    type Error;
    /// Persist an event to the underlying storage.
    fn append(&mut self, event: &MemoryEvent) -> Result<(), Self::Error>;
    /// Load all stored events in chronological order.
    fn load(&mut self) -> Result<Vec<MemoryEvent>, Self::Error>;
}

/// JSON-lines file based implementation of [`MemoryEventStore`].
pub type FileMemoryEventStore = JsonlEventStore<MemoryEvent>;

impl MemoryEventStore for JsonlEventStore<MemoryEvent> {
    type Error = io::Error;

    fn append(&mut self, event: &MemoryEvent) -> Result<(), Self::Error> {
        JsonlEventStore::append(self, event)
    }

    fn load(&mut self) -> Result<Vec<MemoryEvent>, Self::Error> {
        JsonlEventStore::load(self)
    }
}
