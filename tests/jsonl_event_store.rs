use std::path::PathBuf;

use aei_framework::JsonlEventStore;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct TestEvent {
    id: u32,
}

fn temp_path() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("aei_jsonl_store_test_{}.log", Uuid::new_v4()));
    path
}

#[test]
fn generic_store_persists_custom_events() {
    let path = temp_path();
    let mut store = JsonlEventStore::<TestEvent>::new(path.clone());
    let first = TestEvent { id: 1 };
    let second = TestEvent { id: 2 };
    store.append(&first).expect("append first");
    store.append(&second).expect("append second");

    let mut store = JsonlEventStore::<TestEvent>::new(path.clone());
    let events = store.load().expect("load");
    assert_eq!(events, vec![first, second]);
    std::fs::remove_file(path).unwrap();
}
