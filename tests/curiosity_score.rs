use std::path::PathBuf;

use aei_framework::{
    infrastructure::projection::CuriosityScoreProjection, Activation, CuriosityScope,
    CuriosityScoreUpdated, DomainNetwork, Event, FileEventStore, RandomNeuronAdded,
    RecalculateCuriosityScoreCommand, RecalculateCuriosityScoreHandler,
};
use uuid::Uuid;

fn temp_path() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("aei_curiosity_test_{}.log", Uuid::new_v4()));
    path
}

#[test]
fn curiosity_score_event_and_projection() {
    let path = temp_path();
    let mut store = FileEventStore::new(path.clone());
    let neuron_id = Uuid::new_v4();
    let event = Event::RandomNeuronAdded(RandomNeuronAdded {
        neuron_id,
        activation: Activation::Identity,
    });
    store.append(&event).unwrap();

    let mut handler = RecalculateCuriosityScoreHandler::new(store).unwrap();
    let cmd = RecalculateCuriosityScoreCommand {
        target_ids: vec![neuron_id],
        scope: CuriosityScope::Neuron,
    };
    let events = handler.handle(cmd).unwrap();
    assert!(matches!(
        events.first(),
        Some(Event::CuriosityScoreUpdated(CuriosityScoreUpdated { target_id, .. })) if *target_id == neuron_id
    ));
    let mut store = handler.store;
    let events = store.load().unwrap();
    let projection = CuriosityScoreProjection::from_events(&events);
    let score = projection.get(neuron_id).expect("score present");
    assert!(score <= 1.0);
    let net = DomainNetwork::hydrate(&events);
    let neuron = net.neurons.get(&neuron_id).expect("neuron exists");
    assert_eq!(neuron.curiosity_score, score);
}
