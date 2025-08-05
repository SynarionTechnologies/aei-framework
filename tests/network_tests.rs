use aei_framework::{Activation, Network};

/// Checks the propagation of a value through a network using the identity activation.
#[test]
fn simple_propagation() {
    let mut net = Network::new();
    let a = net.add_neuron();
    let b = net.add_neuron();
    net.add_synapse(a, b, 0.5);
    net.propagate(a, 2.0);
    assert_eq!(net.value(b), Some(1.0));
}

/// Ensures the sigmoid activation is applied to the target neuron.
#[test]
fn sigmoid_activation() {
    let mut net = Network::new();
    let a = net.add_neuron();
    let b = net.add_neuron_with_activation(Activation::Sigmoid);
    net.add_synapse(a, b, 1.0);
    net.propagate(a, 0.0);
    let out = net.value(b).unwrap();
    assert!((out - 0.5).abs() < 1e-6);
}

/// Ensures the ReLU activation is applied to the target neuron.
#[test]
fn relu_activation() {
    let mut net = Network::new();
    let a = net.add_neuron();
    let b = net.add_neuron_with_activation(Activation::ReLU);
    net.add_synapse(a, b, 1.0);
    net.propagate(a, -1.0);
    assert_eq!(net.value(b), Some(0.0));
}

/// Ensures the tanh activation is applied to the target neuron.
#[test]
fn tanh_activation() {
    let mut net = Network::new();
    let a = net.add_neuron();
    let b = net.add_neuron_with_activation(Activation::Tanh);
    net.add_synapse(a, b, 1.0);
    net.propagate(a, 1.0);
    let out = net.value(b).unwrap();
    assert!((out - 1.0f64.tanh()).abs() < 1e-6);
}
