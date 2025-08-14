# Scheduler

The scheduler module registers tasks that run at fixed intervals. Tasks execute when `tick()` is called.

```rust
use aei_runtime::scheduler::{InMemoryScheduler, Scheduler};
use std::time::Duration;

let mut sched = InMemoryScheduler::new();
sched.schedule(Duration::from_secs(1), Box::new(|| println!("tick")));
sched.tick();
```
