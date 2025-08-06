use std::path::PathBuf;

use aei_framework::{
    application::{CommandHandler, QueryHandler},
    commands::Command,
    core::Activation,
    events::Event,
    infrastructure::FileEventStore,
    queries::Query,
    EventStore, QueryResult,
};
use uuid::Uuid;

fn temp_path() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("aei_cqrs_{}.log", Uuid::new_v4()));
    path
}

// Verify command->event->query roundtrip.
#[test]
fn command_query_roundtrip() {
    let path = temp_path();
    let store = FileEventStore::new(path.clone());
    let mut cmd = CommandHandler::new(store).unwrap();

    let id = Uuid::new_v4();
    cmd.handle(Command::AddNeuron {
        id,
        activation: Activation::Identity,
    })
    .unwrap();

    let query = QueryHandler::new(&cmd.network);
    match query.handle(Query::GetNeuron { id }) {
        QueryResult::Neuron(Some(n)) => assert_eq!(n.id, id),
        _ => panic!("neuron not found"),
    }
}

// Ensure events are loaded in the order they were appended.
#[test]
fn event_store_preserves_order() {
    let path = temp_path();
    let mut store = FileEventStore::new(path.clone());
    let id = Uuid::new_v4();
    store
        .append(&Event::NeuronAdded {
            id,
            activation: Activation::Identity,
        })
        .unwrap();
    store.append(&Event::NeuronRemoved { id }).unwrap();

    let mut store = FileEventStore::new(path);
    let events = store.load().unwrap();
    assert!(matches!(events[0], Event::NeuronAdded { id: x, .. } if x == id));
    assert!(matches!(events[1], Event::NeuronRemoved { id: x } if x == id));
}
