//! Synapse connecting two neurons.
//!
//! A synapse carries the value from a source neuron to a target neuron,
//! multiplying it by a weight.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Synapse {
    /// Globally unique identifier of the synapse.
    pub id: Uuid,
    /// Identifier of the source neuron.
    pub from: Uuid,
    /// Identifier of the target neuron.
    pub to: Uuid,
    /// Weight applied during propagation.
    pub weight: f64,
}

impl Synapse {
    /// Creates a new directed synapse with a random [`Uuid`].
    pub fn new(from: Uuid, to: Uuid, weight: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            from,
            to,
            weight,
        }
    }

    /// Creates a synapse using the supplied [`Uuid`].
    pub fn with_id(id: Uuid, from: Uuid, to: Uuid, weight: f64) -> Self {
        Self {
            id,
            from,
            to,
            weight,
        }
    }
}
