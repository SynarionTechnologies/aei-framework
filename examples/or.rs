use aei_framework::{Activation, Network};

/// Train a small network to learn the logical OR function.
fn main() {
    // Two inputs, one bias and one sigmoid output.
    let mut net = Network::new();
    let a = net.add_neuron_with_activation(Activation::Identity);
    let b = net.add_neuron_with_activation(Activation::Identity);
    let bias = net.add_neuron_with_activation(Activation::Identity);
    let out = net.add_neuron_with_activation(Activation::Sigmoid);

    // Connections with a bias slightly negative.
    net.add_synapse(a, out, 1.0).unwrap();
    net.add_synapse(b, out, 1.0).unwrap();
    net.add_synapse(bias, out, -0.5).unwrap();

    // Dataset with bias input set to 1.0.
    let dataset = [
        (vec![0.0, 0.0, 1.0], vec![0.0]),
        (vec![0.0, 1.0, 1.0], vec![1.0]),
        (vec![1.0, 0.0, 1.0], vec![1.0]),
        (vec![1.0, 1.0, 1.0], vec![1.0]),
    ];

    net.train(&dataset, 3000, 0.5).unwrap();

    for (inputs, _) in dataset.iter() {
        let out_val = net.predict(inputs).unwrap();
        println!("{:?} -> {:.3}", &inputs[..2], out_val[0]);
    }
}
