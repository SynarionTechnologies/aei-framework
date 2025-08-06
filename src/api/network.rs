//! Dynamic neural network composed of neurons and synapses.

type EdgeList = Vec<Vec<(usize, usize)>>;
type NodeList = Vec<usize>;
type TopoOrder = Vec<usize>;
type GraphStructure = (EdgeList, EdgeList, NodeList, NodeList, TopoOrder);

use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::{Activation, Neuron, Synapse};

/// Dynamic collection of neurons and synapses forming a directed graph.
///
/// A [`Network`] can be extended at runtime by adding neurons or synapses and
/// supports propagating values through the existing connections.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Network {
    /// All neurons stored contiguously.
    neurons: Vec<Neuron>,
    /// Map from neuron [`Uuid`] to its index within `neurons`.
    #[serde(skip)]
    neuron_indices: HashMap<Uuid, usize>,
    /// Directed connections transferring weighted signals between neurons.
    synapses: Vec<Synapse>,
    /// Mapping from logical input names to neuron ids.
    pub input_neurons: HashMap<String, Uuid>,
    /// Mapping from logical output names to neuron ids.
    pub output_neurons: HashMap<String, Uuid>,
    /// Ordered list of input neuron ids for index-based access.
    input_order: Vec<Uuid>,
    /// Ordered list of output neuron ids for index-based access.
    output_order: Vec<Uuid>,
    /// Temporary storage for values set through [`set_inputs`] before propagation.
    #[serde(skip)]
    input_buffer: HashMap<Uuid, f64>,
}

/// Errors that can occur when manipulating a [`Network`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkError {
    /// A provided neuron identifier does not exist in the network.
    UnknownNeuron,
    /// The network contains a cycle and cannot be processed.
    CycleDetected,
}

impl std::fmt::Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownNeuron => write!(f, "unknown neuron"),
            Self::CycleDetected => write!(f, "cycle detected"),
        }
    }
}

impl std::error::Error for NetworkError {}

impl Network {
    /// Creates an empty network.
    pub fn new() -> Self {
        Self::default()
    }

    /// Saves the network as pretty JSON to the specified file path.
    pub fn save_json<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, self).map_err(io::Error::other)
    }

    /// Loads a network from a JSON file created by [`save_json`].
    pub fn load_json<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let mut net: Network = serde_json::from_reader(file).map_err(io::Error::other)?;
        net.rebuild_indices();
        Ok(net)
    }

    fn rebuild_indices(&mut self) {
        self.neuron_indices = self
            .neurons
            .iter()
            .enumerate()
            .map(|(i, n)| (n.id, i))
            .collect();
        self.input_buffer.clear();
    }

    /// Adds a neuron using the default [`Activation::Identity`].
    ///
    /// Returns the identifier assigned to the new neuron.
    pub fn add_neuron(&mut self) -> Uuid {
        self.add_neuron_with_activation(Activation::Identity)
    }

    /// Adds a neuron with a specified activation function and returns its id.
    pub fn add_neuron_with_activation(&mut self, activation: Activation) -> Uuid {
        let neuron = Neuron::new(activation);
        let id = neuron.id;
        self.neuron_indices.insert(id, self.neurons.len());
        self.neurons.push(neuron);
        id
    }

    /// Adds a neuron with an explicit [`Uuid`].
    pub fn add_neuron_with_id(&mut self, id: Uuid, activation: Activation) -> Uuid {
        let neuron = Neuron::with_id(id, activation);
        let id = neuron.id;
        self.neuron_indices.insert(id, self.neurons.len());
        self.neurons.push(neuron);
        id
    }

    /// Adds a neuron designated as an input with the given `name` and `activation`.
    ///
    /// Returns the internal identifier assigned to the new neuron.
    pub fn add_input_neuron(&mut self, name: &str, activation: Activation) -> Uuid {
        let id = self.add_neuron_with_activation(activation);
        self.input_neurons.insert(name.to_string(), id);
        self.input_order.push(id);
        id
    }

    /// Adds a neuron designated as an output with the given `name` and `activation`.
    ///
    /// Returns the internal identifier assigned to the new neuron.
    pub fn add_output_neuron(&mut self, name: &str, activation: Activation) -> Uuid {
        let id = self.add_neuron_with_activation(activation);
        self.output_neurons.insert(name.to_string(), id);
        self.output_order.push(id);
        id
    }

    /// Adds a directed synapse between two neuron identifiers.
    ///
    /// Returns an error if either identifier does not correspond to an existing
    /// neuron.
    pub fn add_synapse(&mut self, from: Uuid, to: Uuid, weight: f64) -> Result<Uuid, NetworkError> {
        if !self.neuron_indices.contains_key(&from) || !self.neuron_indices.contains_key(&to) {
            return Err(NetworkError::UnknownNeuron);
        }
        let syn = Synapse::new(from, to, weight);
        let id = syn.id;
        self.synapses.push(syn);
        Ok(id)
    }

    /// Adds a synapse with an explicit [`Uuid`].
    pub fn add_synapse_with_id(
        &mut self,
        id: Uuid,
        from: Uuid,
        to: Uuid,
        weight: f64,
    ) -> Result<Uuid, NetworkError> {
        if !self.neuron_indices.contains_key(&from) || !self.neuron_indices.contains_key(&to) {
            return Err(NetworkError::UnknownNeuron);
        }
        let syn = Synapse::with_id(id, from, to, weight);
        self.synapses.push(syn);
        Ok(id)
    }

    /// Assigns values to input neurons identified by name.
    pub fn set_inputs(&mut self, values: &[(&str, f64)]) {
        for (name, value) in values {
            if let Some(&id) = self.input_neurons.get(*name) {
                self.input_buffer.insert(id, *value);
            }
        }
    }

    /// Assigns values to input neurons by their logical index.
    pub fn set_inputs_by_index(&mut self, values: &[f64]) {
        assert_eq!(
            values.len(),
            self.input_order.len(),
            "input length mismatch"
        );
        for (&id, &value) in self.input_order.iter().zip(values.iter()) {
            self.input_buffer.insert(id, value);
        }
    }

    /// Propagates previously assigned inputs through the network.
    pub fn propagate_inputs(&mut self) -> Result<(), NetworkError> {
        let (incoming, _outgoing, _auto_inputs, _auto_outputs, order) = self.graph_structure()?;
        let num_neurons = self.neurons.len();
        let mut values = vec![0.0; num_neurons];
        let mut is_input = vec![false; num_neurons];

        for &id in self.input_neurons.values() {
            if let Some(&idx) = self.neuron_indices.get(&id) {
                let activation = self.neurons[idx].activation;
                let v = *self.input_buffer.get(&id).unwrap_or(&0.0);
                values[idx] = activation.apply(v);
                is_input[idx] = true;
            }
        }

        for &idx in &order {
            if is_input[idx] {
                continue;
            }
            let sum: f64 = incoming[idx]
                .iter()
                .map(|&(s_idx, from_idx)| values[from_idx] * self.synapses[s_idx].weight)
                .sum();
            values[idx] = self.neurons[idx].activation.apply(sum);
        }

        for (idx, &v) in values.iter().enumerate() {
            if let Some(n) = self.neurons.get_mut(idx) {
                n.value = v;
            }
        }

        self.input_buffer.clear();
        Ok(())
    }

    /// Returns the values of all output neurons identified by name.
    pub fn get_outputs(&self) -> HashMap<String, f64> {
        self.output_neurons
            .iter()
            .filter_map(|(name, &id)| {
                self.neuron_indices
                    .get(&id)
                    .and_then(|&idx| self.neurons.get(idx).map(|n| (name.clone(), n.value)))
            })
            .collect()
    }

    /// Returns the values of all output neurons ordered by their logical index.
    pub fn get_outputs_by_index(&self) -> Vec<f64> {
        self.output_order
            .iter()
            .map(|&id| {
                self.neuron_indices
                    .get(&id)
                    .and_then(|&idx| self.neurons.get(idx).map(|n| n.value))
                    .unwrap_or(0.0)
            })
            .collect()
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
    /// use aei_framework::{Activation, Network};
    ///
    /// let mut net = Network::new();
    /// let a = net.add_neuron_with_activation(Activation::Sigmoid);
    /// let b = net.add_neuron_with_activation(Activation::ReLU);
    /// net.add_synapse(a, b, 2.0).unwrap();
    /// net.propagate(a, 1.0);
    /// ```
    pub fn propagate(&mut self, start: Uuid, value: f64) {
        // 1. Reset all neuron values.
        for neuron in self.neurons.iter_mut() {
            neuron.value = 0.0;
        }

        // 2. Activate the source neuron with the incoming value.
        let start_idx = match self.neuron_indices.get(&start).copied() {
            Some(i) => i,
            None => return,
        };
        let start_id = self.neurons[start_idx].id;
        let activation = self.neurons[start_idx].activation;
        self.neurons[start_idx].value = activation.apply(value);

        // Track how many inputs each neuron expects so we know when to apply
        // the activation function.
        let mut in_deg: HashMap<Uuid, usize> =
            self.neurons.iter().map(|n| (n.id, 0usize)).collect();
        for s in &self.synapses {
            if let Some(d) = in_deg.get_mut(&s.to) {
                *d += 1;
            }
        }

        // Accumulate weighted sums until all inputs of a neuron are processed.
        let mut sums: HashMap<Uuid, f64> = HashMap::new();

        // 3. Propagate weighted values.
        let mut queue = VecDeque::from([start_id]);
        while let Some(id) = queue.pop_front() {
            let from_idx = match self.neuron_indices.get(&id).copied() {
                Some(i) => i,
                None => continue,
            };
            let from_value = self.neurons[from_idx].value;

            for syn in self.synapses.iter().filter(|s| s.from == id) {
                if !self.neuron_indices.contains_key(&syn.to) {
                    continue;
                }
                *sums.entry(syn.to).or_insert(0.0) += from_value * syn.weight;

                if let Some(d) = in_deg.get_mut(&syn.to) {
                    *d -= 1;
                    if *d == 0 {
                        if let Some(sum) = sums.remove(&syn.to) {
                            if let Some(&to_idx) = self.neuron_indices.get(&syn.to) {
                                let activation = self.neurons[to_idx].activation;
                                self.neurons[to_idx].value = activation.apply(sum);
                            }
                        }
                        queue.push_back(syn.to);
                    }
                }
            }
        }
    }

    /// Returns the current value of a neuron, if it exists.
    pub fn value(&self, id: Uuid) -> Option<f64> {
        self.neuron_indices
            .get(&id)
            .and_then(|&idx| self.neurons.get(idx).map(|n| n.value))
    }

    /// Computes a forward pass for a full set of input values and returns the
    /// outputs of neurons that have no outgoing synapses.
    ///
    /// The number of provided inputs must match the number of input neurons
    /// (neurons without incoming synapses). Output values are returned in the
    /// order of their neuron identifiers.
    pub fn predict(&mut self, inputs: &[f64]) -> Result<Vec<f64>, NetworkError> {
        let (incoming, _outgoing, input_ids, output_ids, order) = self.graph_structure()?;
        let num_neurons = self.neurons.len();
        let mut is_output = vec![false; num_neurons];
        for &idx in &output_ids {
            is_output[idx] = true;
        }
        assert_eq!(inputs.len(), input_ids.len(), "input length mismatch");

        let mut values = vec![0.0; num_neurons];
        let mut is_input = vec![false; num_neurons];
        for (i, &idx) in input_ids.iter().enumerate() {
            let activation = self.neurons[idx].activation;
            values[idx] = activation.apply(inputs[i]);
            is_input[idx] = true;
        }

        for &idx in &order {
            if is_input[idx] {
                continue;
            }
            let sum: f64 = incoming[idx]
                .iter()
                .map(|&(s_idx, from_idx)| values[from_idx] * self.synapses[s_idx].weight)
                .sum();
            values[idx] = self.neurons[idx].activation.apply(sum);
        }

        for (idx, &v) in values.iter().enumerate() {
            if let Some(n) = self.neurons.get_mut(idx) {
                n.value = v;
            }
        }

        Ok(output_ids.iter().map(|&idx| values[idx]).collect())
    }

    /// Trains the network on a dataset using the backpropagation algorithm.
    ///
    /// Each entry in `dataset` is a pair of input values and expected output
    /// values. The number of input values must match the number of input
    /// neurons (neurons without incoming synapses) and similarly for outputs.
    /// Weights are updated using gradient descent with the provided
    /// `learning_rate`.
    pub fn train(
        &mut self,
        dataset: &[(Vec<f64>, Vec<f64>)],
        epochs: usize,
        learning_rate: f64,
    ) -> Result<(), NetworkError> {
        if dataset.is_empty() || epochs == 0 {
            return Ok(());
        }

        let (incoming, outgoing, input_ids, output_ids, order) = self.graph_structure()?;
        let num_neurons = self.neurons.len();
        let mut is_output = vec![false; num_neurons];
        for &idx in &output_ids {
            is_output[idx] = true;
        }

        for epoch in 0..epochs {
            let mut epoch_loss = 0.0;
            for (inputs, targets) in dataset {
                assert_eq!(inputs.len(), input_ids.len(), "input length mismatch");
                assert_eq!(targets.len(), output_ids.len(), "output length mismatch");

                let mut values = vec![0.0; num_neurons];
                let mut is_input = vec![false; num_neurons];
                for (i, &idx) in input_ids.iter().enumerate() {
                    let activation = self.neurons[idx].activation;
                    values[idx] = activation.apply(inputs[i]);
                    is_input[idx] = true;
                }

                for &idx in &order {
                    if is_input[idx] {
                        continue;
                    }
                    let sum: f64 = incoming[idx]
                        .iter()
                        .map(|&(s_idx, from_idx)| values[from_idx] * self.synapses[s_idx].weight)
                        .sum();
                    values[idx] = self.neurons[idx].activation.apply(sum);
                }

                let mut deltas = vec![0.0; num_neurons];
                for (i, &idx) in output_ids.iter().enumerate() {
                    let output = values[idx];
                    let target = targets[i];
                    let error = output - target;
                    epoch_loss += 0.5 * error * error;

                    deltas[idx] = error * self.neurons[idx].activation.derivative(output);
                }

                for &idx in order.iter().rev() {
                    if is_output[idx] {
                        continue;
                    }
                    let mut sum = 0.0;
                    for &(s_idx, to_idx) in &outgoing[idx] {
                        sum += self.synapses[s_idx].weight * deltas[to_idx];
                    }
                    deltas[idx] = self.neurons[idx].activation.derivative(values[idx]) * sum;
                }

                for syn in self.synapses.iter_mut() {
                    let from_idx = match self.neuron_indices.get(&syn.from) {
                        Some(&i) => i,
                        None => continue,
                    };
                    let to_idx = match self.neuron_indices.get(&syn.to) {
                        Some(&i) => i,
                        None => continue,
                    };
                    let grad = values[from_idx] * deltas[to_idx];
                    syn.weight -= learning_rate * grad;
                }

                for (idx, &_v) in values.iter().enumerate() {
                    if let Some(n) = self.neurons.get_mut(idx) {
                        n.value = values[idx];
                    }
                }
            }

            let avg_loss = epoch_loss / dataset.len() as f64;
            println!("Epoch {}/{} - loss: {}", epoch + 1, epochs, avg_loss);
        }
        Ok(())
    }

    /// Builds adjacency lists and a topological ordering of the network.
    ///
    /// Returns `(incoming, outgoing, input_ids, output_ids, order)` where
    /// `incoming[id]` contains `(synapse_index, from_id)` pairs for edges
    /// leading into `id`, `outgoing[id]` contains `(synapse_index, to_id)`
    /// pairs for edges leaving `id`, `input_ids` are neurons with no incoming
    /// edges, `output_ids` are neurons with no outgoing edges and `order` is a
    /// topological ordering of all neurons.
    fn graph_structure(&self) -> Result<GraphStructure, NetworkError> {
        let num_neurons = self.neurons.len();
        let mut incoming = vec![Vec::new(); num_neurons];
        let mut outgoing = vec![Vec::new(); num_neurons];
        for (idx, syn) in self.synapses.iter().enumerate() {
            let from_idx = match self.neuron_indices.get(&syn.from) {
                Some(&i) => i,
                None => continue,
            };
            let to_idx = match self.neuron_indices.get(&syn.to) {
                Some(&i) => i,
                None => continue,
            };
            incoming[to_idx].push((idx, from_idx));
            outgoing[from_idx].push((idx, to_idx));
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
        while let Some(idx) = queue.pop_front() {
            order.push(idx);
            for &(_, to_idx) in &outgoing[idx] {
                in_deg[to_idx] -= 1;
                if in_deg[to_idx] == 0 {
                    queue.push_back(to_idx);
                }
            }
        }
        if order.len() != num_neurons {
            return Err(NetworkError::CycleDetected);
        }

        Ok((incoming, outgoing, input_ids, output_ids, order))
    }
}
