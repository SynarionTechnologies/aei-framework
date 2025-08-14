# Memory Interfaces

The `core::memory` module defines several abstractions for generic memory management:

- `MemoryStore` for CRUD persistence of `MemoryItem` objects.
- `MemoryIndex` for vector-based search.
- `RetentionPolicy` for deciding whether items are kept, archived, or deleted.
- `Compactor` for reducing storage footprint.

## Example

```rust
use aei_framework::core::memory::{InMemoryStore, MemoryItem, MemoryStore};

let mut store = InMemoryStore::new();
let id = store.append(MemoryItem::new("Hello"))?;
let item = store.get(&id)?.unwrap();
```
