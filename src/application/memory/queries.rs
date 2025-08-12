//! Queries against the adaptive memory projection.

use uuid::Uuid;

/// Read-side memory queries.
#[derive(Debug, Clone)]
pub enum MemoryQuery {
    /// Retrieve all memory entries.
    GetMemoryState,
    /// Retrieve the top `limit` entries by score.
    GetTopEntries { limit: usize },
    /// Retrieve up to `limit` entries matching `event_type`.
    GetByEventType { event_type: String, limit: usize },
    /// Retrieve a single entry by identifier.
    GetEntryById { id: Uuid },
}
