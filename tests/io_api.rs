use aei_framework::{Activation, Network};

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-8
}

#[test]
fn test_named_inputs_outputs() {
    let mut net = Network::new();
    let a = net.add_input_neuron("a", Activation::Identity);
    let b = net.add_input_neuron("b", Activation::Identity);
    let out = net.add_output_neuron("sum", Activation::Identity);
    net.add_synapse(a, out, 1.0).unwrap();
    net.add_synapse(b, out, 1.0).unwrap();

    net.set_inputs(&[("a", 1.0), ("b", 2.0)]);
    net.propagate_inputs();
    let outputs = net.get_outputs();
    assert!(approx_eq(outputs["sum"], 3.0));
}

#[test]
fn test_indexed_inputs_outputs() {
    let mut net = Network::new();
    let a = net.add_input_neuron("a", Activation::Identity);
    let b = net.add_input_neuron("b", Activation::Identity);
    let out = net.add_output_neuron("sum", Activation::Identity);
    net.add_synapse(a, out, 1.0).unwrap();
    net.add_synapse(b, out, 1.0).unwrap();

    net.set_inputs_by_index(&[1.0, 2.0]);
    net.propagate_inputs();
    let outputs = net.get_outputs_by_index();
    assert!(approx_eq(outputs[0], 3.0));
}

#[test]
fn test_backward_compatibility_by_id() {
    let mut net = Network::new();
    let input = net.add_input_neuron("in", Activation::Identity);
    let output = net.add_output_neuron("out", Activation::Identity);
    net.add_synapse(input, output, 1.0).unwrap();

    net.propagate(input, 2.0);
    assert!(approx_eq(net.value(output).unwrap(), 2.0));
}
