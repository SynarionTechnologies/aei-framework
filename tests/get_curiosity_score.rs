use aei_framework::{
    application::{Query, QueryHandler, QueryResult},
    domain::{Activation, CuriosityScoreUpdated, Event, RandomNeuronAdded},
    infrastructure::projection::{CuriosityScoreProjection, NetworkProjection},
};
use uuid::Uuid;

#[test]
fn query_handler_returns_curiosity_score() {
    let id = Uuid::new_v4();
    let events = vec![
        Event::RandomNeuronAdded(RandomNeuronAdded {
            neuron_id: id,
            activation: Activation::ReLU,
        }),
        Event::CuriosityScoreUpdated(CuriosityScoreUpdated {
            target_id: id,
            old_score: 0.0,
            new_score: 0.7,
        }),
    ];
    let network = NetworkProjection::from_events(&events);
    let curiosity = CuriosityScoreProjection::from_events(&events);
    let handler = QueryHandler::new(&network).with_curiosity_projection(&curiosity);

    match handler.handle(Query::GetCuriosityScore { id }) {
        QueryResult::CuriosityScore(Some(score)) => assert!((score - 0.7).abs() < f64::EPSILON),
        _ => panic!("score missing"),
    }

    assert_eq!(handler.curiosity_score(id), Some(0.7));
}
