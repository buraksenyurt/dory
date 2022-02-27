use crate::server::server::Server;
use event::{InformativeEvent, TransmitterEvent};
use log::{error, info};
use model::{Candidate, Item, Pack, Search, Value};
use std::env;
use std::process::exit;

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
                "basic" => {
                    basic_mode();
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

fn basic_mode() {
    info!("Basic mode is starting.");
    let alpha = Server::new("0.0.0.0", 5555_u16);
    alpha.run();
    info!("Simulation completed.");
}
