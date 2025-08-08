//! Read model reflecting the current adaptive memory state.

use uuid::Uuid;

use crate::domain::{AdaptiveMemory, MemoryEntry, MemoryEvent};

/// In-memory projection of the [`AdaptiveMemory`] aggregate.
#[derive(Debug)]
pub struct MemoryProjection {
    memory: AdaptiveMemory,
}

impl MemoryProjection {
    /// Builds the projection by replaying events.
    #[must_use]
    pub fn from_events(max_size: usize, events: &[MemoryEvent]) -> Self {
        Self {
            memory: AdaptiveMemory::hydrate(max_size, events),
        }
    }

    /// Applies a new memory event to update the projection.
    pub fn apply(&mut self, event: &MemoryEvent) {
        self.memory.apply(event);
    }

    /// Returns all memory entries currently stored.
    #[must_use]
    pub fn entries(&self) -> Vec<&MemoryEntry> {
        self.memory.entries.iter().collect()
    }

    /// Returns the entry with the specified identifier, if any.
    #[must_use]
    pub fn entry(&self, id: Uuid) -> Option<&MemoryEntry> {
        self.memory.entries.iter().find(|e| e.id == id)
    }

    /// Returns the top `limit` entries with the highest scores.
    #[must_use]
    pub fn top_entries(&self, limit: usize) -> Vec<&MemoryEntry> {
        let mut entries: Vec<&MemoryEntry> = self.memory.entries.iter().collect();
        entries.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        entries.into_iter().take(limit).collect()
    }
}
