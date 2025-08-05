use std::collections::HashMap;

use crate::{Neuron, Synapse};

/// Manager for neurons and synapses.
///
/// The network allows adding neurons or synapses on the fly and
/// propagating values through existing connections.
#[derive(Debug, Default)]
pub struct Network {
    neurons: HashMap<usize, Neuron>,
    synapses: Vec<Synapse>,
    next_id: usize,
}

impl Network {
    /// Creates an empty network.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a neuron and returns its unique identifier.
    pub fn add_neuron(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.neurons.insert(id, Neuron::new(id));
        id
    }

    /// Adds a directed synapse between two existing neurons.
    ///
    /// # Panics
    /// Panics if either neuron does not exist in the network.
    pub fn add_synapse(&mut self, from: usize, to: usize, weight: f64) {
        if !(self.neurons.contains_key(&from) && self.neurons.contains_key(&to)) {
            panic!("nonexistent neuron");
        }
        self.synapses.push(Synapse::new(from, to, weight));
    }

    /// Propagates a value from a source neuron.
    ///
    /// Propagation follows directed synapses and applies successive weights.
    /// Each neuron uses the identity activation function.
    pub fn propagate(&mut self, start: usize, value: f64) {
        if let Some(n) = self.neurons.get_mut(&start) {
            n.value = value;
        } else {
            return;
        }

        let synapses = self.synapses.clone();
        let mut stack = vec![start];
        while let Some(id) = stack.pop() {
            let current = {
                let n = self.neurons.get(&id).unwrap();
                n.activation()
            };

            for s in synapses.iter().filter(|s| s.from == id) {
                if let Some(neuron) = self.neurons.get_mut(&s.to) {
                    neuron.value = current * s.weight;
                }
                stack.push(s.to);
            }
        }
    }

    /// Returns the current value of a neuron, if it exists.
    pub fn value(&self, id: usize) -> Option<f64> {
        self.neurons.get(&id).map(|n| n.value)
    }
}
