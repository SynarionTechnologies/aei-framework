use std::path::PathBuf;

use aei_framework::{
    Activation, Command, CommandHandler, DomainNetwork, Event, FileEventStore, NeuronAdded,
    NeuronRemoved,
};
use uuid::Uuid;

fn temp_path() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("aei_neuron_lifecycle_{}.log", Uuid::new_v4()));
    path
}

#[test]
fn create_neuron_appends_event() {
    let path = temp_path();
    let store = FileEventStore::new(path.clone());
    let mut handler = CommandHandler::new(store).unwrap();

    let id = Uuid::new_v4();
    handler
        .handle(Command::CreateNeuron {
            id,
            activation: Activation::Identity,
        })
        .unwrap();
    assert!(handler.network.neurons.contains_key(&id));

    let mut store = FileEventStore::new(path);
    let events = store.load().unwrap();
    match events.last().unwrap() {
        Event::NeuronAdded(NeuronAdded { neuron_id, .. }) => assert_eq!(*neuron_id, id),
        e => panic!("unexpected event {e:?}"),
    }
}

#[test]
fn remove_neuron_removes_synapses() {
    let path = temp_path();
    let store = FileEventStore::new(path.clone());
    let mut handler = CommandHandler::new(store).unwrap();

    let id1 = Uuid::new_v4();
    let id2 = Uuid::new_v4();
    handler
        .handle(Command::CreateNeuron {
            id: id1,
            activation: Activation::Identity,
        })
        .unwrap();
    handler
        .handle(Command::CreateNeuron {
            id: id2,
            activation: Activation::Identity,
        })
        .unwrap();

    let syn_id = Uuid::new_v4();
    handler
        .handle(Command::CreateSynapse {
            id: syn_id,
            from: id1,
            to: id2,
            weight: 1.0,
        })
        .unwrap();
    assert!(handler.network.synapses.contains_key(&syn_id));

    handler.handle(Command::RemoveNeuron { id: id1 }).unwrap();
    assert!(!handler.network.neurons.contains_key(&id1));
    assert!(handler.network.synapses.is_empty());

    let mut store = FileEventStore::new(path);
    let events = store.load().unwrap();
    match events.last().unwrap() {
        Event::NeuronRemoved(NeuronRemoved { neuron_id }) => assert_eq!(*neuron_id, id1),
        e => panic!("unexpected event {e:?}"),
    }
}

#[test]
fn event_replay_reconstructs_state() {
    let path = temp_path();
    let store = FileEventStore::new(path.clone());
    let mut handler = CommandHandler::new(store).unwrap();

    let id = Uuid::new_v4();
    handler
        .handle(Command::CreateNeuron {
            id,
            activation: Activation::Identity,
        })
        .unwrap();
    handler.handle(Command::RemoveNeuron { id }).unwrap();

    let mut reader = FileEventStore::new(path);
    let events = reader.load().unwrap();
    let net = DomainNetwork::hydrate(&events);
    assert!(!net.neurons.contains_key(&id));
}
