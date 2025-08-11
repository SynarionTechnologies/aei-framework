use aei_framework::{Activation, DomainNetwork, Event, RandomNeuronAdded, RandomSynapseAdded};
use uuid::Uuid;

/// Ensure a synapse pointing to the same neuron is ignored.
#[test]
fn self_loop_synapse_is_ignored() {
    let n1 = Uuid::new_v4();
    let n2 = Uuid::new_v4();
    let s = Uuid::new_v4();

    let events = vec![
        Event::RandomNeuronAdded(RandomNeuronAdded {
            neuron_id: n1,
            activation: Activation::Identity,
        }),
        Event::RandomNeuronAdded(RandomNeuronAdded {
            neuron_id: n2,
            activation: Activation::Identity,
        }),
        Event::RandomSynapseAdded(RandomSynapseAdded {
            synapse_id: s,
            from: n1,
            to: n1,
            weight: 1.0,
        }),
    ];

    let net = DomainNetwork::hydrate(&events);
    assert!(net.synapses.is_empty());
}

/// Verify that duplicate synapses between the same neurons are ignored.
#[test]
fn duplicate_synapse_is_ignored() {
    let n1 = Uuid::new_v4();
    let n2 = Uuid::new_v4();
    let s1 = Uuid::new_v4();
    let s2 = Uuid::new_v4();

    let events = vec![
        Event::RandomNeuronAdded(RandomNeuronAdded {
            neuron_id: n1,
            activation: Activation::Identity,
        }),
        Event::RandomNeuronAdded(RandomNeuronAdded {
            neuron_id: n2,
            activation: Activation::Identity,
        }),
        Event::RandomSynapseAdded(RandomSynapseAdded {
            synapse_id: s1,
            from: n1,
            to: n2,
            weight: 1.0,
        }),
        Event::RandomSynapseAdded(RandomSynapseAdded {
            synapse_id: s2,
            from: n1,
            to: n2,
            weight: 2.0,
        }),
    ];

    let net = DomainNetwork::hydrate(&events);
    assert_eq!(net.synapses.len(), 1);
    assert!(net.synapses.contains_key(&s1));
    assert!(!net.synapses.contains_key(&s2));
}
