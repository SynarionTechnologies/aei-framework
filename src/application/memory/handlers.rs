//! Command handlers mutating the adaptive memory through events.

use chrono::Utc;
use uuid::Uuid;

use crate::domain::{
    MemoryEntry, MemoryEntryAdded, MemoryEntryRemoved, MemoryEvent, MemoryScoreUpdated,
};
use crate::infrastructure::MemoryEventStore;

use super::base::MemoryHandlerBase;
use super::commands::{
    AddMemoryEntryCommand, PruneMemoryCommand, RemoveMemoryEntryCommand, UpdateMemoryScoreCommand,
};

/// Handles [`AddMemoryEntryCommand`].
pub struct AddMemoryEntryHandler<S: MemoryEventStore> {
    /// Shared base containing the event store and memory state.
    pub base: MemoryHandlerBase<S>,
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
    pub fn new(store: S, max_size: usize) -> Result<Self, S::Error> {
        Ok(Self {
            base: MemoryHandlerBase::new(store, max_size)?,
        })
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
        self.base
            .persist(&event)
            .map_err(|_| AddMemoryEntryError::StorageError)?;
        self.base
            .prune()
            .map_err(|_| AddMemoryEntryError::StorageError)?;
        Ok(entry.id)
    }
}

/// Handles [`RemoveMemoryEntryCommand`].
pub struct RemoveMemoryEntryHandler<S: MemoryEventStore> {
    /// Shared base containing the event store and memory state.
    pub base: MemoryHandlerBase<S>,
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
    pub fn new(store: S, max_size: usize) -> Result<Self, S::Error> {
        Ok(Self {
            base: MemoryHandlerBase::new(store, max_size)?,
        })
    }

    /// Handles the command.
    pub fn handle(&mut self, cmd: RemoveMemoryEntryCommand) -> Result<(), RemoveMemoryEntryError> {
        if !self
            .base
            .memory
            .entries
            .iter()
            .any(|e| e.id == cmd.entry_id)
        {
            return Err(RemoveMemoryEntryError::NotFound);
        }
        let event = MemoryEvent::MemoryEntryRemoved(MemoryEntryRemoved {
            entry_id: cmd.entry_id,
        });
        self.base
            .persist(&event)
            .map_err(|_| RemoveMemoryEntryError::StorageError)?;
        Ok(())
    }
}

/// Handles [`PruneMemoryCommand`].
pub struct PruneMemoryHandler<S: MemoryEventStore> {
    /// Shared base containing the event store and memory state.
    pub base: MemoryHandlerBase<S>,
}

/// Errors when pruning memory.
#[derive(Debug, Clone, PartialEq)]
pub enum PruneMemoryError {
    /// Persisting the event failed.
    StorageError,
}

impl<S: MemoryEventStore> PruneMemoryHandler<S> {
    /// Loads state from the event store.
    pub fn new(store: S, max_size: usize) -> Result<Self, S::Error> {
        Ok(Self {
            base: MemoryHandlerBase::new(store, max_size)?,
        })
    }

    /// Removes lowest scoring entries if capacity is exceeded.
    pub fn handle(&mut self, _cmd: PruneMemoryCommand) -> Result<Vec<Uuid>, PruneMemoryError> {
        self.base
            .prune()
            .map_err(|_| PruneMemoryError::StorageError)
    }
}

/// Handles [`UpdateMemoryScoreCommand`].
pub struct UpdateMemoryScoreHandler<S: MemoryEventStore> {
    /// Shared base containing the event store and memory state.
    pub base: MemoryHandlerBase<S>,
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
    pub fn new(store: S, max_size: usize) -> Result<Self, S::Error> {
        Ok(Self {
            base: MemoryHandlerBase::new(store, max_size)?,
        })
    }

    /// Handles the command by emitting a [`MemoryScoreUpdated`] event.
    pub fn handle(&mut self, cmd: UpdateMemoryScoreCommand) -> Result<(), UpdateMemoryScoreError> {
        if !(0.0..=1.0).contains(&cmd.new_score) {
            return Err(UpdateMemoryScoreError::InvalidScore);
        }
        let entry = self
            .base
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
        self.base
            .persist(&event)
            .map_err(|_| UpdateMemoryScoreError::StorageError)?;
        Ok(())
    }
}
