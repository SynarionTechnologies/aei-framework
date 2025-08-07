//! Application layer coordinating commands and queries.

mod add_random_neuron;
mod add_random_synapse;
mod command_handler;
mod commands;
mod queries;
mod query_handler;
mod remove_random_neuron;
mod remove_random_synapse;

pub use add_random_neuron::{AddRandomNeuronCommand, AddRandomNeuronError, AddRandomNeuronHandler};
pub use add_random_synapse::{
    AddRandomSynapseCommand, AddRandomSynapseError, AddRandomSynapseHandler,
};
pub use command_handler::CommandHandler;
pub use commands::Command;
pub use queries::Query;
pub use query_handler::{QueryHandler, QueryResult};
pub use remove_random_neuron::{
    RemoveRandomNeuronCommand, RemoveRandomNeuronError, RemoveRandomNeuronHandler,
};
pub use remove_random_synapse::{
    RemoveRandomSynapseCommand, RemoveRandomSynapseError, RemoveRandomSynapseHandler,
};
