use std::path::PathBuf;

use aei_framework::{
    application::{
        MutateNeuronActivationError, MutateRandomNeuronActivationCommand,
        MutateRandomNeuronActivationHandler, Query, QueryHandler, QueryResult,
    },
    domain::{Activation, Event, NeuronActivationMutated, RandomNeuronAdded},
    infrastructure::{projection::NetworkProjection, EventStore, FileEventStore},
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use uuid::Uuid;

fn temp_path() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("aei_mutate_act_{}.log", Uuid::new_v4()));
    path
}

fn seed_neuron(store: &mut FileEventStore, id: Uuid, activation: Activation) {
    let event = Event::RandomNeuronAdded(RandomNeuronAdded { neuron_id: id, activation });
    store.append(&event).unwrap();
}

#[test]
fn mutate_neuron_activation_appends_event() {
    let path = temp_path();
    let mut store = FileEventStore::new(path.clone());
    let neuron_id = Uuid::new_v4();
    seed_neuron(&mut store, neuron_id, Activation::Identity);

    let rng = ChaCha8Rng::seed_from_u64(1);
    let mut handler = MutateRandomNeuronActivationHandler::new(store, rng).unwrap();
    let mutated_id = handler
        .handle(MutateRandomNeuronActivationCommand { exclude_io: false })
        .unwrap();
    assert_eq!(mutated_id, neuron_id);

    let mut store = handler.store;
    let events = store.load().unwrap();
    match events.last().unwrap() {
        Event::NeuronActivationMutated(NeuronActivationMutated { neuron_id: id, old_activation, new_activation }) => {
            assert_eq!(*id, neuron_id);
            assert_eq!(*old_activation, Activation::Identity);
            assert_ne!(*old_activation, *new_activation);
            assert_eq!(
                handler.network.neurons.get(id).unwrap().activation,
                *new_activation
            );
        }
        e => panic!("unexpected event {e:?}"),
    }

    let projection = NetworkProjection::from_events(&events);
    let handler = QueryHandler::new(&projection);
    match handler.handle(Query::GetNeuronActivation { id: neuron_id }) {
        QueryResult::Activation(Some(a)) => {
            if let Event::NeuronActivationMutated(NeuronActivationMutated { new_activation, .. }) = events.last().unwrap() {
                assert_eq!(*new_activation, a);
            }
        }
        _ => panic!("activation not found"),
    }
}

#[test]
fn mutate_neuron_activation_errors_when_no_eligible() {
    let path = temp_path();
    let mut store = FileEventStore::new(path);
    let neuron_id = Uuid::new_v4();
    seed_neuron(&mut store, neuron_id, Activation::Identity);

    let rng = ChaCha8Rng::seed_from_u64(2);
    let mut handler = MutateRandomNeuronActivationHandler::new(store, rng).unwrap();
    let res = handler.handle(MutateRandomNeuronActivationCommand { exclude_io: true });
    assert!(matches!(res, Err(MutateNeuronActivationError::NoEligibleNeuron)));
}

#[test]
fn mutate_neuron_activation_event_replay() {
    let path = temp_path();
    let mut store = FileEventStore::new(path.clone());
    let neuron_id = Uuid::new_v4();
    seed_neuron(&mut store, neuron_id, Activation::Identity);

    let rng = ChaCha8Rng::seed_from_u64(3);
    let mut handler = MutateRandomNeuronActivationHandler::new(store, rng).unwrap();
    handler
        .handle(MutateRandomNeuronActivationCommand { exclude_io: false })
        .unwrap();
    let store = handler.store;
    let mut replay_store = store;
    let events = replay_store.load().unwrap();
    let net = aei_framework::DomainNetwork::hydrate(&events);
    let last_activation = match events.last().unwrap() {
        Event::NeuronActivationMutated(NeuronActivationMutated { new_activation, .. }) => *new_activation,
        e => panic!("unexpected event {e:?}"),
    };
    assert_eq!(net.neurons.get(&neuron_id).unwrap().activation, last_activation);

    let projection = NetworkProjection::from_events(&events);
    let handler = QueryHandler::new(&projection);
    match handler.handle(Query::GetNeuronActivation { id: neuron_id }) {
        QueryResult::Activation(Some(a)) => assert_eq!(a, last_activation),
        _ => panic!("activation not found"),
    }
}
