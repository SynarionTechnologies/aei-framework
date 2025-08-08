//! Activation functions available for neurons.
//!
//! Each variant provides a mathematical transformation applied to the input
//! value during propagation. More functions can be added in the future by
//! extending this enum.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
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

    /// Returns the derivative of the activation function given its output.
    ///
    /// The derivative is expressed in terms of the already activated output in
    /// order to avoid recomputing the forward pass during backpropagation.
    #[must_use]
    pub fn derivative(self, activated: f64) -> f64 {
        match self {
            Activation::Identity => 1.0,
            Activation::Sigmoid => activated * (1.0 - activated),
            Activation::ReLU => {
                if activated > 0.0 {
                    1.0
                } else {
                    0.0
                }
            }
            Activation::Tanh => 1.0 - activated * activated,
        }
    }
}
