use aei_framework::application::memory::{
    AddMemoryEntryCommand, AddMemoryEntryHandler, MemoryQuery, MemoryQueryHandler,
    RemoveMemoryEntryCommand, RemoveMemoryEntryHandler, UpdateMemoryScoreCommand,
    UpdateMemoryScoreHandler,
};
use aei_framework::domain::{AdaptiveMemory, MemoryEntry, MemoryEntryAdded, MemoryEvent};
use aei_framework::infrastructure::projection::MemoryProjection;
use aei_framework::infrastructure::MemoryEventStore;
use serde_json::json;

#[derive(Default, Clone)]
struct InMemoryStore {
    events: Vec<MemoryEvent>,
}

impl MemoryEventStore for InMemoryStore {
    type Error = ();

    fn append(&mut self, event: &MemoryEvent) -> Result<(), Self::Error> {
        self.events.push(event.clone());
        Ok(())
    }

    fn load(&mut self) -> Result<Vec<MemoryEvent>, Self::Error> {
        Ok(self.events.clone())
    }
}

#[test]
fn add_and_query_memory_entry() {
    let store = InMemoryStore::default();
    let mut handler = AddMemoryEntryHandler::new(store, 10).unwrap();
    let id = handler
        .handle(AddMemoryEntryCommand {
            event_type: "test".into(),
            payload: json!({"value": 1}),
            score: 0.8,
        })
        .unwrap();
    assert!(handler.memory.entries.iter().any(|e| e.id == id));
    let projection = MemoryProjection::from_events(10, &handler.store.events);
    let qh = MemoryQueryHandler::new(&projection);
    match qh.handle(MemoryQuery::GetEntryById { id }) {
        aei_framework::application::memory::MemoryQueryResult::Entry(Some(e)) => {
            assert_eq!(e.score, 0.8)
        }
        _ => panic!("entry not found"),
    }
}

#[test]
fn remove_memory_entry() {
    let store = InMemoryStore::default();
    let mut add = AddMemoryEntryHandler::new(store, 10).unwrap();
    let id = add
        .handle(AddMemoryEntryCommand {
            event_type: "test".into(),
            payload: json!({}),
            score: 0.2,
        })
        .unwrap();
    let store = add.store;
    let mut remove = RemoveMemoryEntryHandler::new(store, 10).unwrap();
    remove
        .handle(RemoveMemoryEntryCommand { entry_id: id })
        .unwrap();
    assert!(remove.memory.entries.is_empty());
}

#[test]
fn update_memory_score() {
    let store = InMemoryStore::default();
    let mut add = AddMemoryEntryHandler::new(store, 10).unwrap();
    let id = add
        .handle(AddMemoryEntryCommand {
            event_type: "test".into(),
            payload: json!({}),
            score: 0.2,
        })
        .unwrap();
    let store = add.store;
    let mut update = UpdateMemoryScoreHandler::new(store, 10).unwrap();
    update
        .handle(UpdateMemoryScoreCommand {
            entry_id: id,
            new_score: 0.9,
        })
        .unwrap();
    assert_eq!(
        update
            .memory
            .entries
            .iter()
            .find(|e| e.id == id)
            .unwrap()
            .score,
        0.9
    );
}

#[test]
fn prune_on_capacity_exceeded() {
    let store = InMemoryStore::default();
    let mut handler = AddMemoryEntryHandler::new(store, 1).unwrap();
    let _id1 = handler
        .handle(AddMemoryEntryCommand {
            event_type: "a".into(),
            payload: json!({}),
            score: 0.1,
        })
        .unwrap();
    let id2 = handler
        .handle(AddMemoryEntryCommand {
            event_type: "b".into(),
            payload: json!({}),
            score: 0.9,
        })
        .unwrap();
    assert_eq!(handler.memory.entries.len(), 1);
    assert_eq!(handler.memory.entries[0].id, id2);
}

#[test]
fn replay_from_event_store() {
    let store = InMemoryStore::default();
    let mut handler = AddMemoryEntryHandler::new(store, 10).unwrap();
    let id = handler
        .handle(AddMemoryEntryCommand {
            event_type: "test".into(),
            payload: json!({}),
            score: 0.5,
        })
        .unwrap();
    let events = handler.store.events.clone();
    let memory = AdaptiveMemory::hydrate(10, &events);
    assert!(memory.entries.iter().any(|e| e.id == id));
}

#[test]
fn top_entries_returns_highest_scores_in_descending_order() {
    use chrono::Utc;
    use uuid::Uuid;

    let events = vec![
        MemoryEvent::MemoryEntryAdded(MemoryEntryAdded {
            entry: MemoryEntry {
                id: Uuid::new_v4(),
                timestamp: Utc::now(),
                event_type: "a".into(),
                payload: json!({}),
                score: 0.1,
            },
        }),
        MemoryEvent::MemoryEntryAdded(MemoryEntryAdded {
            entry: MemoryEntry {
                id: Uuid::new_v4(),
                timestamp: Utc::now(),
                event_type: "b".into(),
                payload: json!({}),
                score: 0.9,
            },
        }),
        MemoryEvent::MemoryEntryAdded(MemoryEntryAdded {
            entry: MemoryEntry {
                id: Uuid::new_v4(),
                timestamp: Utc::now(),
                event_type: "c".into(),
                payload: json!({}),
                score: 0.5,
            },
        }),
    ];

    let projection = MemoryProjection::from_events(10, &events);
    let top = projection.top_entries(2);
    assert_eq!(top.len(), 2);
    assert!(top[0].score > top[1].score);
}
