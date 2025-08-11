//! Adaptive memory aggregate storing past experiences as scored entries.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a memorized experience with an associated usefulness score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    /// Unique identifier of the memory entry.
    pub id: Uuid,
    /// Time when the experience occurred.
    pub timestamp: DateTime<Utc>,
    /// Arbitrary type tag describing the experience.
    pub event_type: String,
    /// User-defined payload holding the experience data.
    pub payload: serde_json::Value,
    /// Estimated usefulness of the experience in the range `[0.0, 1.0]`.
    pub score: f64,
}

/// Event emitted when a new memory entry is added.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntryAdded {
    /// The entry that was added.
    pub entry: MemoryEntry,
}

/// Event emitted when a memory entry is removed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntryRemoved {
    /// Identifier of the entry that was removed.
    pub entry_id: Uuid,
}

/// Event emitted when low scoring entries are pruned.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryPruned {
    /// Identifiers of the entries that were removed.
    pub removed_entries: Vec<Uuid>,
}

/// Event emitted when the usefulness score of an entry changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryScoreUpdated {
    /// Identifier of the updated entry.
    pub entry_id: Uuid,
    /// Previous score before the update.
    pub old_score: f64,
    /// New score after the update.
    pub new_score: f64,
}

/// Domain events for the [`AdaptiveMemory`] aggregate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryEvent {
    /// A new memory entry was recorded.
    MemoryEntryAdded(MemoryEntryAdded),
    /// An existing entry was removed.
    MemoryEntryRemoved(MemoryEntryRemoved),
    /// Multiple entries were pruned to respect capacity constraints.
    MemoryPruned(MemoryPruned),
    /// The score of an entry was updated.
    MemoryScoreUpdated(MemoryScoreUpdated),
}

/// Aggregate maintaining a bounded buffer of memory entries.
#[derive(Debug, Clone)]
pub struct AdaptiveMemory {
    /// Stored memory entries ordered by insertion time.
    pub entries: Vec<MemoryEntry>,
    /// Maximum number of entries retained in memory.
    pub max_size: usize,
}

impl AdaptiveMemory {
    /// Creates a new empty memory with the given capacity.
    #[must_use]
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_size,
        }
    }

    /// Rebuilds a memory instance by replaying past events.
    #[must_use]
    pub fn hydrate(max_size: usize, events: &[MemoryEvent]) -> Self {
        let mut memory = Self::new(max_size);
        for event in events {
            memory.apply(event);
        }
        memory
    }

    /// Applies a domain event to mutate the internal state.
    pub fn apply(&mut self, event: &MemoryEvent) {
        match event {
            MemoryEvent::MemoryEntryAdded(e) => self.apply_entry_added(e),
            MemoryEvent::MemoryEntryRemoved(e) => self.apply_entry_removed(e),
            MemoryEvent::MemoryPruned(e) => self.apply_pruned(e),
            MemoryEvent::MemoryScoreUpdated(e) => self.apply_score_updated(e),
        }
    }

    fn apply_entry_added(&mut self, event: &MemoryEntryAdded) {
        self.entries.push(event.entry.clone());
    }

    fn apply_entry_removed(&mut self, event: &MemoryEntryRemoved) {
        self.entries.retain(|e| e.id != event.entry_id);
    }

    fn apply_pruned(&mut self, event: &MemoryPruned) {
        self.entries
            .retain(|e| !event.removed_entries.contains(&e.id));
    }

    fn apply_score_updated(&mut self, event: &MemoryScoreUpdated) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == event.entry_id) {
            entry.score = event.new_score;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn hydrate_replays_events() {
        let id = Uuid::new_v4();
        let entry = MemoryEntry {
            id,
            timestamp: Utc::now(),
            event_type: "test".into(),
            payload: json!({"value": 1}),
            score: 0.5,
        };
        let events = vec![MemoryEvent::MemoryEntryAdded(MemoryEntryAdded { entry })];
        let memory = AdaptiveMemory::hydrate(10, &events);
        assert_eq!(memory.entries.len(), 1);
    }

    #[test]
    fn apply_entry_removed_removes_entry() {
        let id_to_remove = Uuid::new_v4();
        let id_to_keep = Uuid::new_v4();
        let mut memory = AdaptiveMemory {
            entries: vec![
                MemoryEntry {
                    id: id_to_remove,
                    timestamp: Utc::now(),
                    event_type: "a".into(),
                    payload: json!({}),
                    score: 0.1,
                },
                MemoryEntry {
                    id: id_to_keep,
                    timestamp: Utc::now(),
                    event_type: "b".into(),
                    payload: json!({}),
                    score: 0.2,
                },
            ],
            max_size: 10,
        };
        memory.apply(&MemoryEvent::MemoryEntryRemoved(MemoryEntryRemoved {
            entry_id: id_to_remove,
        }));
        assert!(memory.entries.iter().all(|e| e.id != id_to_remove));
        assert!(memory.entries.iter().any(|e| e.id == id_to_keep));
    }

    #[test]
    fn apply_pruned_removes_multiple_entries() {
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();
        let id3 = Uuid::new_v4();
        let mut memory = AdaptiveMemory {
            entries: vec![
                MemoryEntry {
                    id: id1,
                    timestamp: Utc::now(),
                    event_type: "a".into(),
                    payload: json!({}),
                    score: 0.1,
                },
                MemoryEntry {
                    id: id2,
                    timestamp: Utc::now(),
                    event_type: "b".into(),
                    payload: json!({}),
                    score: 0.2,
                },
                MemoryEntry {
                    id: id3,
                    timestamp: Utc::now(),
                    event_type: "c".into(),
                    payload: json!({}),
                    score: 0.3,
                },
            ],
            max_size: 10,
        };
        memory.apply(&MemoryEvent::MemoryPruned(MemoryPruned {
            removed_entries: vec![id1, id3],
        }));
        assert_eq!(memory.entries.len(), 1);
        assert_eq!(memory.entries[0].id, id2);
    }

    #[test]
    fn apply_score_updated_updates_score() {
        let id = Uuid::new_v4();
        let mut memory = AdaptiveMemory {
            entries: vec![MemoryEntry {
                id,
                timestamp: Utc::now(),
                event_type: "a".into(),
                payload: json!({}),
                score: 0.1,
            }],
            max_size: 10,
        };
        memory.apply(&MemoryEvent::MemoryScoreUpdated(MemoryScoreUpdated {
            entry_id: id,
            old_score: 0.1,
            new_score: 0.9,
        }));
        let entry = memory.entries.iter().find(|e| e.id == id).unwrap();
        assert_eq!(entry.score, 0.9);
    }
}
