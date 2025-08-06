use aei_framework::{Activation, Network, NetworkError};
use uuid::Uuid;

// Helper for floating-point comparisons in tests.
fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-8
}

/// Sigmoid source neuron connected to a ReLU target through a synapse with
/// weight `2.0`.
#[test]
fn test_sigmoid_to_relu_chain() {
    let mut net = Network::new();
    let n1 = net.add_neuron_with_activation(Activation::Sigmoid);
    let n2 = net.add_neuron_with_activation(Activation::ReLU);
    net.add_synapse(n1, n2, 2.0).unwrap();

    net.propagate(n1, 1.0);

    let expected_n1 = 1.0 / (1.0 + (-1.0f64).exp());
    let expected_n2 = (expected_n1 * 2.0).max(0.0);
    assert!(approx_eq(net.value(n1).unwrap(), expected_n1));
    assert!(approx_eq(net.value(n2).unwrap(), expected_n2));
}

/// Propagating several values in sequence should not accumulate state between
/// runs.
#[test]
fn test_multiple_propagations_reset() {
    let mut net = Network::new();
    let src = net.add_neuron_with_activation(Activation::Identity);
    let dst = net.add_neuron_with_activation(Activation::Identity);
    net.add_synapse(src, dst, 1.0).unwrap();

    for &v in &[1.0, -2.0, 0.5] {
        net.propagate(src, v);
        assert!(approx_eq(net.value(src).unwrap(), v));
        assert!(approx_eq(net.value(dst).unwrap(), v));
    }
}

/// Propagating from a non-existent neuron should do nothing.
#[test]
fn test_propagate_nonexistent_neuron() {
    let mut net = Network::new();
    let id = net.add_neuron();

    net.propagate(Uuid::new_v4(), 1.0);
    assert!(approx_eq(net.value(id).unwrap(), 0.0));
}

/// Synapses targeting missing neurons are ignored.
#[test]
fn test_synapse_to_missing_neuron() {
    let mut net = Network::new();
    let src = net.add_neuron();
    let res = net.add_synapse(src, Uuid::new_v4(), 1.0);
    assert!(matches!(res, Err(NetworkError::UnknownNeuron)));
}

/// Synapses whose source neuron is missing never fire.
#[test]
fn test_orphan_synapse() {
    let mut net = Network::new();
    let existing = net.add_neuron();
    let res = net.add_synapse(Uuid::new_v4(), existing, 1.0);
    assert!(matches!(res, Err(NetworkError::UnknownNeuron)));
}

/// Neuron identifiers remain stable through string serialization.
#[test]
fn test_uuid_round_trip() {
    let mut net = Network::new();
    let id = net.add_neuron();
    let s = id.to_string();
    let parsed = Uuid::parse_str(&s).unwrap();
    assert_eq!(id, parsed);
}
