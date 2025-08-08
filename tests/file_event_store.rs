use std::path::PathBuf;

use aei_framework::{Event, EventStore, FileEventStore};
use uuid::Uuid;

fn temp_path() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("aei_event_store_test_{}.log", Uuid::new_v4()));
    path
}

// Loading from a non-existent file should yield an empty event list.
#[test]
fn load_missing_file_returns_empty() {
    let path = temp_path();
    let mut store = FileEventStore::new(path.clone());
    let events = store.load().expect("load should succeed");
    assert!(events.is_empty());
    assert!(!path.exists());
    let _ = std::fs::remove_file(path);
}

// Appending events and reloading should preserve their order and contents.
#[test]
fn append_and_reload_preserves_sequence() {
    let path = temp_path();
    let mut store = FileEventStore::new(path.clone());
    let synapse_id = Uuid::new_v4();
    let from = Uuid::new_v4();
    let to = Uuid::new_v4();
    let first = Event::SynapseCreated {
        id: synapse_id,
        from,
        to,
        weight: 0.5,
    };
    let second = Event::SynapseRemoved { id: synapse_id };
    store.append(&first).expect("append first");
    store.append(&second).expect("append second");

    let mut store = FileEventStore::new(path.clone());
    let events = store.load().expect("reload should succeed");
    assert_eq!(events.len(), 2);
    match &events[0] {
        Event::SynapseCreated { id, from: f, to: t, weight } => {
            assert_eq!(*id, synapse_id);
            assert_eq!(*f, from);
            assert_eq!(*t, to);
            assert!((*weight - 0.5).abs() < f64::EPSILON);
        }
        e => panic!("unexpected first event {e:?}"),
    }
    match &events[1] {
        Event::SynapseRemoved { id } => assert_eq!(*id, synapse_id),
        e => panic!("unexpected second event {e:?}"),
    }
    std::fs::remove_file(path).unwrap();
}

