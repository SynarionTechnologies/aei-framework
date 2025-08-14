use aei_runtime::event_bus::{EventBus, InMemoryEventBus};

#[test]
fn subscriber_receives_published_event() {
    let mut bus: InMemoryEventBus<u32> = InMemoryEventBus::new();
    let rx = bus.subscribe();
    bus.publish(7);
    assert_eq!(rx.recv().unwrap(), 7);
}
