//! Commands operating on the adaptive memory aggregate.

use serde_json::Value;
use uuid::Uuid;

/// Add a new experience to memory.
#[derive(Debug, Clone)]
pub struct AddMemoryEntryCommand {
    /// Type tag describing the experience.
    pub event_type: String,
    /// Arbitrary payload representing the experience.
    pub payload: Value,
    /// Estimated usefulness in range `[0.0, 1.0]`.
    pub score: f64,
}

/// Remove a specific memory entry by identifier.
#[derive(Debug, Clone)]
pub struct RemoveMemoryEntryCommand {
    /// Identifier of the entry to remove.
    pub entry_id: Uuid,
}

/// Prune low scoring entries if the memory exceeds its capacity.
#[derive(Debug, Clone, Copy)]
pub struct PruneMemoryCommand;

/// Update the score of an existing memory entry.
#[derive(Debug, Clone)]
pub struct UpdateMemoryScoreCommand {
    /// Identifier of the entry to update.
    pub entry_id: Uuid,
    /// New normalized score.
    pub new_score: f64,
}
