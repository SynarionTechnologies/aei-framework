# Score de Curiosité

Le *Score de Curiosité* mesure le caractère novateur ou prometteur d'un neurone ou d'une synapse durant l'exploration. Il est calculé à partir de l'historique des événements et utilise actuellement une formule simple basée sur la rareté. Les scores sont stockés dans les entités de domaine et peuvent être interrogés via une projection dédiée.

## Recalcul

Utilisez `RecalculateCuriosityScoreCommand` pour recalculer le score pour des identifiants spécifiques ou pour l'ensemble du réseau. Un événement `CuriosityScoreUpdated` est émis pour chaque élément mis à jour.

## Récupération

Après mise à jour des scores, ils peuvent être interrogés via [`Query::GetCuriosityScore`](../../src/application/queries.rs) et [`QueryHandler`](../../src/application/query_handler.rs) avec une `CuriosityScoreProjection`.

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
