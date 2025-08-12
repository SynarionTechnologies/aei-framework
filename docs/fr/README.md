# Framework AEI (AEIF)

[![Build](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/owner/aei-framework/actions)
[![License: MPL-2.0](https://img.shields.io/badge/license-MPL%202.0-blue)](LICENSE)

AEI Framework est un framework Rust open source pour construire des réseaux de neurones dynamiques, modulaires, évolutifs, intégrés et multi-agents.

## Objectifs

- Modifier la structure du réseau à l'exécution.
- Ajouter ou supprimer des neurones et des synapses à la volée.
- Fournir une architecture simple et bien documentée.

## Stockage des événements

`JsonlEventStore<T>` persiste tout type d'événement implémentant `Serialize` et `DeserializeOwned` au format JSON Lines. Les alias `FileEventStore` et `FileMemoryEventStore` fournissent des magasins prêts à l'emploi pour les événements de domaine et de mémoire.

## Ajout aléatoire de neurone

Étendez le réseau en envoyant une commande gérée par l'infrastructure orientée événements :

```rust
use aei_framework::{
    AddRandomNeuronCommand, AddRandomNeuronHandler, FileEventStore,
};
use rand::thread_rng;
use std::path::PathBuf;

let store = FileEventStore::new(PathBuf::from("events.log"));
let mut handler = AddRandomNeuronHandler::new(store, thread_rng()).unwrap();
let new_neuron_id = handler.handle(AddRandomNeuronCommand).unwrap();
println!("Neurone ajouté : {new_neuron_id}");
```

## Suppression aléatoire de neurone

Réduisez le réseau via un gestionnaire dédié :

```rust
use aei_framework::{
    RemoveRandomNeuronCommand, RemoveRandomNeuronHandler, FileEventStore,
};
use rand::thread_rng;
use std::path::PathBuf;

let store = FileEventStore::new(PathBuf::from("events.log"));
let mut handler = RemoveRandomNeuronHandler::new(store, thread_rng()).unwrap();
if let Ok(removed_id) = handler.handle(RemoveRandomNeuronCommand) {
    println!("Neurone supprimé : {removed_id}");
}
```

## Mutation aléatoire de l’activation d’un neurone

Muter la fonction d’activation d’un neurone choisi aléatoirement :

```rust
use aei_framework::{
    MutateRandomNeuronActivationCommand, MutateRandomNeuronActivationHandler,
    FileEventStore,
};
use rand::thread_rng;
use std::path::PathBuf;

let store = FileEventStore::new(PathBuf::from("events.log"));
let mut handler =
    MutateRandomNeuronActivationHandler::new(store, thread_rng()).unwrap();
if let Ok(neuron_id) = handler.handle(MutateRandomNeuronActivationCommand { exclude_io: false }) {
    println!("Activation mutée : {neuron_id}");
}
```

## Ajout aléatoire de synapse

Créez une synapse entre deux neurones choisis aléatoirement en utilisant le gestionnaire orienté événements :

```rust
use aei_framework::{
    application::{AddRandomSynapseCommand, AddRandomSynapseHandler},
    infrastructure::FileEventStore,
};
use std::path::PathBuf;

let store = FileEventStore::new(PathBuf::from("events.log"));
let mut handler = AddRandomSynapseHandler::new(store, rand::thread_rng()).unwrap();
let synapse_id = handler.handle(AddRandomSynapseCommand).unwrap();
println!("Synapse créée : {synapse_id}");
```

## Suppression aléatoire de synapse

Supprimez une synapse sélectionnée aléatoirement via un gestionnaire orienté événements :

```rust
use aei_framework::{
    RemoveRandomSynapseCommand, RemoveRandomSynapseHandler, FileEventStore,
};
use rand::thread_rng;
use std::path::PathBuf;

let store = FileEventStore::new(PathBuf::from("events.log"));
let mut handler = RemoveRandomSynapseHandler::new(store, thread_rng()).unwrap();
if let Ok(removed_id) = handler.handle(RemoveRandomSynapseCommand) {
    println!("Synapse supprimée : {removed_id}");
}
```

## Définir le poids d'une synapse

Assignez un poids précis à une synapse existante :

```rust
use aei_framework::{SetSynapseWeightCommand, SetSynapseWeightHandler, FileEventStore};
use std::path::PathBuf;

let store = FileEventStore::new(PathBuf::from("events.log"));
let mut handler = SetSynapseWeightHandler::new(store).unwrap();
let synapse_id = uuid::Uuid::new_v4(); // identifiant d'une synapse existante
handler
    .handle(SetSynapseWeightCommand { synapse_id, new_weight: 0.5 })
    .unwrap();
```

## Mutation aléatoire du poids d'une synapse

Ajustez le poids d'une synapse en ajoutant un bruit gaussien :

```rust
use aei_framework::{
    MutateRandomSynapseWeightCommand, MutateRandomSynapseWeightHandler, FileEventStore,
};
use rand::thread_rng;
use std::path::PathBuf;

let store = FileEventStore::new(PathBuf::from("events.log"));
let mut handler = MutateRandomSynapseWeightHandler::new(store, thread_rng()).unwrap();
if let Ok(synapse_id) =
    handler.handle(MutateRandomSynapseWeightCommand { std_dev: 0.1 })
{
    println!("Synapse mutée : {synapse_id}");
}
```

## Mémoire adaptative

Enregistrez et interrogez des expériences passées dans un tampon borné et scoré :

```rust
use aei_framework::{
    application::memory::{
        AddMemoryEntryCommand, AddMemoryEntryHandler, MemoryQuery, MemoryQueryHandler,
    },
    infrastructure::{projection::MemoryProjection, FileMemoryEventStore},
};
use serde_json::json;
use std::path::PathBuf;

let store = FileMemoryEventStore::new(PathBuf::from("memory.log"));
let mut handler = AddMemoryEntryHandler::new(store, 50).unwrap();
handler
    .handle(AddMemoryEntryCommand {
        event_type: "interaction".into(),
        payload: json!({"msg": "bonjour"}),
        score: 0.7,
    })
    .unwrap();
let mut store = handler.base.store;
let events = store.load().unwrap();
let projection = MemoryProjection::from_events(50, &events);
let qh = MemoryQueryHandler::new(&projection);
let _entries = qh.handle(MemoryQuery::GetByEventType {
    event_type: "interaction".into(),
    limit: 10,
});
```

## Score de curiosité

Évaluez le potentiel exploratoire des composants du réseau :

```rust
use aei_framework::{
    CuriosityScope, RecalculateCuriosityScoreCommand, RecalculateCuriosityScoreHandler,
    FileEventStore,
};
use std::path::PathBuf;

let store = FileEventStore::new(PathBuf::from("events.log"));
let mut handler = RecalculateCuriosityScoreHandler::new(store).unwrap();
handler
    .handle(RecalculateCuriosityScoreCommand {
        target_ids: vec![],
        scope: CuriosityScope::All,
    })
    .unwrap();
```

## Journalisation

Le framework émet des messages d'information via la crate [`log`](https://docs.rs/log). Pour afficher ces journaux, initialisez une implémentation de logger comme [`env_logger`](https://docs.rs/env_logger) dans votre application :

```rust
env_logger::init();
```

Avec un logger configuré, la progression des gestionnaires de commandes sera rapportée au niveau `info`.

## Exemple

Un flux de commandes minimal est disponible dans [examples/basic.rs](../../examples/basic.rs) :

```bash
 cargo run --example basic
```

Il ajoute des neurones, les relie par une synapse et interroge le modèle de lecture.

## Structure du projet

```
src/
  domain/      # primitives, agrégats et événements de domaine
  application/ # commandes, requêtes et gestionnaires
  infrastructure/
    event_store.rs  # implémentations du magasin d'événements
    projection/     # projections pour la lecture
examples/
tests/
docs/
  en/
  fr/
```

## Aperçu de l'architecture

AEIF suit le Domain-Driven Design avec Event Sourcing et CQRS. Les opérations modifiant l'état sont exprimées sous forme de **commandes** transformées en **événements** immuables et ajoutées à un journal. Les agrégats tels que `domain::Network` rejouent ces événements pour reconstruire leur état. Les lectures sont servies via des **requêtes** traitées par des projections situées sous `infrastructure/projection`.

**Commande → Événement → Application → Projection**

1. Une commande exprime l'intention de modifier l'état.
2. Le gestionnaire émet et persiste un événement de domaine.
3. L'agrégat applique l'événement pour mettre à jour son état.
4. Les projections consomment l'événement pour rafraîchir les modèles de lecture.

## Documentation

Les guides en anglais et en français sont disponibles dans [docs/en](docs/en/README.md) et [docs/fr](docs/fr/README.md).
Un glossaire des termes métiers et techniques est disponible dans [GLOSSARY.md](GLOSSARY.md).

## Développement

```bash
cargo build    # Compiler le projet
cargo test     # Exécuter la suite de tests
```

## Contribution

Les contributions sont les bienvenues !

## Journal des modifications

Voir [CHANGELOG.md](CHANGELOG.md) pour la liste des modifications.

## Licence

Distribué sous la licence publique Mozilla 2.0.

## Limitations connues

- Les identificateurs de neurones et de synapses utilisent `Uuid`. Les réseaux sérialisés avec d'anciens identifiants numériques ne sont pas pris en charge.
- La persistance JSON est disponible via `save_json` et `load_json`.
- Des abstractions en couches sont prévues mais non encore implémentées.
