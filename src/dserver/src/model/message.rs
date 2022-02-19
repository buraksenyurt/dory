use crate::derror::message_parse_error::MessageParseError;
use crate::model::command::Command;
use std::str::from_utf8;

/// Data model representing incoming messages to the TCP line
pub struct Message {
    pub command: Command,
    pub key: String,
    pub value: Option<String>,
}

impl Message {
    pub fn new(command: Command, key: String, value: Option<String>) -> Self {
        Self {
            command,
            key,
            value,
        }
    }
}

fn get_part(text: &str) -> Option<(&str, &str)> {
    for (i, c) in text.chars().enumerate() {
        if c == '[' || c == ']' {
            return Some((&text[..i], &text[i + 1..]));
        }
    }
    None
}

impl TryFrom<&[u8]> for Message {
    type Error = MessageParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let s = from_utf8(value)?;
        let (command, s) = get_part(s).unwrap();
        let (key, s) = get_part(s).unwrap();

        match command {
            "ADD" => {
                //TODO: I have to write a converter for parsing from value to Value Struct
                Ok(Message::new(Command::Add, key.to_string(), None))
            }
            "DEL" => Ok(Message::new(Command::Del, key.to_string(), None)),
            "GET" => Ok(Message::new(Command::Get, key.to_string(), None)),
            _ => return Err(MessageParseError::Command),
        }

        /*
           ADD[ServerName][STRING][LOCALHOST]
           GET[ServerName]
           DEL[ServerName]
           ADD[Logs][BOOLEAN][0]
           ADD[DefaultPi][U32][3.1415]
        */
    }
}

#[cfg(test)]
mod test {
    use crate::model::command::Command;
    use crate::model::message::Message;

    #[test]
    fn should_add_messages_could_be_parse() {
        let message = "ADD[ServerName][STRING][LOCALHOST]";
        let bytes = message.as_bytes();
        let result = Message::try_from(bytes).unwrap();
        assert_eq!(result.command, Command::Add);
        assert_eq!(result.key, "ServerName".to_string());
    }
}
