# Gestion des neurones

La couche application permet de gérer explicitement le cycle de vie des
neurones grâce à deux commandes :

- `Command::CreateNeuron` ajoute un neurone avec un identifiant et une
  fonction d'activation choisis.
- `Command::RemoveNeuron` supprime un neurone par identifiant et émonde
  toutes les synapses qui y sont reliées.

## Exemples

```rust
use aei_framework::{Activation, Command, CommandHandler, FileEventStore};
use uuid::Uuid;

fn main() {
    let mut path = std::env::temp_dir();
    path.push("neuron_doc.log");
    let store = FileEventStore::new(path.clone());
    let mut handler = CommandHandler::new(store).unwrap();

    let id = Uuid::new_v4();
    handler
        .handle(Command::CreateNeuron {
            id,
            activation: Activation::ReLU,
        })
        .unwrap();
    handler.handle(Command::RemoveNeuron { id }).unwrap();

    std::fs::remove_file(path).unwrap();
}
```

