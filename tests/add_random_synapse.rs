use std::path::PathBuf;

use aei_framework::{
    application::{
        AddRandomSynapseCommand, AddRandomSynapseError, AddRandomSynapseHandler, CommandHandler,
        QueryHandler, QueryResult,
    },
    commands::Command,
    core::Activation,
    infrastructure::FileEventStore,
    Query,
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use uuid::Uuid;

fn temp_path() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("aei_synapse_{}.log", Uuid::new_v4()));
    path
}

#[test]
fn add_random_synapse_via_handler() {
    let path = temp_path();
    let store = FileEventStore::new(path.clone());
    let mut cmd = CommandHandler::new(store).unwrap();
    let n1 = Uuid::new_v4();
    let n2 = Uuid::new_v4();
    cmd.handle(Command::AddNeuron {
        id: n1,
        activation: Activation::Identity,
    })
    .unwrap();
    cmd.handle(Command::AddNeuron {
        id: n2,
        activation: Activation::Identity,
    })
    .unwrap();
    drop(cmd);

    let store = FileEventStore::new(path.clone());
    let rng = ChaCha8Rng::seed_from_u64(7);
    let mut handler = AddRandomSynapseHandler::new(store, rng).unwrap();
    let syn_id = handler.handle(AddRandomSynapseCommand).unwrap();
    let syn = handler.network.synapses.get(&syn_id).unwrap();
    assert_ne!(syn.from, syn.to);
    assert!((-1.0..=1.0).contains(&syn.weight));

    let query = QueryHandler::new(&handler.network);
    match query.handle(Query::ListSynapses) {
        QueryResult::Synapses(list) => assert!(list.iter().any(|s| s.id == syn_id)),
        _ => panic!("unexpected query result"),
    }

    // Reload handler to ensure event persistence.
    let store = FileEventStore::new(path);
    let handler2 = AddRandomSynapseHandler::new(store, ChaCha8Rng::seed_from_u64(7)).unwrap();
    assert!(handler2.network.synapses.contains_key(&syn_id));
}

#[test]
fn fails_when_no_connection_possible() {
    let path = temp_path();
    let store = FileEventStore::new(path.clone());
    let mut cmd = CommandHandler::new(store).unwrap();
    let n1 = Uuid::new_v4();
    let n2 = Uuid::new_v4();
    cmd.handle(Command::AddNeuron {
        id: n1,
        activation: Activation::Identity,
    })
    .unwrap();
    cmd.handle(Command::AddNeuron {
        id: n2,
        activation: Activation::Identity,
    })
    .unwrap();
    let s1 = Uuid::new_v4();
    cmd.handle(Command::CreateSynapse {
        id: s1,
        from: n1,
        to: n2,
        weight: 0.5,
    })
    .unwrap();
    let s2 = Uuid::new_v4();
    cmd.handle(Command::CreateSynapse {
        id: s2,
        from: n2,
        to: n1,
        weight: -0.5,
    })
    .unwrap();
    drop(cmd);

    let store = FileEventStore::new(path);
    let mut handler = AddRandomSynapseHandler::new(store, ChaCha8Rng::seed_from_u64(9)).unwrap();
    let res = handler.handle(AddRandomSynapseCommand);
    assert!(matches!(
        res,
        Err(AddRandomSynapseError::NoAvailableConnection)
    ));
}

#[test]
fn fails_with_insufficient_neurons() {
    let path = temp_path();
    let store = FileEventStore::new(path);
    let mut handler = AddRandomSynapseHandler::new(store, ChaCha8Rng::seed_from_u64(1)).unwrap();
    let res = handler.handle(AddRandomSynapseCommand);
    assert!(matches!(res, Err(AddRandomSynapseError::NotEnoughNeurons)));
}
