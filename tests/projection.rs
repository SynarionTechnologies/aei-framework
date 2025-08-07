use aei_framework::{
    application::{Query, QueryHandler, QueryResult},
    domain::{Event, RandomNeuronAdded},
    infrastructure::projection::NetworkProjection,
    Activation,
};
use uuid::Uuid;

#[test]
fn projection_replays_events_for_queries() {
    let neuron_id = Uuid::new_v4();
    let events = vec![Event::RandomNeuronAdded(RandomNeuronAdded {
        neuron_id,
        activation: Activation::ReLU,
    })];
    let projection = NetworkProjection::from_events(&events);
    let handler = QueryHandler::new(&projection);
    match handler.handle(Query::GetNeuron { id: neuron_id }) {
        QueryResult::Neuron(Some(neuron)) => match neuron.activation {
            Activation::ReLU => {}
            _ => panic!("unexpected activation"),
        },
        _ => panic!("neuron not found"),
    }
}
