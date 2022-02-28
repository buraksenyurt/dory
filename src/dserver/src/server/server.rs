use crate::model::Message;
use crate::{InformativeEvent, Pack, TransmitterEvent};
use crossbeam::channel;
use crossbeam::channel::{Receiver, Sender};
use log::{error, info, warn};
use std::io::Read;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

/// It holds the basic information for the TCP server.
pub struct Server<'a> {
    root: &'a str,
    port: u16,
}

impl<'a> Server<'a> {
    /// Creates a new TCP Server object.
    pub fn new(root: &'a str, port: u16) -> Self {
        Server { root, port }
    }

    /// Returns the server address:port information.
    fn address(&self) -> String {
        format!("{}:{}", &self.root, &self.port)
    }

    /// It starts the TCP server and listens for incoming requests.
    /// It leaves the necessary message to the channel according to the suitability of the requests.
    /// Using these messages, it adds, reads, and deletes objects in the packets.
    pub fn run(self) {
        let (event_transmitter, event_receiver) = channel::unbounded();
        let (informative_transmitter, informative_receiver) = channel::unbounded();

        let pack = Pack {
            id: 1,
            ..Default::default()
        };
        info!("Pack #{} initialized", &pack.id);
        let pack_ref = Arc::new(Mutex::new(pack));

        let _ = thread::spawn(|| pack_worker(event_receiver, informative_transmitter));
        let _ = thread::spawn(|| {
            for info in informative_receiver {
                info!("\t{:?}", info);
            }
        });
        info!("{} is running", self.address());

        let listener = TcpListener::bind(&self.address());
        match listener {
            Ok(l) => {
                info!("Server started.");
                loop {
                    match l.accept() {
                        Ok((mut stream, _addrees)) => {
                            let mut buffer = [0_u8; 512];
                            match stream.read(&mut buffer) {
                                Ok(l) => {
                                    let msg = String::from_utf8(buffer[0..l].to_vec());
                                    info!("Request, {:?}", msg.unwrap());
                                    let message = Message::try_from(&buffer[0..l]).unwrap();
                                    info!("{:?}", message);
                                    message.send(&pack_ref, &event_transmitter);
                                }
                                Err(e) => {
                                    error!("Read error, {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            error!("Server error -> {}", e)
                        }
                    }
                }
            }
            Err(e) => {
                error!("Sunucu başlatılamadı. Hata detayı -> {}", e);
            }
        }
    }
}

pub fn pack_worker(events: Receiver<TransmitterEvent>, informative: Sender<InformativeEvent>) {
    for event in events {
        match event {
            TransmitterEvent::AddNewItem(c) => {
                c.pack.lock().unwrap().add(c.object.clone());
                info!("Item {} added to pack.", c.object);
                if informative
                    .send(InformativeEvent::Added(c.object.clone().uuid))
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
                            .send(InformativeEvent::Found(Arc::new(o.clone())))
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
