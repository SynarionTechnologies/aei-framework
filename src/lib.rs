//! Core of the AEI Framework, a Rust framework for building dynamic and
//! modular neural networks.

pub mod activation;
pub mod network;
pub mod neuron;
pub mod synapse;

pub use activation::Activation;
pub use network::Network;
pub use neuron::Neuron;
pub use synapse::Synapse;
