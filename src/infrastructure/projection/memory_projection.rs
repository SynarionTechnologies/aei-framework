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

    /// Returns up to `limit` entries matching `event_type`, ordered by descending score.
    ///
    /// # Arguments
    ///
    /// * `event_type` - Type tag to filter entries by.
    /// * `limit` - Maximum number of entries to return.
    ///
    /// # Returns
    ///
    /// A vector of references to memory entries sorted from highest to lowest score.
    ///
    /// # Examples
    ///
    /// ```
    /// use aei_framework::domain::{MemoryEntry, MemoryEntryAdded, MemoryEvent};
    /// use aei_framework::infrastructure::projection::MemoryProjection;
    /// use chrono::Utc;
    /// use uuid::Uuid;
    ///
    /// let events = vec![MemoryEvent::MemoryEntryAdded(MemoryEntryAdded {
    ///     entry: MemoryEntry {
    ///         id: Uuid::new_v4(),
    ///         timestamp: Utc::now(),
    ///         event_type: "demo".into(),
    ///         payload: serde_json::json!({}),
    ///         score: 0.4,
    ///     },
    /// })];
    /// let projection = MemoryProjection::from_events(10, &events);
    /// let entries = projection.entries_by_event_type("demo", 5);
    /// assert_eq!(entries.len(), 1);
    /// ```
    #[must_use]
    pub fn entries_by_event_type(&self, event_type: &str, limit: usize) -> Vec<&MemoryEntry> {
        let mut entries: Vec<&MemoryEntry> = self
            .memory
            .entries
            .iter()
            .filter(|e| e.event_type == event_type)
            .collect();
        entries.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        entries.into_iter().take(limit).collect()
    }
}
