use chrono::{Duration, Utc};

use super::store::MemoryItem;

/// Action decided by a [`RetentionPolicy`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetentionAction {
    /// Keep the item as-is.
    Keep,
    /// Archive the item to long-term storage.
    Archive,
    /// Permanently remove the item.
    Delete,
}

/// Evaluates whether memory items should be kept, archived, or deleted.
pub trait RetentionPolicy {
    /// Evaluates the given item and returns the action to apply.
    fn evaluate(&self, item: &MemoryItem) -> RetentionAction;
}

/// Simple time-to-live retention policy.
pub struct TtlRetentionPolicy {
    ttl: Duration,
}

impl TtlRetentionPolicy {
    /// Creates a policy that deletes items older than the given TTL.
    pub fn new(ttl: Duration) -> Self {
        Self { ttl }
    }
}

impl RetentionPolicy for TtlRetentionPolicy {
    fn evaluate(&self, item: &MemoryItem) -> RetentionAction {
        if Utc::now() - item.timestamp > self.ttl {
            RetentionAction::Delete
        } else {
            RetentionAction::Keep
        }
    }
}
