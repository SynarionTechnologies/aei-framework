use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use aei_framework::core::scheduler::{InMemoryScheduler, Scheduler};

#[test]
fn scheduled_task_runs_on_tick() {
    let mut scheduler = InMemoryScheduler::new();
    let counter = Arc::new(AtomicUsize::new(0));
    let c = Arc::clone(&counter);
    scheduler.schedule(
        Duration::from_millis(0),
        Box::new(move || {
            c.fetch_add(1, Ordering::SeqCst);
        }),
    );
    scheduler.tick();
    assert_eq!(counter.load(Ordering::SeqCst), 1);
}
