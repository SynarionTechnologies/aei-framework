//! Infrastructure components such as persistence adapters.

mod event_store;
pub mod projection;

pub use event_store::{EventStore, FileEventStore};
