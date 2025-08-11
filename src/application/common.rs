//! Common base struct for network command handlers leveraging randomness.
//!
//! This base aggregates an event store, the hydrated network state, and a
//! random number generator. It provides a constructor that loads events from
//! the store and rebuilds the network so handlers only need to focus on their
//! specific logic.
//!
//! # Examples
//! ```
//! use aei_framework::{application::NetworkHandlerBase, FileEventStore};
//! use rand::thread_rng;
//! use std::path::PathBuf;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let store = FileEventStore::new(PathBuf::from("events.log"));
//! let _base = NetworkHandlerBase::new(store, thread_rng())?;
//! # Ok(()) }
//! ```
use rand::Rng;

use crate::domain::Network;
use crate::infrastructure::EventStore;

/// Shared state for handlers operating on a [`Network`] with randomness.
pub struct NetworkHandlerBase<S: EventStore, R: Rng> {
    /// Event store used for persistence.
    pub store: S,
    /// Current network state derived from applied events.
    pub network: Network,
    /// Random number generator.
    pub rng: R,
}

impl<S: EventStore, R: Rng> NetworkHandlerBase<S, R> {
    /// Loads events from the store and initializes the base handler.
    pub fn new(mut store: S, rng: R) -> Result<Self, S::Error> {
        let events = store.load()?;
        let network = Network::hydrate(&events);
        Ok(Self {
            store,
            network,
            rng,
        })
    }
}
