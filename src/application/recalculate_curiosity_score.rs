//! Command and handler to recalculate curiosity scores.

use uuid::Uuid;

use crate::domain::{CuriosityScoreUpdated, Event, Network};
use crate::infrastructure::EventStore;

/// Scope of targets whose curiosity score should be recomputed.
#[derive(Debug, Clone, Copy)]
pub enum CuriosityScope {
    /// Only the provided neuron identifiers.
    Neuron,
    /// Only the provided synapse identifiers.
    Synapse,
    /// All neurons and synapses currently in the network.
    All,
}

/// Command requesting curiosity score recalculation.
#[derive(Debug, Clone)]
pub struct RecalculateCuriosityScoreCommand {
    /// Identifiers of targets to update.
    pub target_ids: Vec<Uuid>,
    /// Scope describing the type of targets.
    pub scope: CuriosityScope,
}

/// Handles [`RecalculateCuriosityScoreCommand`].
pub struct RecalculateCuriosityScoreHandler<S: EventStore> {
    /// Event store used for persistence.
    pub store: S,
    /// Current network state reconstructed from events.
    pub network: Network,
}

impl<S: EventStore> RecalculateCuriosityScoreHandler<S> {
    /// Loads events from the store to initialize the handler.
    pub fn new(mut store: S) -> Result<Self, S::Error> {
        let events = store.load()?;
        let network = Network::hydrate(&events);
        Ok(Self { store, network })
    }

    /// Recomputes curiosity scores for the requested targets.
    pub fn handle(
        &mut self,
        cmd: RecalculateCuriosityScoreCommand,
    ) -> Result<Vec<Event>, S::Error> {
        let events = self.store.load()?; // full history for analysis
        let targets = self.resolve_targets(cmd);
        let mut emitted = Vec::new();
        for id in targets {
            let old = self
                .network
                .neurons
                .get(&id)
                .map(|n| n.curiosity_score)
                .or_else(|| self.network.synapses.get(&id).map(|s| s.curiosity_score))
                .unwrap_or_default();
            let new_score = Self::compute_score(&events, id);
            if (new_score - old).abs() > f64::EPSILON {
                let event = Event::CuriosityScoreUpdated(CuriosityScoreUpdated {
                    target_id: id,
                    old_score: old,
                    new_score,
                });
                self.store.append(&event)?;
                self.network.apply(&event);
                emitted.push(event);
            }
        }
        Ok(emitted)
    }

    fn resolve_targets(&self, cmd: RecalculateCuriosityScoreCommand) -> Vec<Uuid> {
        match cmd.scope {
            CuriosityScope::Neuron => cmd.target_ids,
            CuriosityScope::Synapse => cmd.target_ids,
            CuriosityScope::All => self
                .network
                .neurons
                .keys()
                .chain(self.network.synapses.keys())
                .copied()
                .collect(),
        }
    }

    /// Computes a simple curiosity score based on event rarity.
    fn compute_score(events: &[Event], id: Uuid) -> f64 {
        let occurrences = events.iter().filter(|e| Self::touches(e, id)).count();
        1.0 / (1.0 + occurrences as f64)
    }

    fn touches(event: &Event, id: Uuid) -> bool {
        match event {
            Event::RandomNeuronAdded(e) => e.neuron_id == id,
            Event::RandomNeuronRemoved(e) => e.neuron_id == id,
            Event::NeuronAdded(e) => e.neuron_id == id,
            Event::NeuronRemoved(e) => e.neuron_id == id,
            Event::SynapseCreated {
                id: sid, from, to, ..
            } => *sid == id || *from == id || *to == id,
            Event::SynapseRemoved { id: sid } => *sid == id,
            Event::RandomSynapseAdded(e) => e.synapse_id == id || e.from == id || e.to == id,
            Event::RandomSynapseRemoved(e) => e.synapse_id == id,
            Event::SynapseWeightMutated(e) => e.synapse_id == id,
            Event::NeuronActivationMutated(e) => e.neuron_id == id,
            Event::CuriosityScoreUpdated(e) => e.target_id == id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{RandomNeuronAdded, RandomNeuronRemoved};

    #[test]
    fn compute_score_decreases_with_occurrences() {
        let id = Uuid::new_v4();
        let events = vec![
            Event::RandomNeuronAdded(RandomNeuronAdded {
                neuron_id: id,
                activation: crate::domain::Activation::Identity,
            }),
            Event::RandomNeuronRemoved(RandomNeuronRemoved { neuron_id: id }),
        ];
        let score =
            RecalculateCuriosityScoreHandler::<crate::FileEventStore>::compute_score(&events, id);
        assert!(score < 1.0);
    }
}
