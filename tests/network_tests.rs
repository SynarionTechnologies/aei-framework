use aei_framework::{Activation, Network};

// Helper function for floating point comparisons.
fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-6
}

/// Ensures propagation applies activations in order and resets neuron values
/// between runs.
#[test]
fn propagation_with_activation_and_reset() {
    let mut net = Network::new();
    let input = net.add_neuron_with_activation(Activation::Sigmoid);
    let output = net.add_neuron_with_activation(Activation::ReLU);
    net.add_synapse(input, output, 2.0);

    // First propagation
    net.propagate(input, 1.0);
    let expected_in = Activation::Sigmoid.apply(1.0);
    let expected_out = Activation::ReLU.apply(expected_in * 2.0);
    assert!(approx_eq(net.value(input).unwrap(), expected_in));
    assert!(approx_eq(net.value(output).unwrap(), expected_out));

    // Second propagation with a different value to ensure reset
    net.propagate(input, 0.0);
    let expected_in2 = Activation::Sigmoid.apply(0.0);
    let expected_out2 = Activation::ReLU.apply(expected_in2 * 2.0);
    assert!(approx_eq(net.value(input).unwrap(), expected_in2));
    assert!(approx_eq(net.value(output).unwrap(), expected_out2));
}

/// Propagation where the source neuron uses ReLU and the target Identity.
#[test]
fn propagation_with_relu_source() {
    let mut net = Network::new();
    let input = net.add_neuron_with_activation(Activation::ReLU);
    let output = net.add_neuron(); // Identity
    net.add_synapse(input, output, 1.0);

    net.propagate(input, -1.0);
    assert_eq!(net.value(input), Some(0.0));
    assert_eq!(net.value(output), Some(0.0));
}

/// Propagation where the source neuron uses Tanh.
#[test]
fn propagation_with_tanh_source() {
    let mut net = Network::new();
    let input = net.add_neuron_with_activation(Activation::Tanh);
    let output = net.add_neuron(); // Identity
    net.add_synapse(input, output, 1.0);

    net.propagate(input, 1.0);
    let expected = Activation::Tanh.apply(1.0);
    assert!(approx_eq(net.value(input).unwrap(), expected));
    assert!(approx_eq(net.value(output).unwrap(), expected));
}

/// Propagation where the target neuron uses Sigmoid.
#[test]
fn propagation_with_sigmoid_target() {
    let mut net = Network::new();
    let input = net.add_neuron(); // Identity
    let output = net.add_neuron_with_activation(Activation::Sigmoid);
    net.add_synapse(input, output, 1.0);

    net.propagate(input, 1.0);
    let expected_out = Activation::Sigmoid.apply(1.0);
    assert_eq!(net.value(input), Some(1.0));
    assert!(approx_eq(net.value(output).unwrap(), expected_out));
}
