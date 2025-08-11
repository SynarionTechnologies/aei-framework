//! Infrastructure components such as persistence adapters.

mod event_store;
mod jsonl_event_store;
mod memory_event_store;
pub mod projection;

pub use event_store::{EventStore, FileEventStore};
pub use jsonl_event_store::JsonlEventStore;
pub use memory_event_store::{FileMemoryEventStore, MemoryEventStore};
