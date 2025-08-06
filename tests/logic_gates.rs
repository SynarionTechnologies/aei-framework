use aei_framework::{Activation, Network};

fn evaluate(net: &mut Network, dataset: &[(Vec<f64>, f64)]) {
    for (inputs, expected) in dataset {
        let out = net.predict(inputs).unwrap();
        if *expected < 0.5 {
            assert!(out[0] < 0.5);
        } else {
            assert!(out[0] > 0.5);
        }
    }
}

#[test]
fn learn_and_gate() {
    let mut net = Network::new();
    let a = net.add_neuron_with_activation(Activation::Identity);
    let b = net.add_neuron_with_activation(Activation::Identity);
    let bias = net.add_neuron_with_activation(Activation::Identity);
    let out = net.add_neuron_with_activation(Activation::Sigmoid);

    net.add_synapse(a, out, 1.0).unwrap();
    net.add_synapse(b, out, 1.0).unwrap();
    net.add_synapse(bias, out, -1.5).unwrap();

    let dataset = [
        (vec![0.0, 0.0, 1.0], 0.0),
        (vec![0.0, 1.0, 1.0], 0.0),
        (vec![1.0, 0.0, 1.0], 0.0),
        (vec![1.0, 1.0, 1.0], 1.0),
    ];

    net.train(
        &dataset
            .iter()
            .map(|(inputs, output)| (inputs.clone(), vec![*output]))
            .collect::<Vec<_>>(),
        5000,
        0.5,
    )
    .unwrap();

    evaluate(&mut net, &dataset);
}

#[test]
fn learn_or_gate() {
    let mut net = Network::new();
    let a = net.add_neuron_with_activation(Activation::Identity);
    let b = net.add_neuron_with_activation(Activation::Identity);
    let bias = net.add_neuron_with_activation(Activation::Identity);
    let out = net.add_neuron_with_activation(Activation::Sigmoid);

    net.add_synapse(a, out, 1.0).unwrap();
    net.add_synapse(b, out, 1.0).unwrap();
    net.add_synapse(bias, out, -0.5).unwrap();

    let dataset = [
        (vec![0.0, 0.0, 1.0], 0.0),
        (vec![0.0, 1.0, 1.0], 1.0),
        (vec![1.0, 0.0, 1.0], 1.0),
        (vec![1.0, 1.0, 1.0], 1.0),
    ];

    net.train(
        &dataset
            .iter()
            .map(|(inputs, output)| (inputs.clone(), vec![*output]))
            .collect::<Vec<_>>(),
        3000,
        0.5,
    )
    .unwrap();

    evaluate(&mut net, &dataset);
}
