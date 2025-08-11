//! Generic JSON-lines event store.
//!
//! `JsonlEventStore` persists each event as a single line of JSON.
//! It accepts any event type that implements [`Serialize`] and [`DeserializeOwned`].

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::marker::PhantomData;
use std::path::PathBuf;

use serde::{de::DeserializeOwned, Serialize};

/// Append-only storage backed by a JSON Lines file.
///
/// Each event is serialized to JSON and written on its own line. The store
/// can later be replayed to rebuild application state.
///
/// # Examples
///
/// ```
/// use aei_framework::infrastructure::JsonlEventStore;
/// use serde::{Deserialize, Serialize};
/// use std::path::PathBuf;
///
/// #[derive(Debug, Serialize, Deserialize, PartialEq)]
/// struct MyEvent {
///     value: u32,
/// }
///
/// let path = PathBuf::from("events.log");
/// let mut store = JsonlEventStore::<MyEvent>::new(path.clone());
/// store.append(&MyEvent { value: 42 }).unwrap();
/// let events = store.load().unwrap();
/// assert_eq!(events, vec![MyEvent { value: 42 }]);
/// std::fs::remove_file(path).unwrap();
/// ```
#[derive(Debug)]
pub struct JsonlEventStore<T> {
    path: PathBuf,
    _marker: PhantomData<T>,
}

impl<T> JsonlEventStore<T> {
    /// Creates a new store writing to the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - Location of the JSON Lines file.
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            _marker: PhantomData,
        }
    }
}

impl<T> JsonlEventStore<T>
where
    T: Serialize + DeserializeOwned,
{
    /// Persist an event to the underlying storage.
    ///
    /// # Arguments
    ///
    /// * `event` - The event to append.
    ///
    /// # Errors
    ///
    /// Returns [`io::Error`] if the event cannot be serialized or written.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aei_framework::infrastructure::JsonlEventStore;
    /// # use serde::{Deserialize, Serialize};
    /// # use std::path::PathBuf;
    /// # #[derive(Serialize, Deserialize)]
    /// # struct MyEvent { value: u32 }
    /// # let path = PathBuf::from("append.log");
    /// # let mut store = JsonlEventStore::<MyEvent>::new(path.clone());
    /// store.append(&MyEvent { value: 7 }).unwrap();
    /// # std::fs::remove_file(path).unwrap();
    /// ```
    pub fn append(&mut self, event: &T) -> Result<(), io::Error> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        let json = serde_json::to_string(event).map_err(io::Error::other)?;
        writeln!(file, "{json}")
    }

    /// Load all events in chronological order.
    ///
    /// # Returns
    ///
    /// A vector containing the deserialized events.
    ///
    /// # Errors
    ///
    /// Returns [`io::Error`] if the file cannot be read or an event fails to
    /// deserialize.
    ///
    /// # Examples
    ///
    /// ```
    /// # use aei_framework::infrastructure::JsonlEventStore;
    /// # use serde::{Deserialize, Serialize};
    /// # use std::path::PathBuf;
    /// # #[derive(Debug, Serialize, Deserialize, PartialEq)]
    /// # struct MyEvent { value: u32 }
    /// # let path = PathBuf::from("load.log");
    /// # let mut store = JsonlEventStore::<MyEvent>::new(path.clone());
    /// store.append(&MyEvent { value: 1 }).unwrap();
    /// let events = store.load().unwrap();
    /// assert_eq!(events, vec![MyEvent { value: 1 }]);
    /// # std::fs::remove_file(path).unwrap();
    /// ```
    pub fn load(&mut self) -> Result<Vec<T>, io::Error> {
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
            let event: T = serde_json::from_str(&line).map_err(io::Error::other)?;
            events.push(event);
        }
        Ok(events)
    }
}
