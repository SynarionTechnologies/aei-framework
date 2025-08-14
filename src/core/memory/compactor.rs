use super::store::{MemoryStore, Result};

/// Reduces memory storage by merging or removing items.
pub trait Compactor {
    /// Compacts the given store.
    fn compact(&mut self, store: &mut dyn MemoryStore) -> Result<()>;
}

/// No-op compactor used for tests.
#[derive(Default)]
pub struct NoopCompactor;

impl Compactor for NoopCompactor {
    fn compact(&mut self, _store: &mut dyn MemoryStore) -> Result<()> {
        Ok(())
    }
}
