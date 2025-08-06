use aei_framework::{Activation, Network};

fn main() {
    let mut net = Network::new();
    let i1 = net.add_neuron_with_activation(Activation::Identity);
    let i2 = net.add_neuron_with_activation(Activation::Identity);
    let h1 = net.add_neuron_with_activation(Activation::Sigmoid);
    let h2 = net.add_neuron_with_activation(Activation::Sigmoid);
    let o = net.add_neuron_with_activation(Activation::Sigmoid);

    net.add_synapse(i1, h1, 0.5).unwrap();
    net.add_synapse(i1, h2, -0.5).unwrap();
    net.add_synapse(i2, h1, -0.5).unwrap();
    net.add_synapse(i2, h2, 0.5).unwrap();
    net.add_synapse(h1, o, 0.5).unwrap();
    net.add_synapse(h2, o, 0.5).unwrap();

    let dataset = [
        (vec![0.0, 0.0], vec![0.0]),
        (vec![0.0, 1.0], vec![1.0]),
        (vec![1.0, 0.0], vec![1.0]),
        (vec![1.0, 1.0], vec![0.0]),
    ];

    net.train(&dataset, 10000, 0.5);

    for (inputs, _) in dataset.iter() {
        let out = net.predict(inputs);
        println!("{:?} -> {:.3}", inputs, out[0]);
    }
}
