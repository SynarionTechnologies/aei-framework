//! Representation of neurons within a [`Network`].

use crate::Activation;
use uuid::Uuid;

/// Represents a neuron within the network.
///
/// Each neuron has a unique identifier, an activation function and a
/// floating-point value representing its current state.
#[derive(Debug, Clone)]
pub struct Neuron {
    /// Unique identifier of the neuron.
    pub id: usize,
    /// Globally unique identifier.
    ///
    /// TODO: replace `id` usages with this `uuid` to avoid collisions and to
    /// support serialization across processes.
    pub uuid: Uuid,
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
            uuid: Uuid::new_v4(),
            value: 0.0,
            activation,
        }
    }
}
