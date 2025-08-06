//! Core of the AEI Framework, a Rust framework for building dynamic and
//! modular neural networks.

pub mod api;
pub mod core;

pub use api::Network;
pub use core::{Activation, Neuron, Synapse};
