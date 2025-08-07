//! Commands describing intent to change the domain state.

use uuid::Uuid;

/// Write-side operations handled by the [`CommandHandler`].
#[derive(Debug, Clone)]
pub enum Command {
    /// Create a synapse between two existing neurons.
    CreateSynapse {
        id: Uuid,
        from: Uuid,
        to: Uuid,
        weight: f64,
    },
    /// Delete a synapse by its identifier.
    RemoveSynapse { id: Uuid },
}
