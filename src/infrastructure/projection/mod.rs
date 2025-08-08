//! Projections translating event streams into queryable read models.

mod memory_projection;
mod network;
mod curiosity;

pub use memory_projection::MemoryProjection;
pub use network::NetworkProjection;
pub use curiosity::CuriosityScoreProjection;
