use aei_framework::{Activation, Network};

/// Fit y = 2x + 1 using a single neuron and a bias.
fn main() {
    // One input, one bias, one linear output.
    let mut net = Network::new();
    let x = net.add_neuron_with_activation(Activation::Identity);
    let bias = net.add_neuron_with_activation(Activation::Identity);
    let out = net.add_neuron_with_activation(Activation::Identity);

    net.add_synapse(x, out, 0.0).unwrap();
    net.add_synapse(bias, out, 0.0).unwrap();

    // Dataset for y = 2x + 1 with bias input fixed at 1.0.
    let dataset = [
        (vec![0.0, 1.0], vec![1.0]),
        (vec![1.0, 1.0], vec![3.0]),
        (vec![2.0, 1.0], vec![5.0]),
        (vec![3.0, 1.0], vec![7.0]),
    ];

    net.train(&dataset, 2000, 0.01).unwrap();

    // Predict for x=4 and print the result.
    let out_val = net.predict(&[4.0, 1.0]).unwrap();
    println!("Input 4.0 => {:.3} (expected 9.0)", out_val[0]);
}
