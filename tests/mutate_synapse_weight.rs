use std::path::PathBuf;

use aei_framework::{
    application::{
        MutateRandomSynapseWeightCommand, MutateRandomSynapseWeightError,
        MutateRandomSynapseWeightHandler, Query, QueryHandler, QueryResult,
    },
    domain::{Event, RandomNeuronAdded, RandomSynapseAdded, SynapseWeightMutated},
    infrastructure::{projection::NetworkProjection, FileEventStore},
    Activation,
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use uuid::Uuid;

fn temp_path() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("aei_mutate_test_{}.log", Uuid::new_v4()));
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
    let event = Event::RandomSynapseAdded(RandomSynapseAdded {
        synapse_id: id,
        from,
        to,
        weight: 1.0,
    });
    store.append(&event).unwrap();
}

#[test]
fn mutate_synapse_weight_appends_event() {
    let path = temp_path();
    let mut store = FileEventStore::new(path.clone());
    let n1 = Uuid::new_v4();
    let n2 = Uuid::new_v4();
    seed_two_neurons(&mut store, n1, n2);
    let syn_id = Uuid::new_v4();
    seed_synapse(&mut store, syn_id, n1, n2);

    let rng = ChaCha8Rng::seed_from_u64(7);
    let mut handler = MutateRandomSynapseWeightHandler::new(store, rng).unwrap();
    let mutated_id = handler
        .handle(MutateRandomSynapseWeightCommand { std_dev: 0.5 })
        .unwrap();
    assert_eq!(mutated_id, syn_id);

    let mut store = handler.store;
    let events = store.load().unwrap();
    match events.last().unwrap() {
        Event::SynapseWeightMutated(SynapseWeightMutated {
            synapse_id,
            old_weight,
            new_weight,
        }) => {
            assert_eq!(*synapse_id, syn_id);
            assert_eq!(*old_weight, 1.0);
            assert_ne!(*new_weight, *old_weight);
            assert_eq!(
                handler.network.synapses.get(synapse_id).unwrap().weight,
                *new_weight
            );
        }
        e => panic!("unexpected event {e:?}"),
    }
}

#[test]
fn mutate_synapse_weight_errors_when_empty() {
    let path = temp_path();
    let store = FileEventStore::new(path);
    let rng = ChaCha8Rng::seed_from_u64(8);
    let mut handler = MutateRandomSynapseWeightHandler::new(store, rng).unwrap();
    let res = handler.handle(MutateRandomSynapseWeightCommand { std_dev: 0.1 });
    assert!(matches!(
        res,
        Err(MutateRandomSynapseWeightError::NoSynapseAvailable)
    ));
}

#[test]
fn mutate_synapse_weight_event_replay() {
    let path = temp_path();
    let mut store = FileEventStore::new(path.clone());
    let n1 = Uuid::new_v4();
    let n2 = Uuid::new_v4();
    seed_two_neurons(&mut store, n1, n2);
    let syn_id = Uuid::new_v4();
    seed_synapse(&mut store, syn_id, n1, n2);

    let rng = ChaCha8Rng::seed_from_u64(9);
    let mut handler = MutateRandomSynapseWeightHandler::new(store, rng).unwrap();
    handler
        .handle(MutateRandomSynapseWeightCommand { std_dev: 0.5 })
        .unwrap();

    let store = handler.store;
    let mut replay_store = store;
    let events = replay_store.load().unwrap();
    let net = aei_framework::DomainNetwork::hydrate(&events);
    let last_weight = match events.last().unwrap() {
        Event::SynapseWeightMutated(SynapseWeightMutated { new_weight, .. }) => *new_weight,
        e => panic!("unexpected event {e:?}"),
    };
    assert_eq!(net.synapses.get(&syn_id).unwrap().weight, last_weight);

    let projection = NetworkProjection::from_events(&events);
    let handler = QueryHandler::new(&projection);
    match handler.handle(Query::GetSynapse { id: syn_id }) {
        QueryResult::Synapse(Some(s)) => assert_eq!(s.weight, last_weight),
        _ => panic!("synapse not found"),
    }
}
