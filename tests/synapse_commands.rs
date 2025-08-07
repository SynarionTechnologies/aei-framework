use std::path::PathBuf;

use aei_framework::{
    Activation, AddRandomSynapseCommand, AddRandomSynapseError, AddRandomSynapseHandler, Event,
    EventStore, FileEventStore, RandomNeuronAdded, RandomSynapseAdded, RandomSynapseRemoved,
    RemoveRandomSynapseCommand, RemoveRandomSynapseError, RemoveRandomSynapseHandler,
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use uuid::Uuid;

fn temp_path() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("aei_synapse_test_{}.log", Uuid::new_v4()));
    path
}

fn seed_two_neurons(store: &mut FileEventStore, n1: Uuid, n2: Uuid) {
    let events = [
        Event::RandomNeuronAdded(RandomNeuronAdded {
            neuron_id: n1,
            activation: Activation::Identity,
        }),
        Event::RandomNeuronAdded(RandomNeuronAdded {
            neuron_id: n2,
            activation: Activation::Identity,
        }),
    ];
    for e in &events {
        store.append(e).unwrap();
    }
}

fn seed_synapse(store: &mut FileEventStore, id: Uuid, from: Uuid, to: Uuid) {
    let event = Event::SynapseCreated {
        id,
        from,
        to,
        weight: 1.0,
    };
    store.append(&event).unwrap();
}

#[test]
fn add_random_synapse_appends_event() {
    let path = temp_path();
    let mut store = FileEventStore::new(path.clone());
    let n1 = Uuid::new_v4();
    let n2 = Uuid::new_v4();
    seed_two_neurons(&mut store, n1, n2);

    let rng = ChaCha8Rng::seed_from_u64(1);
    let mut handler = AddRandomSynapseHandler::new(store, rng).unwrap();
    let syn_id = handler.handle(AddRandomSynapseCommand).unwrap();
    assert!(handler.network.synapses.contains_key(&syn_id));

    let mut store = handler.store;
    let events = store.load().unwrap();
    match events.last().unwrap() {
        Event::RandomSynapseAdded(RandomSynapseAdded { synapse_id, .. }) => {
            assert_eq!(*synapse_id, syn_id)
        }
        e => panic!("unexpected event {e:?}"),
    }
}

#[test]
fn add_random_synapse_requires_two_neurons() {
    let path = temp_path();
    let mut store = FileEventStore::new(path.clone());
    let n1 = Uuid::new_v4();
    let event = Event::RandomNeuronAdded(RandomNeuronAdded {
        neuron_id: n1,
        activation: Activation::Identity,
    });
    store.append(&event).unwrap();

    let rng = ChaCha8Rng::seed_from_u64(2);
    let mut handler = AddRandomSynapseHandler::new(store, rng).unwrap();
    let res = handler.handle(AddRandomSynapseCommand);
    assert!(matches!(res, Err(AddRandomSynapseError::NotEnoughNeurons)));
}

#[test]
fn add_random_synapse_errors_when_no_connection_available() {
    let path = temp_path();
    let mut store = FileEventStore::new(path.clone());
    let n1 = Uuid::new_v4();
    let n2 = Uuid::new_v4();
    seed_two_neurons(&mut store, n1, n2);

    let rng = ChaCha8Rng::seed_from_u64(3);
    let mut handler = AddRandomSynapseHandler::new(store, rng).unwrap();
    handler.handle(AddRandomSynapseCommand).unwrap();
    handler.handle(AddRandomSynapseCommand).unwrap();
    let res = handler.handle(AddRandomSynapseCommand);
    assert!(matches!(
        res,
        Err(AddRandomSynapseError::NoAvailableConnection)
    ));
}

#[test]
fn remove_random_synapse_appends_event() {
    let path = temp_path();
    let mut store = FileEventStore::new(path.clone());
    let n1 = Uuid::new_v4();
    let n2 = Uuid::new_v4();
    seed_two_neurons(&mut store, n1, n2);
    let syn = Uuid::new_v4();
    seed_synapse(&mut store, syn, n1, n2);

    let rng = ChaCha8Rng::seed_from_u64(4);
    let mut handler = RemoveRandomSynapseHandler::new(store, rng).unwrap();
    let removed_id = handler
        .handle(RemoveRandomSynapseCommand)
        .expect("synapse removed");
    assert!(!handler.network.synapses.contains_key(&removed_id));

    let mut store = handler.store;
    let events = store.load().unwrap();
    match events.last().unwrap() {
        Event::RandomSynapseRemoved(RandomSynapseRemoved { synapse_id }) => {
            assert_eq!(*synapse_id, removed_id)
        }
        e => panic!("unexpected event {e:?}"),
    }
}

#[test]
fn remove_random_synapse_errors_when_empty() {
    let path = temp_path();
    let store = FileEventStore::new(path);
    let rng = ChaCha8Rng::seed_from_u64(5);
    let mut handler = RemoveRandomSynapseHandler::new(store, rng).unwrap();
    let res = handler.handle(RemoveRandomSynapseCommand);
    assert!(matches!(
        res,
        Err(RemoveRandomSynapseError::NoSynapseAvailable)
    ));
}

#[test]
fn remove_random_synapse_event_replay() {
    let path = temp_path();
    let mut store = FileEventStore::new(path.clone());
    let n1 = Uuid::new_v4();
    let n2 = Uuid::new_v4();
    seed_two_neurons(&mut store, n1, n2);
    let syn = Uuid::new_v4();
    seed_synapse(&mut store, syn, n1, n2);

    let rng = ChaCha8Rng::seed_from_u64(6);
    let mut handler = RemoveRandomSynapseHandler::new(store, rng).unwrap();
    let removed_id = handler
        .handle(RemoveRandomSynapseCommand)
        .expect("synapse removed");

    let store = handler.store;
    let mut replay_store = store;
    let events = replay_store.load().unwrap();
    let net = aei_framework::DomainNetwork::hydrate(&events);
    assert!(!net.synapses.contains_key(&removed_id));

    let projection =
        aei_framework::infrastructure::projection::NetworkProjection::from_events(&events);
    assert!(projection.synapses().is_empty());
}
