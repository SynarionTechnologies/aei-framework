use aei_framework::{Activation, Network};

#[test]
fn add_random_neuron_creates_connection() {
    let mut net = Network::new();
    let existing = net.add_neuron_with_activation(Activation::ReLU);
    let new_id = net.add_random_neuron();

    assert_ne!(new_id, existing);
    assert!(!net.input_neurons.values().any(|&id| id == new_id));
    assert!(!net.output_neurons.values().any(|&id| id == new_id));

    let value = serde_json::to_value(&net).unwrap();
    let neurons = value["neurons"].as_array().unwrap();
    let n_json = neurons
        .iter()
        .find(|n| n["id"] == new_id.to_string())
        .expect("new neuron missing");
    let activation = n_json["activation"].as_str().unwrap();
    assert!(["Identity", "Sigmoid", "ReLU", "Tanh"].contains(&activation));

    let synapses = value["synapses"].as_array().unwrap();
    let syn = synapses
        .iter()
        .find(|s| s["from"] == new_id.to_string() || s["to"] == new_id.to_string())
        .expect("no synapse for new neuron");
    let weight = syn["weight"].as_f64().unwrap();
    assert!(weight >= -1.0 && weight <= 1.0);
}
