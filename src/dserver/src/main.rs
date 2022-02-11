use crossbeam::channel;
use crossbeam::channel::{Receiver, Sender};
use dserver::{Candidate, InformativeEvent, Item, TransmitterEvent, Value};
use log::{error, info};
use std::thread;

fn main() {
    env_logger::init();

    //TODO: We need long life Packs for selected mod. How?
    //TODO: Main always on live. How?

    //TODO: This is one pack sample candidate. How can I increase it one more pack but onle one?
    let (event_transmitter, event_receiver) = channel::unbounded();
    let (informative_transmitter, informative_receiver) = channel::unbounded();

    let t1 = thread::spawn(|| pack_worker(event_receiver, informative_transmitter));
    let _ = event_transmitter.send(TransmitterEvent::StartPack);
    let _ = event_transmitter.send(TransmitterEvent::AddNewItem(Candidate {
        pack_id: 1,
        object: Item::new("server", Value::Text("localhost")).unwrap(),
    }));
    drop(event_transmitter);

    for info in informative_receiver{
        info!("{:?}",info);
    }

    let _ = t1.join();
}

fn pack_worker(events: Receiver<TransmitterEvent>, informative: Sender<InformativeEvent>) {
    for event in events {
        match event {
            TransmitterEvent::StartPack => {
                info!("A new pack initialized");
                //TODO: Create Pack and publish event with ID
                if informative.send(InformativeEvent::PackCreated).is_err(){
                    error!("{:?}",InformativeEvent::PackCreatedError);
                    break;
                }
            }
            TransmitterEvent::AddNewItem(c) => {
                //TODO: try to add candidate's object to related pack
                info!("Item {} added to pack {}", c.object, c.pack_id);

                if informative.send(InformativeEvent::ItemAdded(c.object.uuid)).is_err(){
                    error!("{:?}",InformativeEvent::ItemAddError);
                    break;
                }
            }
        }
    }
}
