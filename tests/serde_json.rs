use aei_framework::{Activation, Network};
use std::fs;

#[test]
fn network_json_roundtrip() {
    let mut net = Network::new();
    let inp = net.add_input_neuron("in", Activation::Identity);
    let out = net.add_output_neuron("out", Activation::Identity);
    net.add_synapse(inp, out, 1.5);

    let mut path = std::env::temp_dir();
    path.push("net.json");
    net.save_json(&path).unwrap();

    let mut loaded = Network::load_json(&path).unwrap();
    fs::remove_file(&path).ok();

    loaded.set_inputs(&[("in", 2.0)]);
    loaded.propagate_inputs();
    let outputs = loaded.get_outputs();
    assert_eq!(outputs.get("out").copied().unwrap(), 3.0);
}
