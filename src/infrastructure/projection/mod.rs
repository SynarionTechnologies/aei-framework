//! Projections translating event streams into queryable read models.

mod curiosity;
mod memory_projection;
mod network;

pub use curiosity::CuriosityScoreProjection;
pub use memory_projection::MemoryProjection;
pub use network::NetworkProjection;
