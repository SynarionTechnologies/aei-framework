use aei_framework::Network;

/// Checks the propagation of a value through a simple network.
#[test]
fn simple_propagation() {
    let mut net = Network::new();
    let a = net.add_neuron();
    let b = net.add_neuron();
    net.add_synapse(a, b, 0.5);
    net.propagate(a, 2.0);
    assert_eq!(net.value(b), Some(1.0));
}
