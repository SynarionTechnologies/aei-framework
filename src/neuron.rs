use crate::Activation;

/// Represents a neuron within the network.
///
/// Each neuron has a unique identifier, an activation function and a
/// floating-point value representing its current state.
#[derive(Debug, Clone)]
pub struct Neuron {
    /// Unique identifier of the neuron.
    pub id: usize,
    /// Current output value of the neuron (after activation).
    pub value: f64,
    /// Activation function used by this neuron.
    pub activation: Activation,
}

impl Neuron {
    /// Creates a new neuron with the provided activation function.
    ///
    /// The neuron starts with a value of `0.0`.
    pub fn new(id: usize, activation: Activation) -> Self {
        Self {
            id,
            value: 0.0,
            activation,
        }
    }
}
