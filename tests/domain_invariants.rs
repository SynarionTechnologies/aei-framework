use aei_framework::{Activation, DomainNetwork, Event};
use uuid::Uuid;

// Ensure domain rules such as synapse cleanup are respected.
#[test]
fn removing_neuron_cleans_up_synapses() {
    let n1 = Uuid::new_v4();
    let n2 = Uuid::new_v4();
    let s = Uuid::new_v4();

    let events = vec![
        Event::NeuronAdded {
            id: n1,
            activation: Activation::Identity,
        },
        Event::NeuronAdded {
            id: n2,
            activation: Activation::Identity,
        },
        Event::SynapseCreated {
            id: s,
            from: n1,
            to: n2,
            weight: 1.0,
        },
        Event::NeuronRemoved { id: n1 },
    ];

    let net = DomainNetwork::hydrate(&events);
    assert!(!net.neurons.contains_key(&n1));
    assert!(net.neurons.contains_key(&n2));
    assert!(net.synapses.is_empty());
}

// Synapses referencing unknown neurons should be ignored.
#[test]
fn synapse_with_unknown_neuron_is_ignored() {
    let n1 = Uuid::new_v4();
    let events = vec![
        Event::NeuronAdded {
            id: n1,
            activation: Activation::Identity,
        },
        Event::SynapseCreated {
            id: Uuid::new_v4(),
            from: n1,
            to: Uuid::new_v4(),
            weight: 1.0,
        },
    ];

    let net = DomainNetwork::hydrate(&events);
    assert!(net.synapses.is_empty());
}
