use std::str::from_utf8;
use crate::derror::message_parse_error::MessageParseError;
use crate::model::command::Command;

/// Data model representing incoming messages to the TCP line
pub struct Message {
    pub command: Command,
    pub key: String,
    pub value: String,
}

impl TryFrom<&[u8]> for Message {
    type Error = MessageParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let s=from_utf8(value)?;

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
