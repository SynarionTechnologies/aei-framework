use std::path::PathBuf;

use aei_framework::{
    application::{CommandHandler, QueryHandler},
    commands::Command,
    core::Activation,
    infrastructure::FileEventStore,
    Query,
    QueryResult,
};
use uuid::Uuid;

fn temp_path() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("aei_es_{}.log", Uuid::new_v4()));
    path
}

#[test]
fn add_and_replay_neuron() {
    let path = temp_path();
    let store = FileEventStore::new(path.clone());
    let mut handler = CommandHandler::new(store).unwrap();
    let id = Uuid::new_v4();
    handler
        .handle(Command::AddNeuron {
            id,
            activation: Activation::Identity,
        })
        .unwrap();

    let query = QueryHandler::new(&handler.network);
    match query.handle(Query::GetNeuron { id }) {
        QueryResult::Neuron(Some(n)) => assert_eq!(n.id, id),
        _ => panic!("neuron not found"),
    }

    // Reload from the same event store and ensure the neuron persists.
    let store = FileEventStore::new(path);
    let handler = CommandHandler::new(store).unwrap();
    assert!(handler.network.neurons.get(&id).is_some());
}

#[test]
fn create_and_remove_synapse() {
    let path = temp_path();
    let store = FileEventStore::new(path);
    let mut handler = CommandHandler::new(store).unwrap();

    let n1 = Uuid::new_v4();
    let n2 = Uuid::new_v4();
    handler
        .handle(Command::AddNeuron {
            id: n1,
            activation: Activation::ReLU,
        })
        .unwrap();
    handler
        .handle(Command::AddNeuron {
            id: n2,
            activation: Activation::Tanh,
        })
        .unwrap();

    let syn_id = Uuid::new_v4();
    handler
        .handle(Command::CreateSynapse {
            id: syn_id,
            from: n1,
            to: n2,
            weight: 1.0,
        })
        .unwrap();
    assert!(handler.network.synapses.get(&syn_id).is_some());

    handler
        .handle(Command::RemoveSynapse { id: syn_id })
        .unwrap();
    assert!(handler.network.synapses.get(&syn_id).is_none());
}

