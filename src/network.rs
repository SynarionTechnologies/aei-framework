use std::collections::HashMap;

use crate::{Activation, Neuron, Synapse};

/// Dynamic collection of neurons and synapses forming a directed graph.
///
/// A [`Network`] can be extended at runtime by adding neurons or synapses and
/// supports propagating values through the existing connections.
#[derive(Debug, Default)]
pub struct Network {
    /// All neurons indexed by their unique identifier.
    neurons: HashMap<usize, Neuron>,
    /// Directed connections transferring weighted signals between neurons.
    synapses: Vec<Synapse>,
    /// Identifier assigned to the next neuron added to the network.
    next_id: usize,
}

impl Network {
    /// Creates an empty network.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a neuron using the default [`Activation::Identity`].
    ///
    /// Returns the identifier assigned to the new neuron.
    pub fn add_neuron(&mut self) -> usize {
        self.add_neuron_with_activation(Activation::Identity)
    }

    /// Adds a neuron with a specified activation function and returns its id.
    pub fn add_neuron_with_activation(&mut self, activation: Activation) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.neurons.insert(id, Neuron::new(id, activation));
        id
    }

    /// Adds a directed synapse between two neuron identifiers.
    ///
    /// If either identifier does not correspond to an existing neuron the
    /// synapse is still recorded but will have no effect during propagation.
    pub fn add_synapse(&mut self, from: usize, to: usize, weight: f64) {
        self.synapses.push(Synapse::new(from, to, weight));
    }

    /// Propagates a value through the network starting from `start`.
    ///
    /// # Propagation sequence
    /// 1. **Reset** – all neuron values are cleared to `0.0`.
    /// 2. **Source activation** – the input `value` is transformed by the source
    ///    neuron's activation function.
    /// 3. **Weighting** – each synapse transmits `from_value * weight` to its
    ///    target neuron.
    /// 4. **Activation** – once all sums are collected, every target neuron
    ///    applies its activation function.
    ///
    /// # Edge cases
    /// - If the source neuron does not exist, the propagation stops immediately.
    /// - Synapses referencing missing neurons are ignored.
    /// - Orphan synapses (whose source neuron is absent) never fire.
    /// - The reset step guarantees that consecutive propagations are
    ///   independent.
    ///
    /// # Example
    /// ```
    /// use aei_framework::{activation::Activation, network::Network};
    ///
    /// let mut net = Network::new();
    /// let a = net.add_neuron_with_activation(Activation::Sigmoid);
    /// let b = net.add_neuron_with_activation(Activation::ReLU);
    /// net.add_synapse(a, b, 2.0);
    /// net.propagate(a, 1.0);
    /// ```
    pub fn propagate(&mut self, start: usize, value: f64) {
        use std::collections::VecDeque;

        // 1. Reset all neuron values.
        for neuron in self.neurons.values_mut() {
            neuron.value = 0.0;
        }

        // 2. Activate the source neuron with the incoming value.
        let start_neuron = match self.neurons.get_mut(&start) {
            Some(n) => {
                n.value = n.activation.apply(value);
                n.id
            }
            None => return,
        };

        // Pre-compute the number of incoming synapses for each neuron. This is
        // required so we only activate a neuron after all of its inputs have
        // been processed.
        let mut in_deg: HashMap<usize, usize> = self.neurons.keys().map(|&k| (k, 0usize)).collect();
        for s in &self.synapses {
            if let Some(d) = in_deg.get_mut(&s.to) {
                *d += 1;
            }
        }

        // 3. Propagate weighted sums through the synapses using a queue.
        let mut queue = VecDeque::from([start_neuron]);
        while let Some(id) = queue.pop_front() {
            let from_value = match self.neurons.get(&id) {
                Some(n) => n.value,
                None => continue,
            };

            for syn in self.synapses.iter().filter(|s| s.from == id) {
                let weighted = from_value * syn.weight;
                if let Some(target) = self.neurons.get_mut(&syn.to) {
                    target.value += weighted;
                }

                // Decrease the remaining input count and, if all inputs are
                // processed, apply the activation and enqueue the neuron so its
                // outputs can be propagated further.
                if let Some(d) = in_deg.get_mut(&syn.to) {
                    *d -= 1;
                    if *d == 0 {
                        if let Some(n) = self.neurons.get_mut(&syn.to) {
                            n.value = n.activation.apply(n.value);
                        }
                        queue.push_back(syn.to);
                    }
                }
            }
        }
    }

    /// Returns the current value of a neuron, if it exists.
    pub fn value(&self, id: usize) -> Option<f64> {
        self.neurons.get(&id).map(|n| n.value)
    }
}
