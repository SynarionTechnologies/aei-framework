# Curiosity Score

The *Curiosity Score* measures how novel or promising a neuron or synapse is during exploration. It is computed from the history of events and currently uses a simple rarity-based formula. Scores are stored inside the domain entities and can be queried through a dedicated projection.

## Recalculation

Use `RecalculateCuriosityScoreCommand` to recompute the score for specific identifiers or for the entire network. A `CuriosityScoreUpdated` event is emitted for each updated element.

## Retrieval

After scores are updated, they can be queried using [`Query::GetCuriosityScore`](../../src/application/queries.rs) and the [`QueryHandler`](../../src/application/query_handler.rs) with a `CuriosityScoreProjection`.

```rust
use aei_framework::{
    application::{Query, QueryHandler, QueryResult},
    domain::{Activation, CuriosityScoreUpdated, Event, RandomNeuronAdded},
    infrastructure::projection::{CuriosityScoreProjection, NetworkProjection},
};
use uuid::Uuid;

let id = Uuid::new_v4();
let events = vec![
    Event::RandomNeuronAdded(RandomNeuronAdded { neuron_id: id, activation: Activation::ReLU }),
    Event::CuriosityScoreUpdated(CuriosityScoreUpdated { target_id: id, old_score: 0.0, new_score: 0.8 }),
];
let network = NetworkProjection::from_events(&events);
let curiosity = CuriosityScoreProjection::from_events(&events);
let handler = QueryHandler::new(&network).with_curiosity_projection(&curiosity);

if let QueryResult::CuriosityScore(Some(score)) = handler.handle(Query::GetCuriosityScore { id }) {
    assert_eq!(score, 0.8);
}
```
