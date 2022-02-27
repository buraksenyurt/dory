use crate::{InformativeEvent, TransmitterEvent};
use crossbeam::channel::{Receiver, Sender};
use log::{error, info};
use std::io::Read;
use std::net::TcpListener;

pub struct Server<'a> {
    root: &'a str,
    port: u16,
    alias: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(root: &'a str, port: u16, alias: &'a str) -> Self {
        Server { root, port, alias }
    }

    fn address(&self) -> String {
        format!("{}:{}", &self.root, &self.port)
    }

    pub fn run(self, events: Receiver<TransmitterEvent>, informative: Sender<InformativeEvent>) {
        info!("{} is running", self.address());
        let listener = TcpListener::bind(&self.address());
        match listener {
            Ok(l) => {
                info!("Server started.");
                loop {
                    match l.accept() {
                        Ok((mut stream, addrees)) => {
                            let mut buffer = [0_u8; 1024];
                            match stream.read(&mut buffer) {
                                Ok(l) => {
                                    let msg = String::from_utf8(buffer[0..l].to_vec());
                                    info!("Request, {:?}", msg.unwrap());

                                    // let _ = event_transmitter.send(TransmitterEvent::AddNewItem(Candidate {
                                    //     pack: pack_ref.clone(),
                                    //     object: Item::new("server", Value::Text("localhost")).unwrap(),
                                    // }));
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
