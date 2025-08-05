//! Dynamic neural network composed of neurons and synapses.

type EdgeList = Vec<Vec<(usize, usize)>>;
type NodeList = Vec<usize>;
type TopoOrder = Vec<usize>;
type GraphStructure = (EdgeList, EdgeList, NodeList, NodeList, TopoOrder);

use std::collections::{HashMap, VecDeque};

use crate::{Activation, Neuron, Synapse};

/// Dynamic collection of neurons and synapses forming a directed graph.
///
/// A [`Network`] can be extended at runtime by adding neurons or synapses and
/// supports propagating values through the existing connections.
#[derive(Debug, Default)]
pub struct Network {
    /// All neurons indexed by their unique identifier.
    ///
    /// TODO: migrate to `Uuid` once `Neuron` identifiers switch to globally
    /// unique values.
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

        // Track how many inputs each neuron expects so we know when to apply
        // the activation function.
        let mut in_deg: HashMap<usize, usize> = self.neurons.keys().map(|&k| (k, 0usize)).collect();
        for s in &self.synapses {
            if let Some(d) = in_deg.get_mut(&s.to) {
                *d += 1;
            }
        }

        // Accumulate weighted sums until all inputs of a neuron are processed.
        let mut sums: HashMap<usize, f64> = HashMap::new();

        // 3. Propagate weighted values.
        let mut queue = VecDeque::from([start_neuron]);
        while let Some(id) = queue.pop_front() {
            let from_value = match self.neurons.get(&id) {
                Some(n) => n.value,
                None => continue,
            };

            for syn in self.synapses.iter().filter(|s| s.from == id) {
                if !self.neurons.contains_key(&syn.to) {
                    continue;
                }
                *sums.entry(syn.to).or_insert(0.0) += from_value * syn.weight;

                if let Some(d) = in_deg.get_mut(&syn.to) {
                    *d -= 1;
                    if *d == 0 {
                        if let Some(sum) = sums.remove(&syn.to) {
                            if let Some(n) = self.neurons.get_mut(&syn.to) {
                                n.value = n.activation.apply(sum);
                            }
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

    /// Computes a forward pass for a full set of input values and returns the
    /// outputs of neurons that have no outgoing synapses.
    ///
    /// The number of provided inputs must match the number of input neurons
    /// (neurons without incoming synapses). Output values are returned in the
    /// order of their neuron identifiers.
    pub fn predict(&mut self, inputs: &[f64]) -> Vec<f64> {
        let (incoming, _outgoing, input_ids, output_ids, order) = self.graph_structure();
        let mut is_output = vec![false; self.next_id];
        for &id in &output_ids {
            is_output[id] = true;
        }
        assert_eq!(inputs.len(), input_ids.len(), "input length mismatch");

        let mut values = vec![0.0; self.next_id];
        let mut is_input = vec![false; self.next_id];
        for (i, &id) in input_ids.iter().enumerate() {
            let activation = self.neurons.get(&id).unwrap().activation;
            values[id] = activation.apply(inputs[i]);
            is_input[id] = true;
        }

        for &id in &order {
            if is_input[id] {
                continue;
            }
            let sum: f64 = incoming[id]
                .iter()
                .map(|&(idx, from)| values[from] * self.synapses[idx].weight)
                .sum();
            if let Some(n) = self.neurons.get(&id) {
                values[id] = n.activation.apply(sum);
            }
        }

        for (id, &v) in values.iter().enumerate().take(self.next_id) {
            if let Some(n) = self.neurons.get_mut(&id) {
                n.value = v;
            }
        }

        output_ids.iter().map(|&id| values[id]).collect()
    }

    /// Trains the network on a dataset using the backpropagation algorithm.
    ///
    /// Each entry in `dataset` is a pair of input values and expected output
    /// values. The number of input values must match the number of input
    /// neurons (neurons without incoming synapses) and similarly for outputs.
    /// Weights are updated using gradient descent with the provided
    /// `learning_rate`.
    pub fn train(&mut self, dataset: &[(Vec<f64>, Vec<f64>)], epochs: usize, learning_rate: f64) {
        if dataset.is_empty() || epochs == 0 {
            return;
        }

        let (incoming, outgoing, input_ids, output_ids, order) = self.graph_structure();
        let mut is_output = vec![false; self.next_id];
        for &id in &output_ids {
            is_output[id] = true;
        }

        for epoch in 0..epochs {
            let mut epoch_loss = 0.0;
            for (inputs, targets) in dataset {
                assert_eq!(inputs.len(), input_ids.len(), "input length mismatch");
                assert_eq!(targets.len(), output_ids.len(), "output length mismatch");

                let mut values = vec![0.0; self.next_id];
                let mut is_input = vec![false; self.next_id];
                for (i, &id) in input_ids.iter().enumerate() {
                    let activation = self.neurons.get(&id).unwrap().activation;
                    values[id] = activation.apply(inputs[i]);
                    is_input[id] = true;
                }

                for &id in &order {
                    if is_input[id] {
                        continue;
                    }
                    let sum: f64 = incoming[id]
                        .iter()
                        .map(|&(idx, from)| values[from] * self.synapses[idx].weight)
                        .sum();
                    if let Some(n) = self.neurons.get(&id) {
                        values[id] = n.activation.apply(sum);
                    }
                }

                let mut deltas = vec![0.0; self.next_id];
                for (i, &id) in output_ids.iter().enumerate() {
                    let output = values[id];
                    let target = targets[i];
                    if let Some(n) = self.neurons.get(&id) {
                        let error = output - target;
                        epoch_loss += 0.5 * error * error;

                        deltas[id] = error * n.activation.derivative(output);
                    }
                }

                for &id in order.iter().rev() {
                    if is_output[id] {
                        continue;
                    }
                    let mut sum = 0.0;
                    for &(idx, to) in &outgoing[id] {
                        sum += self.synapses[idx].weight * deltas[to];
                    }
                    if let Some(n) = self.neurons.get(&id) {
                        deltas[id] = n.activation.derivative(values[id]) * sum;
                    }
                }

                for syn in self.synapses.iter_mut() {
                    let grad = values[syn.from] * deltas[syn.to];
                    syn.weight -= learning_rate * grad;
                }

                for (id, &_v) in values.iter().enumerate().take(self.next_id) {
                    if let Some(n) = self.neurons.get_mut(&id) {
                        n.value = values[id];
                    }
                }
            }

            let avg_loss = epoch_loss / dataset.len() as f64;
            println!("Epoch {}/{} - loss: {}", epoch + 1, epochs, avg_loss);
        }
    }

    /// Builds adjacency lists and a topological ordering of the network.
    ///
    /// Returns `(incoming, outgoing, input_ids, output_ids, order)` where
    /// `incoming[id]` contains `(synapse_index, from_id)` pairs for edges
    /// leading into `id`, `outgoing[id]` contains `(synapse_index, to_id)`
    /// pairs for edges leaving `id`, `input_ids` are neurons with no incoming
    /// edges, `output_ids` are neurons with no outgoing edges and `order` is a
    /// topological ordering of all neurons.
    fn graph_structure(&self) -> GraphStructure {
        let num_neurons = self.next_id;
        let mut incoming = vec![Vec::new(); num_neurons];
        let mut outgoing = vec![Vec::new(); num_neurons];
        for (idx, syn) in self.synapses.iter().enumerate() {
            if syn.from < num_neurons && syn.to < num_neurons {
                incoming[syn.to].push((idx, syn.from));
                outgoing[syn.from].push((idx, syn.to));
            }
        }

        let input_ids: Vec<usize> = (0..num_neurons)
            .filter(|&i| incoming[i].is_empty())
            .collect();
        let output_ids: Vec<usize> = (0..num_neurons)
            .filter(|&i| outgoing[i].is_empty())
            .collect();

        let mut in_deg: Vec<usize> = incoming.iter().map(|v| v.len()).collect();
        let mut queue: VecDeque<usize> = input_ids.iter().copied().collect();
        let mut order = Vec::new();
        while let Some(id) = queue.pop_front() {
            order.push(id);
            for &(_, to) in &outgoing[id] {
                in_deg[to] -= 1;
                if in_deg[to] == 0 {
                    queue.push_back(to);
                }
            }
        }
        if order.len() != num_neurons {
            panic!("Network contains cycles which are not supported");
        }

        (incoming, outgoing, input_ids, output_ids, order)
    }
}
