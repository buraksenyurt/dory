use super::super::model::{Candidate, Search};

/// It is the enum that contains the event definitions to be used in the transmitter channel.
#[derive(Debug)]
pub enum TransmitterEvent {
    AddNewItem(Candidate),
    GetItem(Search),
}
