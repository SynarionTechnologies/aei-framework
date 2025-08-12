# Neuron Management

Explicit neuron lifecycle operations are available through the application
layer. Two commands drive these changes:

- `Command::CreateNeuron` inserts a neuron with a chosen identifier and
  activation function.
- `Command::RemoveNeuron` deletes a neuron by identifier and prunes all
  attached synapses.

## Examples

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

