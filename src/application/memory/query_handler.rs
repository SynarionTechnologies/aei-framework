//! Handles read-side memory queries using the projection.

use uuid::Uuid;

use crate::domain::MemoryEntry;
use crate::infrastructure::projection::MemoryProjection;

use super::queries::MemoryQuery;

/// Result of executing a [`MemoryQuery`].
pub enum MemoryQueryResult<'a> {
    /// Full memory state.
    Entries(Vec<&'a MemoryEntry>),
    /// Subset of entries.
    TopEntries(Vec<&'a MemoryEntry>),
    /// Entries filtered by event type.
    EntriesByEventType(Vec<&'a MemoryEntry>),
    /// Single entry lookup.
    Entry(Option<&'a MemoryEntry>),
}

/// Provides read-only access to the adaptive memory.
pub struct MemoryQueryHandler<'a> {
    projection: &'a MemoryProjection,
}

impl<'a> MemoryQueryHandler<'a> {
    /// Creates a new query handler from the projection.
    pub fn new(projection: &'a MemoryProjection) -> Self {
        Self { projection }
    }

    /// Executes a memory query.
    pub fn handle(&self, query: MemoryQuery) -> MemoryQueryResult<'a> {
        match query {
            MemoryQuery::GetMemoryState => MemoryQueryResult::Entries(self.projection.entries()),
            MemoryQuery::GetTopEntries { limit } => {
                MemoryQueryResult::TopEntries(self.projection.top_entries(limit))
            }
            MemoryQuery::GetByEventType { event_type, limit } => {
                MemoryQueryResult::EntriesByEventType(
                    self.projection.entries_by_event_type(&event_type, limit),
                )
            }
            MemoryQuery::GetEntryById { id } => MemoryQueryResult::Entry(self.projection.entry(id)),
        }
    }

    /// Convenience method to fetch an entry directly.
    #[must_use]
    pub fn entry(&self, id: Uuid) -> Option<&'a MemoryEntry> {
        self.projection.entry(id)
    }
}
