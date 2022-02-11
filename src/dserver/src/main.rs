use crossbeam::channel;
use crossbeam::channel::{Receiver, Sender};
use dserver::{Candidate, InformativeEvent, Item, TransmitterEvent, Value};
use log::info;
use std::thread;

fn main() {
    env_logger::init();

    let (event_transmitter, event_receiver) = channel::unbounded();
    let (informative_transmitter, _information_receiver) = channel::unbounded();

    let t1 = thread::spawn(|| pack_worker(event_receiver, informative_transmitter));
    let _ = event_transmitter.send(TransmitterEvent::StartPack);
    let _ = event_transmitter.send(TransmitterEvent::AddNewItem(Candidate {
        pack_id: 1,
        object: Item::new("server", Value::Text("localhost")).unwrap(),
    }));
    drop(event_transmitter);

    let _ = t1.join();
}

fn pack_worker(events: Receiver<TransmitterEvent>, _informative: Sender<InformativeEvent>) {
    for event in events {
        match event {
            TransmitterEvent::StartPack => {
                info!("A new pack initialized");
            }
            TransmitterEvent::AddNewItem(c) => {
                info!("Item {} added to pack {}", c.object, c.pack_id);
            }
        }
    }
}
