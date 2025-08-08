//! Append-only store for [`MemoryEvent`](crate::domain::MemoryEvent).

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

use crate::domain::MemoryEvent;

/// Storage backend dedicated to memory events.
pub trait MemoryEventStore {
    /// Error type returned by the store.
    type Error;
    /// Persist an event to the underlying storage.
    fn append(&mut self, event: &MemoryEvent) -> Result<(), Self::Error>;
    /// Load all stored events in chronological order.
    fn load(&mut self) -> Result<Vec<MemoryEvent>, Self::Error>;
}

/// JSON-lines file based implementation of [`MemoryEventStore`].
#[derive(Debug)]
pub struct FileMemoryEventStore {
    path: PathBuf,
}

impl FileMemoryEventStore {
    /// Creates a new store writing to the given file path.
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl MemoryEventStore for FileMemoryEventStore {
    type Error = io::Error;

    fn append(&mut self, event: &MemoryEvent) -> Result<(), Self::Error> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        let json = serde_json::to_string(event).map_err(io::Error::other)?;
        writeln!(file, "{json}")
    }

    fn load(&mut self) -> Result<Vec<MemoryEvent>, Self::Error> {
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
            let event: MemoryEvent = serde_json::from_str(&line).map_err(io::Error::other)?;
            events.push(event);
        }
        Ok(events)
    }
}
