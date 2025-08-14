use serde::{Deserialize, Serialize};

use super::store::MemoryId;

/// Emitted when a new memory item is appended.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryItemAppended {
    /// Identifier of the appended item.
    pub id: MemoryId,
}

/// Emitted when an existing memory item is updated.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryItemUpdated {
    /// Identifier of the updated item.
    pub id: MemoryId,
}

/// Emitted when a memory item is deleted.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryItemDeleted {
    /// Identifier of the removed item.
    pub id: MemoryId,
}

/// Emitted when a memory item is archived.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryItemArchived {
    /// Identifier of the archived item.
    pub id: MemoryId,
}

/// Emitted when memory compaction has occurred.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryCompacted;

/// Emitted when the retention policy changes.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryRetentionPolicyChanged;

/// Emitted when an index has been rebuilt.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexRebuilt;
