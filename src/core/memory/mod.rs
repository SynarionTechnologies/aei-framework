//! Memory abstractions and utilities.

pub mod compactor;
pub mod events;
pub mod index;
pub mod retention;
pub mod store;

pub use compactor::{Compactor, NoopCompactor};
pub use events::*;
pub use index::{InMemoryIndex, MemoryIndex, SearchResult};
pub use retention::{RetentionAction, RetentionPolicy, TtlRetentionPolicy};
pub use store::{InMemoryStore, MemoryError, MemoryId, MemoryItem, MemoryStore, Result};
