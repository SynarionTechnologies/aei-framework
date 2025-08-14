//! Agent runtime responsible for orchestrating and scheduling
//! Autonomous Conscious Units (ACUs) within the framework.

pub mod event_bus;
pub mod scheduler;

pub use event_bus::{EventBus, InMemoryEventBus};
pub use scheduler::{InMemoryScheduler, Scheduler};
