# Framework AEI (AEIF)

[![Build](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/owner/aei-framework/actions)
[![License: MPL-2.0](https://img.shields.io/badge/license-MPL%202.0-blue)](LICENSE)

AEI Framework est un framework Rust open source pour construire des réseaux de neurones dynamiques, modulaires, évolutifs, intégrés et multi-agents.

## Objectifs

- Modifiez la structure du réseau lors de l'exécution.
- Ajouter ou supprimer les neurones et les synapses à la volée.
- Fournir une API simple et bien documentée.

## Exemple rapide

```rust
use aei_framework::{Activation, Network};

fn main() {
    let mut net = Network::new();
    let input = net.add_neuron(); // Uses the default identity activation
    let hidden = net.add_neuron_with_activation(Activation::ReLU);
    let output = net.add_neuron_with_activation(Activation::Sigmoid);
    net.add_synapse(input, hidden, 1.0).unwrap();
    net.add_synapse(hidden, output, 1.0).unwrap();
    net.propagate(input, -0.5);
    println!("Value of output neuron: {:?}", net.value(output));
}
```

## Exemple avancé

Créer un petit réseau avec des activations hétérogènes et observer
Propagation d'une valeur à travers la chaîne de neurones.

```rust
use aei_framework::{Activation, Network};

let mut net = Network::new();
let input = net.add_neuron_with_activation(Activation::Identity);
let hidden = net.add_neuron_with_activation(Activation::ReLU);
let output = net.add_neuron_with_activation(Activation::Tanh);
net.add_synapse(input, hidden, 0.5).unwrap();
net.add_synapse(hidden, output, 1.0).unwrap();

// Propagate once from the input neuron.
net.propagate(input, 1.0);

println!("Hidden neuron value: {}", net.value(hidden).unwrap());
println!("Output neuron value: {}", net.value(output).unwrap());
```

## Entrées et sorties nommées

Affectez explicitement les neurones en entrées ou sorties et interagissez avec eux par leur nom:

```rust
use aei_framework::{Activation, Network};

let mut net = Network::new();
let a = net.add_input_neuron("a", Activation::Identity);
let b = net.add_input_neuron("b", Activation::Identity);
let out = net.add_output_neuron("out", Activation::Sigmoid);
net.add_synapse(a, out, 1.0).unwrap();
net.add_synapse(b, out, 1.0).unwrap();

net.set_inputs(&[("a", 1.0), ("b", 0.0)]);
net.propagate_inputs().unwrap();
let result = net.get_outputs();
println!("Result: {:?}", result.get("out"));
```

## Ajout aléatoire de neurone

Étendez le réseau en insérant un neurone avec une activation et des connexions
automatiques aléatoires :

```rust
use aei_framework::Network;

let mut net = Network::new();
let new_neuron_id = net.add_random_neuron();
println!("Neurone ajouté : {new_neuron_id}");
```

## Suppression aléatoire de neurone

Réduisez le réseau en supprimant un neurone interne aléatoire ainsi que toutes ses connexions :

```rust
use aei_framework::Network;

let mut net = Network::new();
// ... initialiser le réseau ...
if let Some(removed_id) = net.remove_random_neuron() {
    println!("Neurone supprimé : {removed_id}");
}
```

## Ajout aléatoire de synapse

Créer une synapse entre deux neurones choisis aléatoirement en utilisant le
handler orienté événements :

```rust
use aei_framework::{
    application::{AddRandomSynapseCommand, AddRandomSynapseHandler},
    infrastructure::FileEventStore,
};
use std::path::PathBuf;

let store = FileEventStore::new(PathBuf::from("events.log"));
let mut handler = AddRandomSynapseHandler::new(store, rand::thread_rng()).unwrap();
let synapse_id = handler.handle(AddRandomSynapseCommand).unwrap();
println!("Synapse créée : {synapse_id}");
```

## Sérialisation

Persistez les réseaux sur disque et rechargez-les ensuite en JSON :

```rust
use aei_framework::{Activation, Network};
use std::path::Path;

let mut net = Network::new();
let a = net.add_input_neuron("a", Activation::Identity);
let b = net.add_output_neuron("b", Activation::Identity);
net.add_synapse(a, b, 1.0).unwrap();

let path = Path::new("network.json");
net.save_json(path).unwrap();
let restored = Network::load_json(path).unwrap();
```

## Identificateurs

Chaque neurone et synapse reçoit un [`uuid`] aléatoire (https://docs.rs/uuid) quand
créé. 
fusion des réseaux sans collisions. 
`Neuron :: with_id` et` Network :: add_neuron_with_id` vous permettent de fournir explicite
Identificateurs si nécessaire:

```rust
use aei_framework::{Activation, Neuron};
use uuid::Uuid;

let id = Uuid::new_v4();
let neuron = Neuron::with_id(id, Activation::Sigmoid);
println!("Neuron {id} has value {}", neuron.value);
```

## Apprendre Xor

Former un petit réseau pour approximer la table de vérité XOR en utilisant
rétropropagation:

```rust
use aei_framework::{Activation, Network};

let mut net = Network::new();
let i1 = net.add_neuron_with_activation(Activation::Identity);
let i2 = net.add_neuron_with_activation(Activation::Identity);
let h1 = net.add_neuron_with_activation(Activation::Sigmoid);
let h2 = net.add_neuron_with_activation(Activation::Sigmoid);
let o = net.add_neuron_with_activation(Activation::Sigmoid);

net.add_synapse(i1, h1, 0.5).unwrap();
net.add_synapse(i1, h2, -0.5).unwrap();
net.add_synapse(i2, h1, -0.5).unwrap();
net.add_synapse(i2, h2, 0.5).unwrap();
net.add_synapse(h1, o, 0.5).unwrap();
net.add_synapse(h2, o, 0.5).unwrap();

let dataset = [
    (vec![0.0, 0.0], vec![0.0]),
    (vec![0.0, 1.0], vec![1.0]),
    (vec![1.0, 0.0], vec![1.0]),
    (vec![1.0, 1.0], vec![0.0]),
];

net.train(&dataset, 10_000, 0.5).unwrap();

let output = net.predict(&[0.0, 1.0]).unwrap()[0];
println!("XOR(0,1) ≈ {output}");
```

## Propagation étape par étape

`Network :: Propagate 'effectue quatre phases ordonnées:

1. ** Réinitialiser ** - La valeur de chaque neurone est effacée à «0,0».
2. ** Activation de la source ** - La valeur d'entrée est passé par la source
Fonction d'activation du neurone.
3. ** Propagation pondérée ** - Les synapses contribuent `de_value * poids 'à
leurs cibles.
4. ** Activation ** - Chaque neurone cible applique sa fonction d'activation une fois
Des entrées ont été reçues.

Cette séquence déterministe garantit que les appels répétés ne s'accumulent pas
état et que les activations sont appliquées uniquement après que toutes les entrées sont traitées.

## Fonctions d'activation

Les neurones prennent en charge plusieurs fonctions d'activation:

- «Identité»
- `Sigmoïde»
- «Relu»
- «Tanh»

Par défaut, les neurones utilisent «l'identité». 
Activation, soit instancier un [«neuron»] ou utiliser directement
`Network :: add_neuron_with_activation`:

```rust
use aei_framework::{Activation, Neuron};

let neuron = Neuron::new(Activation::Tanh);
println!("Neuron id: {}", neuron.id);
```

## Exemple: réseau multi-activation

```rust
use aei_framework::{Activation, Network};

let mut net = Network::new();
let n1 = net.add_neuron_with_activation(Activation::Sigmoid);
let n2 = net.add_neuron_with_activation(Activation::ReLU);
net.add_synapse(n1, n2, 2.0).unwrap();

net.propagate(n1, 1.0);
println!("Value of neuron {n1} (Sigmoid): {}", net.value(n1).unwrap());
println!("Value of neuron {n2} (ReLU): {}", net.value(n2).unwrap());
```

L'utilisation d'activations variées permet à chaque neurone de traiter les données différemment. 
des fonctions lisses comme «sigmoïde» avec des par morceaux linéaires comme «relu» augmente
la puissance de représentation du réseau.

## Flux de propagation

`Network :: Propagate` effectue une passe en avant dans le réseau en quatre
Étapes commandées:

1. ** Réinitialiser ** - Toutes les valeurs des neurones sont définies sur «0,0», garantissant que
Les propagations n'interfèrent pas entre elles.
2. ** Activation source ** - La valeur d'entrée est passée par l'activation
fonction du neurone source.
3. ** Propagation pondérée ** - Chaque synapse ajoute `de_value * poids 'au
la somme en attente du neurone cible.
4. ** Activation ** - Une fois que toutes les sommes ont été collectées, chaque neurone s'applique
sa fonction d'activation pour produire la nouvelle sortie.

Cette séquence garantit que chaque neurone est activé exactement une fois par
La propagation et que les courses précédentes ne fuient pas dans la suivante.

## Journalisation

Le framework émet des messages d'information via la crate [`log`](https://docs.rs/log). Pour afficher ces journaux, initialisez une implémentation de logger comme [`env_logger`](https://docs.rs/env_logger) dans votre application :

```rust
env_logger::init();
```

Avec un logger configuré, la progression de fonctions comme `Network::train` sera rapportée au niveau `info`.

## Structure du projet

```
src/
  core/        # activation, neuron, synapse primitives
  api/         # network implementation and public API
examples/
tests/
docs/
  en/
  fr/
```

## Documentation

Les guides en anglais et en français sont disponibles dans [docs/en](docs/en/README.md) et [docs/fr](docs/fr/README.md).

## Développement

```bash
cargo build    # Build the project
cargo test     # Run the test suite
```

## Contribution

Les contributions sont les bienvenues! 

## Journal des modifications

Voir [CHANGELOG.md](CHANGELOG.md) pour la liste des modifications.

## Licence

Distribué sous la licence publique de Mozilla 2.0. 

## Limitations connues

- Les identificateurs de neurones et de synapses utilisent `Uuid`. Les réseaux sérialisés avec d'anciens identifiants numériques ne sont pas pris en charge.
- La persistance JSON est disponible via `save_json` et `load_json`.
- Les abstractions en couches sont planifiées mais non mises en œuvre.
