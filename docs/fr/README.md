# Framework AEI (AEIF)

[![Build](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/owner/aei-framework/actions)
[![License: MPL-2.0](https://img.shields.io/badge/license-MPL%202.0-blue)](LICENSE)

AEI Framework est un framework Rust open source pour construire des réseaux de neurones dynamiques, modulaires, évolutifs, intégrés et multi-agents.

## Objectifs

- Modifier la structure du réseau à l'exécution.
- Ajouter ou supprimer des neurones et des synapses à la volée.
- Fournir une architecture simple et bien documentée.

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
