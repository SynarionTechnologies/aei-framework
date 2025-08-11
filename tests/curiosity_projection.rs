use aei_framework::{
    infrastructure::projection::CuriosityScoreProjection, CuriosityScoreUpdated, Event,
};
use uuid::Uuid;

#[test]
fn curiosity_projection_seeds_and_updates_scores() {
    let id = Uuid::new_v4();
    let events = vec![Event::CuriosityScoreUpdated(CuriosityScoreUpdated {
        target_id: id,
        old_score: 0.0,
        new_score: 0.5,
    })];

    let mut projection = CuriosityScoreProjection::from_events(&events);
    assert_eq!(projection.get(id), Some(0.5));

    let update_event = Event::CuriosityScoreUpdated(CuriosityScoreUpdated {
        target_id: id,
        old_score: 0.5,
        new_score: 0.8,
    });
    projection.apply(&update_event);
    assert_eq!(projection.get(id), Some(0.8));

    let unknown_id = Uuid::new_v4();
    assert_eq!(projection.get(unknown_id), None);
}
