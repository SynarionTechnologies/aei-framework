use crossbeam_channel::{unbounded, Receiver, Sender};

/// Publish/subscribe event bus.
///
/// # Examples
/// ```
/// use aei_runtime::event_bus::{EventBus, InMemoryEventBus};
/// let mut bus: InMemoryEventBus<u32> = InMemoryEventBus::new();
/// let rx = bus.subscribe();
/// bus.publish(1);
/// assert_eq!(rx.recv().unwrap(), 1);
/// ```
pub trait EventBus<T: Clone + Send + 'static> {
    /// Publishes an event to all subscribers.
    fn publish(&self, event: T);
    /// Subscribes to events, returning a receiver channel.
    fn subscribe(&mut self) -> Receiver<T>;
}

/// In-memory implementation of [`EventBus`].
pub struct InMemoryEventBus<T: Clone + Send + 'static> {
    subscribers: Vec<Sender<T>>,
}

impl<T: Clone + Send + 'static> InMemoryEventBus<T> {
    /// Creates a new empty bus.
    pub fn new() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }
}

impl<T: Clone + Send + 'static> EventBus<T> for InMemoryEventBus<T> {
    fn publish(&self, event: T) {
        for sub in &self.subscribers {
            let _ = sub.send(event.clone());
        }
    }

    fn subscribe(&mut self) -> Receiver<T> {
        let (tx, rx) = unbounded();
        self.subscribers.push(tx);
        rx
    }
}
