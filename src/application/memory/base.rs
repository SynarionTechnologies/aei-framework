//! Shared base for memory command handlers.
//!
//! Aggregates a memory event store and the hydrated [`AdaptiveMemory`].
//! Provides helpers to persist events and prune excess entries.
//!
//! # Examples
//! ```
//! use aei_framework::application::memory::MemoryHandlerBase;
//! use aei_framework::infrastructure::FileMemoryEventStore;
//! use std::path::PathBuf;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let store = FileMemoryEventStore::new(PathBuf::from("memory.log"));
//! let _base = MemoryHandlerBase::new(store, 10)?;
//! # Ok(()) }
//! ```

use uuid::Uuid;

use crate::domain::{AdaptiveMemory, MemoryEvent, MemoryPruned};
use crate::infrastructure::MemoryEventStore;

/// Maintains shared state for memory handlers.
pub struct MemoryHandlerBase<S: MemoryEventStore> {
    /// Event store used for persistence.
    pub store: S,
    /// Current adaptive memory rebuilt from events.
    pub memory: AdaptiveMemory,
}

impl<S: MemoryEventStore> MemoryHandlerBase<S> {
    /// Loads events from the store and hydrates [`AdaptiveMemory`].
    ///
    /// # Arguments
    /// * `store` - Event store containing past memory events.
    /// * `max_size` - Maximum number of entries retained in memory.
    ///
    /// # Errors
    /// Returns [`MemoryEventStore::Error`] if loading events fails.
    pub fn new(mut store: S, max_size: usize) -> Result<Self, S::Error> {
        let events = store.load()?;
        let memory = AdaptiveMemory::hydrate(max_size, &events);
        Ok(Self { store, memory })
    }

    /// Persists an event and applies it to the memory state.
    ///
    /// # Errors
    /// Returns [`MemoryEventStore::Error`] if persistence fails.
    pub fn persist(&mut self, event: &MemoryEvent) -> Result<(), S::Error> {
        self.store.append(event)?;
        self.memory.apply(event);
        Ok(())
    }

    /// Prunes lowest scoring entries when capacity is exceeded.
    ///
    /// Returns the identifiers of removed entries.
    ///
    /// # Errors
    /// Returns [`MemoryEventStore::Error`] if persisting the pruning event fails.
    pub fn prune(&mut self) -> Result<Vec<Uuid>, S::Error> {
        if self.memory.entries.len() <= self.memory.max_size {
            return Ok(Vec::new());
        }
        let excess = self.memory.entries.len() - self.memory.max_size;
        let mut entries = self.memory.entries.clone();
        entries.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
        let removed: Vec<Uuid> = entries.iter().take(excess).map(|e| e.id).collect();
        let event = MemoryEvent::MemoryPruned(MemoryPruned {
            removed_entries: removed.clone(),
        });
        self.store.append(&event)?;
        self.memory.apply(&event);
        Ok(removed)
    }
}
