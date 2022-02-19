use std::str::FromStr;
use crate::model::command::Command;

/// Data model representing incoming messages to the TCP line
pub struct Message {
    pub command: Command,
    pub key: String,
    pub value: String,
}

impl FromStr for Message{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        /*
            ADD[ServerName][STRING][LOCALHOST]
            GET[ServerName]
            DEL[ServerName]
            ADD[Logs][BOOLEAN][0]
            ADD[DefaultPi][U32][3.1415]
         */
        //TODO: Codes will be written to translate the textual content in the network packet into the Message data model.
        todo!()
    }
}