# Score de Curiosité

Le *Score de Curiosité* mesure le caractère novateur ou prometteur d'un neurone ou d'une synapse durant l'exploration. Il est calculé à partir de l'historique des événements et utilise actuellement une formule simple basée sur la rareté. Les scores sont stockés dans les entités de domaine et peuvent être interrogés via une projection dédiée.

## Recalcul

Utilisez `RecalculateCuriosityScoreCommand` pour recalculer le score pour des identifiants spécifiques ou pour l'ensemble du réseau. Un événement `CuriosityScoreUpdated` est émis pour chaque élément mis à jour.
