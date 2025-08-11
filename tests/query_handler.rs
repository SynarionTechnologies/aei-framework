use aei_framework::{
    application::QueryHandler,
    domain::{Event, RandomNeuronAdded},
    infrastructure::projection::NetworkProjection,
    Activation,
};
use uuid::Uuid;

#[test]
fn convenience_methods_return_expected_values() {
    let neuron_a = Uuid::new_v4();
    let neuron_b = Uuid::new_v4();
    let synapse_id = Uuid::new_v4();
    let events = vec![
        Event::RandomNeuronAdded(RandomNeuronAdded {
            neuron_id: neuron_a,
            activation: Activation::ReLU,
        }),
        Event::RandomNeuronAdded(RandomNeuronAdded {
            neuron_id: neuron_b,
            activation: Activation::Sigmoid,
        }),
        Event::SynapseCreated {
            id: synapse_id,
            from: neuron_a,
            to: neuron_b,
            weight: 0.5,
        },
    ];
    let projection = NetworkProjection::from_events(&events);
    let handler = QueryHandler::new(&projection);

    let neuron = handler.neuron(neuron_a).expect("neuron missing");
    assert_eq!(neuron.activation, Activation::ReLU);
    assert!(handler.neuron(Uuid::new_v4()).is_none());

    let synapse = handler.synapse(synapse_id).expect("synapse missing");
    assert_eq!(synapse.from, neuron_a);
    assert_eq!(synapse.to, neuron_b);
    assert!(handler.synapse(Uuid::new_v4()).is_none());

    assert_eq!(handler.activation(neuron_a), Some(Activation::ReLU));
    assert_eq!(handler.activation(Uuid::new_v4()), None);
}
