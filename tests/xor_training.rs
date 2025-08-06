use aei_framework::{Activation, Network};

fn loss(net: &mut Network, data: &[(Vec<f64>, Vec<f64>)]) -> f64 {
    data.iter()
        .map(|(inp, out)| {
            let pred = net.predict(inp).unwrap();
            pred.iter()
                .zip(out.iter())
                .map(|(a, b)| {
                    let e = a - b;
                    0.5 * e * e
                })
                .sum::<f64>()
        })
        .sum::<f64>()
        / data.len() as f64
}

#[test]
fn xor_training_reduces_loss() {
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

    let initial_loss = loss(&mut net, &dataset);
    net.train(&dataset, 10000, 0.5).unwrap();
    let final_loss = loss(&mut net, &dataset);

    assert!(final_loss < initial_loss);
}
