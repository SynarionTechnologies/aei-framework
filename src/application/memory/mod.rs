//! Application layer for adaptive memory commands and queries.

pub mod commands;
pub mod handlers;
pub mod queries;
pub mod query_handler;

pub use commands::{
    AddMemoryEntryCommand, PruneMemoryCommand, RemoveMemoryEntryCommand, UpdateMemoryScoreCommand,
};
pub use handlers::{
    AddMemoryEntryError, AddMemoryEntryHandler, PruneMemoryError, PruneMemoryHandler,
    RemoveMemoryEntryError, RemoveMemoryEntryHandler, UpdateMemoryScoreError,
    UpdateMemoryScoreHandler,
};
pub use queries::MemoryQuery;
pub use query_handler::{MemoryQueryHandler, MemoryQueryResult};
