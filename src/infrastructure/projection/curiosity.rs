//! Projection storing curiosity scores for quick lookup.

use std::collections::HashMap;

use uuid::Uuid;

use crate::domain::{CuriosityScoreUpdated, Event};

/// Read model mapping identifiers to curiosity scores.
#[derive(Debug, Default)]
pub struct CuriosityScoreProjection {
    scores: HashMap<Uuid, f64>,
}

impl CuriosityScoreProjection {
    /// Builds the projection by replaying events.
    #[must_use]
    pub fn from_events(events: &[Event]) -> Self {
        let mut proj = Self::default();
        for event in events {
            proj.apply(event);
        }
        proj
    }

    /// Applies a new event to update a score.
    pub fn apply(&mut self, event: &Event) {
        if let Event::CuriosityScoreUpdated(CuriosityScoreUpdated {
            target_id,
            new_score,
            ..
        }) = event
        {
            self.scores.insert(*target_id, *new_score);
        }
    }

    /// Returns the curiosity score for the given identifier.
    #[must_use]
    pub fn get(&self, id: Uuid) -> Option<f64> {
        self.scores.get(&id).copied()
    }
}
