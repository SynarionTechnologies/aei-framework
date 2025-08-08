//! Command handlers mutating the adaptive memory through events.

use chrono::Utc;
use uuid::Uuid;

use crate::domain::{
    AdaptiveMemory, MemoryEntry, MemoryEntryAdded, MemoryEntryRemoved, MemoryEvent, MemoryPruned,
    MemoryScoreUpdated,
};
use crate::infrastructure::MemoryEventStore;

use super::commands::{
    AddMemoryEntryCommand, PruneMemoryCommand, RemoveMemoryEntryCommand, UpdateMemoryScoreCommand,
};

/// Handles [`AddMemoryEntryCommand`].
pub struct AddMemoryEntryHandler<S: MemoryEventStore> {
    pub store: S,
    pub memory: AdaptiveMemory,
}

/// Possible errors when adding a memory entry.
#[derive(Debug, Clone, PartialEq)]
pub enum AddMemoryEntryError {
    /// Provided score was outside the `[0.0, 1.0]` range.
    InvalidScore,
    /// Persisting the event failed.
    StorageError,
}

impl<S: MemoryEventStore> AddMemoryEntryHandler<S> {
    /// Loads the memory state from the event store.
    pub fn new(mut store: S, max_size: usize) -> Result<Self, S::Error> {
        let events = store.load()?;
        let memory = AdaptiveMemory::hydrate(max_size, &events);
        Ok(Self { store, memory })
    }

    /// Handles the command, returning the identifier of the created entry.
    pub fn handle(&mut self, cmd: AddMemoryEntryCommand) -> Result<Uuid, AddMemoryEntryError> {
        if !(0.0..=1.0).contains(&cmd.score) {
            return Err(AddMemoryEntryError::InvalidScore);
        }
        let entry = MemoryEntry {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: cmd.event_type,
            payload: cmd.payload,
            score: cmd.score,
        };
        let event = MemoryEvent::MemoryEntryAdded(MemoryEntryAdded {
            entry: entry.clone(),
        });
        self.store
            .append(&event)
            .map_err(|_| AddMemoryEntryError::StorageError)?;
        self.memory.apply(&event);
        if self.memory.entries.len() > self.memory.max_size {
            self.prune_lowest()
                .map_err(|_| AddMemoryEntryError::StorageError)?;
        }
        Ok(entry.id)
    }

    fn prune_lowest(&mut self) -> Result<(), S::Error> {
        let excess = self.memory.entries.len() - self.memory.max_size;
        let mut entries = self.memory.entries.clone();
        entries.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
        let removed: Vec<Uuid> = entries.iter().take(excess).map(|e| e.id).collect();
        let event = MemoryEvent::MemoryPruned(MemoryPruned {
            removed_entries: removed,
        });
        self.store.append(&event)?;
        self.memory.apply(&event);
        Ok(())
    }
}

/// Handles [`RemoveMemoryEntryCommand`].
pub struct RemoveMemoryEntryHandler<S: MemoryEventStore> {
    pub store: S,
    pub memory: AdaptiveMemory,
}

/// Possible errors when removing a memory entry.
#[derive(Debug, Clone, PartialEq)]
pub enum RemoveMemoryEntryError {
    /// No entry with the specified identifier exists.
    NotFound,
    /// Persisting the event failed.
    StorageError,
}

impl<S: MemoryEventStore> RemoveMemoryEntryHandler<S> {
    /// Loads state from the event store.
    pub fn new(mut store: S, max_size: usize) -> Result<Self, S::Error> {
        let events = store.load()?;
        let memory = AdaptiveMemory::hydrate(max_size, &events);
        Ok(Self { store, memory })
    }

    /// Handles the command.
    pub fn handle(&mut self, cmd: RemoveMemoryEntryCommand) -> Result<(), RemoveMemoryEntryError> {
        if !self.memory.entries.iter().any(|e| e.id == cmd.entry_id) {
            return Err(RemoveMemoryEntryError::NotFound);
        }
        let event = MemoryEvent::MemoryEntryRemoved(MemoryEntryRemoved {
            entry_id: cmd.entry_id,
        });
        self.store
            .append(&event)
            .map_err(|_| RemoveMemoryEntryError::StorageError)?;
        self.memory.apply(&event);
        Ok(())
    }
}

/// Handles [`PruneMemoryCommand`].
pub struct PruneMemoryHandler<S: MemoryEventStore> {
    pub store: S,
    pub memory: AdaptiveMemory,
}

/// Errors when pruning memory.
#[derive(Debug, Clone, PartialEq)]
pub enum PruneMemoryError {
    /// Persisting the event failed.
    StorageError,
}

impl<S: MemoryEventStore> PruneMemoryHandler<S> {
    /// Loads state from the event store.
    pub fn new(mut store: S, max_size: usize) -> Result<Self, S::Error> {
        let events = store.load()?;
        let memory = AdaptiveMemory::hydrate(max_size, &events);
        Ok(Self { store, memory })
    }

    /// Removes lowest scoring entries if capacity is exceeded.
    pub fn handle(&mut self, _cmd: PruneMemoryCommand) -> Result<Vec<Uuid>, PruneMemoryError> {
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
        self.store
            .append(&event)
            .map_err(|_| PruneMemoryError::StorageError)?;
        self.memory.apply(&event);
        Ok(removed)
    }
}

/// Handles [`UpdateMemoryScoreCommand`].
pub struct UpdateMemoryScoreHandler<S: MemoryEventStore> {
    pub store: S,
    pub memory: AdaptiveMemory,
}

/// Errors when updating a score.
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateMemoryScoreError {
    /// Specified entry was not found.
    NotFound,
    /// Provided score was outside the `[0.0, 1.0]` range.
    InvalidScore,
    /// Persisting the event failed.
    StorageError,
}

impl<S: MemoryEventStore> UpdateMemoryScoreHandler<S> {
    /// Loads state from the event store.
    pub fn new(mut store: S, max_size: usize) -> Result<Self, S::Error> {
        let events = store.load()?;
        let memory = AdaptiveMemory::hydrate(max_size, &events);
        Ok(Self { store, memory })
    }

    /// Handles the command by emitting a [`MemoryScoreUpdated`] event.
    pub fn handle(&mut self, cmd: UpdateMemoryScoreCommand) -> Result<(), UpdateMemoryScoreError> {
        if !(0.0..=1.0).contains(&cmd.new_score) {
            return Err(UpdateMemoryScoreError::InvalidScore);
        }
        let entry = self
            .memory
            .entries
            .iter()
            .find(|e| e.id == cmd.entry_id)
            .cloned()
            .ok_or(UpdateMemoryScoreError::NotFound)?;
        let event = MemoryEvent::MemoryScoreUpdated(MemoryScoreUpdated {
            entry_id: cmd.entry_id,
            old_score: entry.score,
            new_score: cmd.new_score,
        });
        self.store
            .append(&event)
            .map_err(|_| UpdateMemoryScoreError::StorageError)?;
        self.memory.apply(&event);
        Ok(())
    }
}
