use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// Identifier used for memory items.
pub type MemoryId = Uuid;

/// Item persisted by a [`MemoryStore`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryItem {
    /// Unique identifier for the item.
    pub id: MemoryId,
    /// Textual content associated with the item.
    pub content: String,
    /// Timestamp when the item was created.
    pub timestamp: DateTime<Utc>,
}

impl MemoryItem {
    /// Creates a new memory item.
    ///
    /// # Examples
    /// ```
    /// use aei_framework::core::memory::MemoryItem;
    /// let item = MemoryItem::new("Hello");
    /// assert_eq!(item.content, "Hello");
    /// ```
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            content: content.into(),
            timestamp: Utc::now(),
        }
    }
}

/// Errors returned by memory operations.
#[derive(Debug, Error)]
pub enum MemoryError {
    /// The requested item does not exist.
    #[error("memory item not found")]
    NotFound,
    /// Generic storage error.
    #[error("storage error: {0}")]
    Storage(String),
}

/// Result type used by memory APIs.
pub type Result<T> = std::result::Result<T, MemoryError>;

/// Generic CRUD operations for memory items.
pub trait MemoryStore {
    /// Appends a new item and returns its identifier.
    fn append(&mut self, item: MemoryItem) -> Result<MemoryId>;
    /// Retrieves an item by identifier.
    fn get(&self, id: &MemoryId) -> Result<Option<MemoryItem>>;
    /// Deletes an item by identifier.
    fn delete(&mut self, id: &MemoryId) -> Result<()>;
}

/// In-memory [`MemoryStore`] implementation backed by a `HashMap`.
#[derive(Default)]
pub struct InMemoryStore {
    items: HashMap<MemoryId, MemoryItem>,
}

impl InMemoryStore {
    /// Creates a new empty store.
    pub fn new() -> Self {
        Self::default()
    }
}

impl MemoryStore for InMemoryStore {
    fn append(&mut self, item: MemoryItem) -> Result<MemoryId> {
        let id = item.id;
        self.items.insert(id, item);
        Ok(id)
    }

    fn get(&self, id: &MemoryId) -> Result<Option<MemoryItem>> {
        Ok(self.items.get(id).cloned())
    }

    fn delete(&mut self, id: &MemoryId) -> Result<()> {
        self.items
            .remove(id)
            .map(|_| ())
            .ok_or(MemoryError::NotFound)
    }
}
