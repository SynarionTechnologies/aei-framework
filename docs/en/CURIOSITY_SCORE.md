# Curiosity Score

The *Curiosity Score* measures how novel or promising a neuron or synapse is during exploration. It is computed from the history of events and currently uses a simple rarity-based formula. Scores are stored inside the domain entities and can be queried through a dedicated projection.

## Recalculation

Use `RecalculateCuriosityScoreCommand` to recompute the score for specific identifiers or for the entire network. A `CuriosityScoreUpdated` event is emitted for each updated element.
