//! Task scheduling utilities.

use std::time::{Duration, Instant};

/// Schedules recurring tasks.
///
/// # Examples
/// ```
/// use aei_framework::core::scheduler::{InMemoryScheduler, Scheduler};
/// use std::sync::atomic::{AtomicUsize, Ordering};
/// use std::sync::Arc;
/// use std::time::Duration;
/// let mut sched = InMemoryScheduler::new();
/// let counter = Arc::new(AtomicUsize::new(0));
/// let c = Arc::clone(&counter);
/// sched.schedule(Duration::from_millis(0), Box::new(move || {
///     c.fetch_add(1, Ordering::SeqCst);
/// }));
/// sched.tick();
/// assert_eq!(counter.load(Ordering::SeqCst), 1);
/// ```
pub trait Scheduler {
    /// Schedules a task to run every `interval`.
    fn schedule(&mut self, interval: Duration, task: Box<dyn FnMut() + Send>);
    /// Executes due tasks.
    fn tick(&mut self);
}

/// Entry in the scheduler's task list.
type Task = (Instant, Duration, Box<dyn FnMut() + Send>);

/// In-memory scheduler running tasks on manual ticks.
#[derive(Default)]
pub struct InMemoryScheduler {
    tasks: Vec<Task>,
}

impl InMemoryScheduler {
    /// Creates an empty scheduler.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Scheduler for InMemoryScheduler {
    fn schedule(&mut self, interval: Duration, task: Box<dyn FnMut() + Send>) {
        self.tasks.push((Instant::now() + interval, interval, task));
    }

    fn tick(&mut self) {
        let now = Instant::now();
        for (next_run, interval, task) in &mut self.tasks {
            if now >= *next_run {
                task();
                *next_run = now + *interval;
            }
        }
    }
}
