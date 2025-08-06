//! Infrastructure components such as persistence adapters.

mod event_store;

pub use event_store::{EventStore, FileEventStore};
