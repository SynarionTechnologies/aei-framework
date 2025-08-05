use aei_framework::{activation::Activation, network::Network};

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-8
}

#[test]
fn test_all_activation_functions() {
    let mut net = Network::new();
    let id = net.add_neuron_with_activation(Activation::Identity);
    let sigmoid = net.add_neuron_with_activation(Activation::Sigmoid);
    let relu = net.add_neuron_with_activation(Activation::ReLU);
    let tanh = net.add_neuron_with_activation(Activation::Tanh);

    net.propagate(id, 0.5);
    assert!(approx_eq(net.value(id).unwrap(), 0.5));

    net.propagate(sigmoid, 1.0);
    let expected_sigmoid = 1.0 / (1.0 + (-1.0f64).exp());
    assert!(approx_eq(net.value(sigmoid).unwrap(), expected_sigmoid));

    net.propagate(relu, -2.0);
    assert!(approx_eq(net.value(relu).unwrap(), 0.0));

    net.propagate(tanh, 1.0);
    assert!(approx_eq(net.value(tanh).unwrap(), 1.0f64.tanh()));
}

#[test]
fn test_chained_propagation() {
    let mut net = Network::new();
    let a = net.add_neuron_with_activation(Activation::Sigmoid);
    let b = net.add_neuron_with_activation(Activation::ReLU);
    let c = net.add_neuron_with_activation(Activation::Tanh);
    net.add_synapse(a, b, 2.0);
    net.add_synapse(b, c, -1.0);

    net.propagate(a, 1.0);

    let expected_a = 1.0 / (1.0 + (-1.0f64).exp());
    let expected_b = (expected_a * 2.0).max(0.0);
    let expected_c = -expected_b.tanh();

    assert!(approx_eq(net.value(a).unwrap(), expected_a));
    assert!(approx_eq(net.value(b).unwrap(), expected_b));
    assert!(approx_eq(net.value(c).unwrap(), expected_c));
}

#[test]
fn test_no_accumulation() {
    let mut net = Network::new();
    let a = net.add_neuron_with_activation(Activation::Sigmoid);
    let b = net.add_neuron_with_activation(Activation::ReLU);
    net.add_synapse(a, b, 1.5);

    net.propagate(a, 1.0);
    let first_b = net.value(b).unwrap();

    net.propagate(a, 0.5);
    let second_expected_a = 1.0 / (1.0 + (-0.5f64).exp());
    let second_expected_b = (second_expected_a * 1.5).max(0.0);

    assert!(approx_eq(net.value(b).unwrap(), second_expected_b));
    assert_ne!(first_b, net.value(b).unwrap());
}
