//! Demonstrates basic memory operations using a temporary file-backed event store.
//!
//! This example shows how to:
//! - insert a memory entry with `AddMemoryEntryHandler`.
//! - update its score using `UpdateMemoryScoreHandler`.
//! - query the top entries through `MemoryProjection`.

use aei_framework::{
    application::memory::{
        AddMemoryEntryCommand, AddMemoryEntryHandler, UpdateMemoryScoreCommand,
        UpdateMemoryScoreHandler,
    },
    infrastructure::{projection::MemoryProjection, FileMemoryEventStore},
};
use serde_json::json;
use uuid::Uuid;

/// Runs the memory example.
fn main() {
    env_logger::init();
    let path = std::env::temp_dir().join(format!("aei_memory_{}.log", Uuid::new_v4()));
    let store = FileMemoryEventStore::new(path.clone());

    // Insert a new memory entry.
    let mut add = AddMemoryEntryHandler::new(store, 10).expect("store");
    let entry_id = add
        .handle(AddMemoryEntryCommand {
            event_type: "interaction".into(),
            payload: json!({"msg": "hello"}),
            score: 0.7,
        })
        .expect("add entry");

    // Update the score of that entry.
    let store = add.base.store;
    let mut update = UpdateMemoryScoreHandler::new(store, 10).expect("store");
    update
        .handle(UpdateMemoryScoreCommand {
            entry_id,
            new_score: 0.9,
        })
        .expect("update score");

    // Build a projection and query the top entry.
    let mut store = update.base.store;
    let events = store.load().expect("load events");
    let projection = MemoryProjection::from_events(10, &events);
    if let Some(entry) = projection.top_entries(1).first() {
        println!("Top entry: {} with score {}", entry.id, entry.score);
    }

    // Clean up the temporary file.
    std::fs::remove_file(path).expect("cleanup");
}
