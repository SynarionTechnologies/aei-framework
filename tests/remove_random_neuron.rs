use aei_framework::{Activation, Network};

#[test]
fn remove_random_neuron_deletes_internal_and_synapses() {
    let mut net = Network::new();
    let input = net.add_input_neuron("in", Activation::Identity);
    let hidden = net.add_neuron();
    let output = net.add_output_neuron("out", Activation::Identity);
    net.add_synapse(input, hidden, 1.0).unwrap();
    net.add_synapse(hidden, output, 1.0).unwrap();

    let removed = net.remove_random_neuron().expect("should remove");
    assert_eq!(removed, hidden);

    let value = serde_json::to_value(&net).unwrap();
    let neurons = value["neurons"].as_array().unwrap();
    assert!(neurons.iter().all(|n| n["id"] != removed.to_string()));
    assert!(neurons.iter().any(|n| n["id"] == input.to_string()));
    assert!(neurons.iter().any(|n| n["id"] == output.to_string()));

    let synapses = value["synapses"].as_array().unwrap();
    assert!(synapses
        .iter()
        .all(|s| s["from"] != removed.to_string() && s["to"] != removed.to_string()));
}

#[test]
fn remove_random_neuron_returns_none_when_unavailable() {
    let mut net = Network::new();
    let a = net.add_input_neuron("a", Activation::Identity);
    let b = net.add_output_neuron("b", Activation::Identity);
    net.add_synapse(a, b, 1.0).unwrap();

    assert!(net.remove_random_neuron().is_none());
}
