# Scheduler

Le module Scheduler enregistre des tâches exécutées à intervalles fixes. Les tâches sont lancées lorsque `tick()` est appelé.

```rust
use aei_runtime::scheduler::{InMemoryScheduler, Scheduler};
use std::time::Duration;

let mut sched = InMemoryScheduler::new();
sched.schedule(Duration::from_secs(1), Box::new(|| println!("tick")));
sched.tick();
```
