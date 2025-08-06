use aei_framework::Activation;

// Verify derivative computations for all activation functions.
#[test]
fn test_activation_derivatives() {
    let x = 0.5;

    // Identity derivative is constant 1.
    let id_out = Activation::Identity.apply(x);
    assert!((Activation::Identity.derivative(id_out) - 1.0).abs() < 1e-8);

    // Sigmoid derivative: s * (1 - s).
    let sig_out = Activation::Sigmoid.apply(x);
    let expected_sig = sig_out * (1.0 - sig_out);
    assert!((Activation::Sigmoid.derivative(sig_out) - expected_sig).abs() < 1e-8);

    // ReLU derivative is 1 for positive inputs, 0 for negatives.
    let relu_pos = Activation::ReLU.apply(x);
    let relu_neg = Activation::ReLU.apply(-x);
    assert!((Activation::ReLU.derivative(relu_pos) - 1.0).abs() < 1e-8);
    assert!((Activation::ReLU.derivative(relu_neg) - 0.0).abs() < 1e-8);

    // Tanh derivative: 1 - t^2.
    let tanh_out = Activation::Tanh.apply(x);
    let expected_tanh = 1.0 - tanh_out * tanh_out;
    assert!((Activation::Tanh.derivative(tanh_out) - expected_tanh).abs() < 1e-8);
}
