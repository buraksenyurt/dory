use crate::server::server::Server;
use crossbeam::channel;
use crossbeam::channel::{Receiver, Sender};
use event::{InformativeEvent, TransmitterEvent};
use log::{error, info, warn};
use model::{Candidate, Item, Pack, Search, Value};
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::{env, thread};

mod constant;
mod derror;
mod event;
mod model;
mod server;

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let command = &args[1];
            match command.to_lowercase().as_str() {
                "simple" => {
                    info!("Single mode is starting.");
                    simple_mode();
                    info!("Simulation completed.");
                }
                _ => {
                    error!("Understandable command.");
                    exit(1);
                }
            }
        }
        _ => {
            error!("No one else is here. Argument error.");
            exit(1);
        }
    }
}

fn simple_mode() {
    let (event_transmitter, event_receiver) = channel::unbounded();
    let (informative_transmitter, informative_receiver) = channel::unbounded();
    let web_event_receiver=event_receiver.clone();
    let web_informative_transmitter=informative_transmitter.clone();

    let pack = Pack {
        id: 1,
        ..Default::default()
    };
    info!("Pack #{} initialized", &pack.id);
    let pack_ref = Arc::new(Mutex::new(pack));

    let t1 = thread::spawn(|| pack_worker(event_receiver, informative_transmitter));
    let t2=thread::spawn(||{
        //TODO Read ip and port from environment variables
        let alpha = Server::new("0.0.0.0", 5555_u16, "localhost");
        alpha.run(web_event_receiver,web_informative_transmitter);
    });

    // let _ = event_transmitter.send(TransmitterEvent::AddNewItem(Candidate {
    //     pack: pack_ref.clone(),
    //     object: Item::new("server", Value::Text("localhost")).unwrap(),
    // }));
    //
    // let _ = event_transmitter.send(TransmitterEvent::AddNewItem(Candidate {
    //     pack: pack_ref.clone(),
    //     object: Item::new("level", Value::ThinNumber(50)).unwrap(),
    // }));
    //
    // let _ = event_transmitter.send(TransmitterEvent::GetItem(Search {
    //     key: "server",
    //     pack: pack_ref.clone(),
    // }));
    //
    // let _ = event_transmitter.send(TransmitterEvent::GetItem(Search {
    //     key: "mail_server",
    //     pack: pack_ref,
    // }));

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
                    .send(InformativeEvent::Added(c.object.uuid))
                    .is_err()
                {
                    error!("{:?}", InformativeEvent::AddError);
                    break;
                }
            }
            TransmitterEvent::GetItem(s) => {
                let pack = s.pack.lock().unwrap();
                info!("{:?}", pack);
                let item = pack.get(s.key);
                info!("{:?}", item);
                match item {
                    Some(o) => {
                        info!("{} founded.", o.to_string());
                        if informative
                            .send(InformativeEvent::Found(Arc::new(*o)))
                            .is_err()
                        {
                            error!("{:?}", InformativeEvent::GetError);
                            break;
                        }
                    }
                    None => {
                        warn!("{:?}", InformativeEvent::NotFound);
                    }
                }
            }
        }
    }
}
