use aei_framework::{
    infrastructure::{projection::NetworkProjection, FileEventStore},
    AddRandomNeuronCommand, AddRandomNeuronHandler, AddRandomSynapseCommand,
    AddRandomSynapseHandler, Query, QueryHandler, QueryResult,
};
use rand::thread_rng;

fn main() {
    env_logger::init();
    let path = std::env::temp_dir().join("aei_example.log");
    let store = FileEventStore::new(path);

    // Add two neurons to the network.
    let mut add_neuron = AddRandomNeuronHandler::new(store, thread_rng()).expect("store");
    let n1 = add_neuron
        .handle(AddRandomNeuronCommand)
        .expect("first neuron");
    let n2 = add_neuron
        .handle(AddRandomNeuronCommand)
        .expect("second neuron");

    // Reuse the same event store to add a synapse between them.
    let store = add_neuron.base.store;
    let mut add_synapse = AddRandomSynapseHandler::new(store, thread_rng()).expect("store");
    let syn = add_synapse
        .handle(AddRandomSynapseCommand)
        .expect("synapse");
    println!("Added neurons {n1} and {n2} with synapse {syn}");

    // Build a projection and query neuron information.
    let mut store = add_synapse.base.store;
    let events = store.load().expect("load events");
    let projection = NetworkProjection::from_events(&events);
    let handler = QueryHandler::new(&projection);
    if let QueryResult::Neuron(Some(neuron)) = handler.handle(Query::GetNeuron { id: n1 }) {
        println!("Neuron {n1} activation: {:?}", neuron.activation);
    }
}
