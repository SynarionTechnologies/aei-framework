# Interfaces mémoire

Le module `core::memory` fournit plusieurs abstractions génériques pour la gestion de mémoire :

- `MemoryStore` pour la persistance CRUD des `MemoryItem`.
- `MemoryIndex` pour la recherche vectorielle.
- `RetentionPolicy` pour décider si les éléments sont conservés, archivés ou supprimés.
- `Compactor` pour réduire l'espace de stockage.

## Exemple

```rust
use aei_framework::core::memory::{InMemoryStore, MemoryItem, MemoryStore};

let mut store = InMemoryStore::new();
let id = store.append(MemoryItem::new("Bonjour"))?;
let item = store.get(&id)?.unwrap();
```
