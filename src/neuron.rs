/// Represents a basic neuron within the network.
///
/// Each neuron has a unique identifier and a floating-point value
/// representing its current state. The activation function used here is
/// the identity: the neuron's value is returned unchanged.
#[derive(Debug, Clone)]
pub struct Neuron {
    /// Unique identifier of the neuron.
    pub id: usize,
    /// Current value of the neuron.
    pub value: f64,
}

impl Neuron {
    /// Creates a new neuron with an initial value of `0.0`.
    pub fn new(id: usize) -> Self {
        Self { id, value: 0.0 }
    }

    /// Applies the neuron's activation function.
    ///
    /// For this MVP, activation is the identity and simply returns the
    /// neuron's current value.
    pub fn activation(&self) -> f64 {
        self.value
    }
}
