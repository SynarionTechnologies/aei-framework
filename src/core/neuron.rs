//! Representation of neurons within a [`Network`].

use crate::Activation;
use uuid::Uuid;

/// Represents a neuron within the network.
///
/// Each neuron has a unique identifier, an activation function and a
/// floating-point value representing its current state.
#[derive(Debug, Clone)]
pub struct Neuron {
    /// Globally unique identifier of the neuron.
    pub id: Uuid,
    /// Current output value of the neuron (after activation).
    pub value: f64,
    /// Activation function used by this neuron.
    pub activation: Activation,
}

impl Neuron {
    /// Creates a new neuron with the provided activation function and a fresh
    /// random [`Uuid`].
    pub fn new(activation: Activation) -> Self {
        Self {
            id: Uuid::new_v4(),
            value: 0.0,
            activation,
        }
    }

    /// Creates a neuron using the supplied [`Uuid`].
    pub fn with_id(id: Uuid, activation: Activation) -> Self {
        Self {
            id,
            value: 0.0,
            activation,
        }
    }
}
