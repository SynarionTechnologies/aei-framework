/// Represents a synapse connecting two neurons.
///
/// A synapse carries the value from a source neuron to a target neuron,
/// multiplying it by a weight.
#[derive(Debug, Clone)]
pub struct Synapse {
    /// Identifier of the source neuron.
    pub from: usize,
    /// Identifier of the target neuron.
    pub to: usize,
    /// Weight applied during propagation.
    pub weight: f64,
}

impl Synapse {
    /// Creates a new directed synapse.
    pub fn new(from: usize, to: usize, weight: f64) -> Self {
        Self { from, to, weight }
    }
}
