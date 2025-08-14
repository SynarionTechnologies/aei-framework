use aei_memory::{InMemoryIndex, InMemoryStore, MemoryIndex, MemoryItem, MemoryStore};

#[test]
fn store_crud_and_search() {
    let mut store = InMemoryStore::new();
    let mut index = InMemoryIndex::new();

    let item = MemoryItem::new("Hello World");
    let id = store.append(item.clone()).unwrap();
    index.add_embedding(&id, vec![1.0, 0.0, 0.0]).unwrap();

    let fetched = store.get(&id).unwrap().unwrap();
    assert_eq!(fetched.content, "Hello World");

    let results = index.search(vec![1.0, 0.0, 0.0], 1).unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, id);

    store.delete(&id).unwrap();
    assert!(store.get(&id).unwrap().is_none());
}
