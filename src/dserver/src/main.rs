use crossbeam::channel;
use crossbeam::channel::{Receiver, Sender};
use dserver::{Candidate, InformativeEvent, Item, Pack, TransmitterEvent, Value};
use log::{error, info};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    env_logger::init();

    let (event_transmitter, event_receiver) = channel::unbounded();
    let (informative_transmitter, informative_receiver) = channel::unbounded();

    let pack = Pack {
        id: 1,
        ..Default::default()
    };
    info!("Pack #{} initialized", &pack.id);
    let pack_ref = Arc::new(Mutex::new(pack));

    let t1 = thread::spawn(|| pack_worker(event_receiver, informative_transmitter));

    let _ = event_transmitter.send(TransmitterEvent::AddNewItem(Candidate {
        pack: pack_ref.clone(),
        object: Item::new("server", Value::Text("localhost")).unwrap(),
    }));

    let _ = event_transmitter.send(TransmitterEvent::AddNewItem(Candidate {
        pack: pack_ref,
        object: Item::new("level", Value::ThinNumber(50)).unwrap(),
    }));

    drop(event_transmitter);

    for info in informative_receiver {
        info!("{:?}", info);
    }

    let _ = t1.join();
}

fn pack_worker(events: Receiver<TransmitterEvent>, informative: Sender<InformativeEvent>) {
    for event in events {
        match event {
            TransmitterEvent::AddNewItem(c) => {
                c.pack.lock().unwrap().add(c.object);
                info!("Item {} added to pack.", c.object);
                if informative
                    .send(InformativeEvent::ItemAdded(c.object.uuid))
                    .is_err()
                {
                    error!("{:?}", InformativeEvent::ItemAddError);
                    break;
                }
            }
        }
    }
}
