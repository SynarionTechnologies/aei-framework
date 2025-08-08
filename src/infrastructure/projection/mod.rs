//! Projections translating event streams into queryable read models.

mod memory_projection;
mod network;

pub use memory_projection::MemoryProjection;
pub use network::NetworkProjection;
