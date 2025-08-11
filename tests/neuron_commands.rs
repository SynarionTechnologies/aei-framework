use std::path::PathBuf;

use aei_framework::{
    AddRandomNeuronCommand, AddRandomNeuronHandler, Event, FileEventStore, RandomNeuronAdded,
    RemoveRandomNeuronCommand, RemoveRandomNeuronError, RemoveRandomNeuronHandler,
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use uuid::Uuid;

fn temp_path() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("aei_neuron_test_{}.log", Uuid::new_v4()));
    path
}

#[test]
fn add_random_neuron_appends_event() {
    let path = temp_path();
    let store = FileEventStore::new(path.clone());
    let rng = ChaCha8Rng::seed_from_u64(42);
    let mut handler = AddRandomNeuronHandler::new(store, rng).unwrap();

    let id = handler
        .handle(AddRandomNeuronCommand)
        .expect("neuron added");
    assert!(handler.base.network.neurons.contains_key(&id));

    let mut store = FileEventStore::new(path);
    let events = store.load().unwrap();
    match &events[0] {
        Event::RandomNeuronAdded(RandomNeuronAdded { neuron_id, .. }) => {
            assert_eq!(*neuron_id, id)
        }
        e => panic!("unexpected event {e:?}"),
    }
}

#[test]
fn remove_random_neuron_removes_synapses() {
    let path = temp_path();
    let store = FileEventStore::new(path.clone());
    let rng = ChaCha8Rng::seed_from_u64(1);
    let mut add = AddRandomNeuronHandler::new(store, rng).unwrap();
    let id1 = add.handle(AddRandomNeuronCommand).unwrap();
    let id2 = add.handle(AddRandomNeuronCommand).unwrap();

    // Manually connect the two neurons.
    let syn_id = Uuid::new_v4();
    let event = Event::SynapseCreated {
        id: syn_id,
        from: id1,
        to: id2,
        weight: 1.0,
    };
    add.base.store.append(&event).unwrap();
    add.base.network.apply(&event);

    // Recreate handler to ensure state is loaded from events.
    let store = add.base.store; // reuse path via move
    let rng = ChaCha8Rng::seed_from_u64(2);
    let mut remove = RemoveRandomNeuronHandler::new(store, rng).unwrap();
    let removed_id = remove
        .handle(RemoveRandomNeuronCommand)
        .expect("removed neuron");
    assert!(!remove.base.network.neurons.contains_key(&removed_id));
    assert!(remove.base.network.synapses.is_empty());
}

#[test]
fn remove_random_neuron_errors_when_empty() {
    let path = temp_path();
    let store = FileEventStore::new(path);
    let rng = ChaCha8Rng::seed_from_u64(7);
    let mut handler = RemoveRandomNeuronHandler::new(store, rng).unwrap();
    let res = handler.handle(RemoveRandomNeuronCommand);
    assert!(matches!(
        res,
        Err(RemoveRandomNeuronError::NoNeuronAvailable)
    ));
}

#[test]
fn event_replay_reconstructs_state() {
    let path = temp_path();
    let store = FileEventStore::new(path.clone());
    let rng = ChaCha8Rng::seed_from_u64(5);
    let mut add = AddRandomNeuronHandler::new(store, rng).unwrap();
    let id = add.handle(AddRandomNeuronCommand).unwrap();

    let store = add.base.store; // move
    let rng = ChaCha8Rng::seed_from_u64(6);
    let mut remove = RemoveRandomNeuronHandler::new(store, rng).unwrap();
    remove.handle(RemoveRandomNeuronCommand).unwrap();

    // Load all events and rebuild network
    let store = remove.base.store; // move
    let mut replay_store = store; // same path
    let events = replay_store.load().unwrap();
    let net = aei_framework::DomainNetwork::hydrate(&events);
    assert!(!net.neurons.contains_key(&id));
}
