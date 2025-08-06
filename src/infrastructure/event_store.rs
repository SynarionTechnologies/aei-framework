//! Append-only event storage.
//!
//! The default [`FileEventStore`] writes JSON encoded events to disk, one per
//! line. The log can later be replayed to rebuild the full network state.

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

use crate::events::Event;

/// Storage backend for domain events.
pub trait EventStore {
    /// The error type produced by this event store.
    type Error;
    /// Persist an event to the underlying storage.
    fn append(&mut self, event: &Event) -> Result<(), Self::Error>;
    /// Load all events in chronological order.
    fn load(&mut self) -> Result<Vec<Event>, Self::Error>;
}

/// JSON-lines file based implementation of [`EventStore`].
#[derive(Debug)]
pub struct FileEventStore {
    path: PathBuf,
}

impl FileEventStore {
    /// Creates a new store writing to the specified path.
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl EventStore for FileEventStore {
    type Error = io::Error;

    fn append(&mut self, event: &Event) -> Result<(), Self::Error> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        let json = serde_json::to_string(event).map_err(io::Error::other)?;
        writeln!(file, "{json}")
    }

    fn load(&mut self) -> Result<Vec<Event>, Self::Error> {
        let mut events = Vec::new();
        if !self.path.exists() {
            return Ok(events);
        }
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            let event: Event = serde_json::from_str(&line).map_err(io::Error::other)?;
            events.push(event);
        }
        Ok(events)
    }
}
