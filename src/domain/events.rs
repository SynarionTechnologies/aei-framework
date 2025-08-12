//! Domain events representing state changes in the network.
//!
//! Events are persisted in an append-only log and can be replayed to
//! reconstruct the state of the system.

use super::Activation;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Business events emitted by command handlers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    /// A neuron was added to the network with a random activation.
    RandomNeuronAdded(RandomNeuronAdded),
    /// A neuron was removed from the network.
    RandomNeuronRemoved(RandomNeuronRemoved),
    /// A neuron was explicitly added to the network.
    NeuronAdded(NeuronAdded),
    /// A neuron was explicitly removed from the network.
    NeuronRemoved(NeuronRemoved),
    /// A synapse connecting two neurons was created.
    SynapseCreated {
        id: Uuid,
        from: Uuid,
        to: Uuid,
        weight: f64,
    },
    /// A synapse was removed from the network.
    SynapseRemoved { id: Uuid },
    /// A synapse between two randomly selected neurons was added.
    RandomSynapseAdded(RandomSynapseAdded),
    /// A randomly chosen synapse was removed from the network.
    RandomSynapseRemoved(RandomSynapseRemoved),
    /// The weight of an existing synapse was mutated.
    SynapseWeightMutated(SynapseWeightMutated),
    /// The weight of an existing synapse was explicitly set.
    SynapseWeightSet(SynapseWeightSet),
    /// The activation function of a neuron was mutated.
    NeuronActivationMutated(NeuronActivationMutated),
    /// The curiosity score of a neuron or synapse was updated.
    CuriosityScoreUpdated(CuriosityScoreUpdated),
}

/// Event emitted when a random neuron is added to the network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomNeuronAdded {
    /// Identifier of the created neuron.
    pub neuron_id: Uuid,
    /// Activation assigned to the neuron.
    pub activation: Activation,
}

/// Event emitted when a random neuron is removed from the network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomNeuronRemoved {
    /// Identifier of the removed neuron.
    pub neuron_id: Uuid,
}

/// Event emitted when a neuron is added to the network.
///
/// # Examples
///
/// ```
/// use aei_framework::{Activation, Event, NeuronAdded};
/// use uuid::Uuid;
///
/// let id = Uuid::new_v4();
/// let event = Event::NeuronAdded(NeuronAdded { neuron_id: id, activation: Activation::ReLU });
/// # let _ = event;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuronAdded {
    /// Identifier of the created neuron.
    pub neuron_id: Uuid,
    /// Activation assigned to the neuron.
    pub activation: Activation,
}

/// Event emitted when a neuron is removed from the network.
///
/// # Examples
///
/// ```
/// use aei_framework::{Event, NeuronRemoved};
/// use uuid::Uuid;
///
/// let id = Uuid::new_v4();
/// let event = Event::NeuronRemoved(NeuronRemoved { neuron_id: id });
/// # let _ = event;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuronRemoved {
    /// Identifier of the removed neuron.
    pub neuron_id: Uuid,
}

/// Event emitted when a random synapse is created between two existing neurons.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomSynapseAdded {
    /// Identifier of the new synapse.
    pub synapse_id: Uuid,
    /// Source neuron of the synapse.
    pub from: Uuid,
    /// Target neuron of the synapse.
    pub to: Uuid,
    /// Weight associated with the synapse.
    pub weight: f64,
}

/// Event emitted when a random synapse is removed from the network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomSynapseRemoved {
    /// Identifier of the removed synapse.
    pub synapse_id: Uuid,
}

/// Event emitted when the weight of a synapse changes due to mutation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynapseWeightMutated {
    /// Identifier of the mutated synapse.
    pub synapse_id: Uuid,
    /// Previous weight of the synapse before mutation.
    pub old_weight: f64,
    /// Newly assigned weight after mutation.
    pub new_weight: f64,
}

/// Event emitted when the weight of a synapse is set explicitly.
///
/// # Examples
///
/// ```
/// use aei_framework::{Event, SynapseWeightSet};
/// use uuid::Uuid;
///
/// let id = Uuid::new_v4();
/// let event = Event::SynapseWeightSet(SynapseWeightSet {
///     synapse_id: id,
///     old_weight: 0.2,
///     new_weight: 0.5,
/// });
/// # let _ = event;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynapseWeightSet {
    /// Identifier of the updated synapse.
    pub synapse_id: Uuid,
    /// Previous weight of the synapse before the update.
    pub old_weight: f64,
    /// New weight assigned to the synapse.
    pub new_weight: f64,
}

/// Event emitted when the activation of a neuron changes due to mutation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuronActivationMutated {
    /// Identifier of the mutated neuron.
    pub neuron_id: Uuid,
    /// Activation function prior to mutation.
    pub old_activation: Activation,
    /// Newly assigned activation function.
    pub new_activation: Activation,
}

/// Event emitted when a curiosity score changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuriosityScoreUpdated {
    /// Identifier of the neuron or synapse.
    pub target_id: Uuid,
    /// Previous curiosity score.
    pub old_score: f64,
    /// Newly computed curiosity score.
    pub new_score: f64,
}
