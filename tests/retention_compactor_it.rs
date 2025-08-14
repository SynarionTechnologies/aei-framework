use aei_framework::core::memory::{
    Compactor, InMemoryStore, MemoryItem, NoopCompactor, RetentionAction, RetentionPolicy,
    TtlRetentionPolicy,
};

#[test]
fn ttl_retention_deletes_old_items() {
    let mut item = MemoryItem::new("temp");
    // Set timestamp in the past
    item.timestamp = chrono::Utc::now() - chrono::Duration::seconds(2);
    let policy = TtlRetentionPolicy::new(chrono::Duration::seconds(1));
    assert_eq!(policy.evaluate(&item), RetentionAction::Delete);
}

#[test]
fn noop_compactor_runs_without_error() {
    let mut store = InMemoryStore::new();
    let mut compactor = NoopCompactor::default();
    compactor.compact(&mut store).unwrap();
}
