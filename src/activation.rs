//! Activation functions available for neurons.
//!
//! Each variant provides a mathematical transformation applied to the input
//! value during propagation. More functions can be added in the future by
//! extending this enum.
#[derive(Debug, Clone, Copy, Default)]
pub enum Activation {
    /// Returns the input unchanged.
    #[default]
    Identity,
    /// Logistic sigmoid: `1 / (1 + e^{-x})`.
    Sigmoid,
    /// Rectified Linear Unit: `max(0, x)`.
    ReLU,
    /// Hyperbolic tangent function.
    Tanh,
}

impl Activation {
    /// Applies the activation function to the provided value.
    #[must_use]
    pub fn apply(self, x: f64) -> f64 {
        match self {
            Activation::Identity => x,
            Activation::Sigmoid => 1.0 / (1.0 + (-x).exp()),
            Activation::ReLU => x.max(0.0),
            Activation::Tanh => x.tanh(),
        }
    }
}
