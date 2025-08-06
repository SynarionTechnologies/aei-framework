use aei_framework::{Activation, Network};

/// Train a tiny network to learn the logical AND function.
fn main() {
    // Build network with two inputs, one bias and one sigmoid output.
    let mut net = Network::new();
    let a = net.add_neuron_with_activation(Activation::Identity);
    let b = net.add_neuron_with_activation(Activation::Identity);
    let bias = net.add_neuron_with_activation(Activation::Identity);
    let out = net.add_neuron_with_activation(Activation::Sigmoid);

    // Initial synapses including a bias weight.
    net.add_synapse(a, out, 1.0).unwrap();
    net.add_synapse(b, out, 1.0).unwrap();
    net.add_synapse(bias, out, -1.5).unwrap();

    // Dataset with bias input fixed at 1.0 for every sample.
    let dataset = [
        (vec![0.0, 0.0, 1.0], vec![0.0]),
        (vec![0.0, 1.0, 1.0], vec![0.0]),
        (vec![1.0, 0.0, 1.0], vec![0.0]),
        (vec![1.0, 1.0, 1.0], vec![1.0]),
    ];

    // Train and display results.
    net.train(&dataset, 5000, 0.5).unwrap();

    for (inputs, _) in dataset.iter() {
        let out_val = net.predict(inputs).unwrap();
        println!("{:?} -> {:.3}", &inputs[..2], out_val[0]);
    }
}
