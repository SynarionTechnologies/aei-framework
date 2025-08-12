use std::path::PathBuf;

use aei_framework::{
    application::{Query, QueryHandler, QueryResult},
    domain::{Event, RandomNeuronAdded, RandomSynapseAdded, SynapseWeightSet},
    infrastructure::{projection::NetworkProjection, FileEventStore},
    Activation, SetSynapseWeightCommand, SetSynapseWeightHandler,
};
use uuid::Uuid;

fn temp_path() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("aei_set_weight_{}.log", Uuid::new_v4()));
    path
}

fn seed_network(store: &mut FileEventStore, n1: Uuid, n2: Uuid, syn: Uuid) {
    let events = [
        Event::RandomNeuronAdded(RandomNeuronAdded {
            neuron_id: n1,
            activation: Activation::Identity,
        }),
        Event::RandomNeuronAdded(RandomNeuronAdded {
            neuron_id: n2,
            activation: Activation::Identity,
        }),
        Event::RandomSynapseAdded(RandomSynapseAdded {
            synapse_id: syn,
            from: n1,
            to: n2,
            weight: 1.0,
        }),
    ];
    for e in &events {
        store.append(e).unwrap();
    }
}

#[test]
fn set_synapse_weight_appends_event() {
    let path = temp_path();
    let mut store = FileEventStore::new(path.clone());
    let n1 = Uuid::new_v4();
    let n2 = Uuid::new_v4();
    let syn_id = Uuid::new_v4();
    seed_network(&mut store, n1, n2, syn_id);

    let mut handler = SetSynapseWeightHandler::new(FileEventStore::new(path.clone())).unwrap();
    handler
        .handle(SetSynapseWeightCommand {
            synapse_id: syn_id,
            new_weight: 2.0,
        })
        .unwrap();

    let mut store = handler.store;
    let events = store.load().unwrap();
    match events.last().unwrap() {
        Event::SynapseWeightSet(SynapseWeightSet {
            synapse_id,
            new_weight,
            ..
        }) => {
            assert_eq!(*synapse_id, syn_id);
            assert_eq!(*new_weight, 2.0);
            assert_eq!(
                handler.network.synapses.get(synapse_id).unwrap().weight,
                *new_weight
            );
        }
        e => panic!("unexpected event {e:?}"),
    }
}

#[test]
fn set_synapse_weight_event_replay() {
    let path = temp_path();
    let mut store = FileEventStore::new(path.clone());
    let n1 = Uuid::new_v4();
    let n2 = Uuid::new_v4();
    let syn_id = Uuid::new_v4();
    seed_network(&mut store, n1, n2, syn_id);

    let mut handler = SetSynapseWeightHandler::new(FileEventStore::new(path.clone())).unwrap();
    handler
        .handle(SetSynapseWeightCommand {
            synapse_id: syn_id,
            new_weight: 2.0,
        })
        .unwrap();

    let events = handler.store.load().unwrap();
    let net = aei_framework::DomainNetwork::hydrate(&events);
    assert_eq!(net.synapses.get(&syn_id).unwrap().weight, 2.0);

    let projection = NetworkProjection::from_events(&events);
    let handler = QueryHandler::new(&projection);
    match handler.handle(Query::GetSynapse { id: syn_id }) {
        QueryResult::Synapse(Some(s)) => assert_eq!(s.weight, 2.0),
        _ => panic!("synapse not found"),
    }
}
