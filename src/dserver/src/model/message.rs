use crate::constant::constant::MAX_KEY_LEN;
use crate::derror::message_parse_error::MessageParseError;
use crate::derror::message_send_error::MessageSendError;
use crate::model::command::Command;
use crate::model::Search;
use crate::{Candidate, Item, Pack, TransmitterEvent, Value};
use crossbeam::channel::Sender;
use log::info;
use std::str::from_utf8;
use std::sync::{Arc, Mutex};

/// Data model representing incoming messages to the TCP line
#[derive(Debug, PartialEq, Clone)]
pub struct Message {
    pub command: Command,
    pub key: String,
    pub value: Option<Value>,
}

impl Message {
    pub fn new(command: Command, key: String, value: Option<Value>) -> Self {
        Self {
            command,
            key,
            value,
        }
    }

    pub fn send(
        self,
        pack: &Arc<Mutex<Pack>>,
        event: &Sender<TransmitterEvent>,
    ) -> Result<u8, MessageSendError> {
        match self.command {
            Command::Add => {
                info!("{:?}", self);
                let r = event.send(TransmitterEvent::AddNewItem(Candidate {
                    pack: pack.clone(),
                    object: Item::new(self.key, self.value.unwrap()).unwrap(),
                }));
                match r {
                    Ok(_) => Ok(1),
                    Err(_) => Err(MessageSendError::Add),
                }
            }
            Command::Get => {
                let r = event.send(TransmitterEvent::GetItem(Search {
                    pack: pack.clone(),
                    key: self.key,
                }));
                match r {
                    Ok(_) => Ok(1),
                    Err(_) => Err(MessageSendError::Get),
                }
            }
            Command::Del => {
                //TODO Not completed
                info!("Del request");
                Ok(1)
            }
        }
    }
}

fn get_part(text: &str) -> Option<(&str, &str)> {
    for (i, c) in text.chars().enumerate() {
        if c == '|' {
            return Some((&text[..i], &text[i + 1..]));
        }
    }
    None
}

impl<'a> TryFrom<&'a [u8]> for Message {
    type Error = MessageParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(MessageParseError::Empty);
        }
        //TODO I have to check incoming message pattern. If not available raise error.



        let s = from_utf8(value)?;
        let (command, s) = get_part(s).unwrap();
        let (key, s) = get_part(s).unwrap();

        if key.chars().count() > MAX_KEY_LEN {
            return Err(MessageParseError::KeyNameTooLong);
        }

        match command {
            "ADD" => {
                //TODO: Bug. If get_part fails then this match raise panic.
                let (data_type, s) = get_part(s).unwrap();
                let (v, _) = get_part(s).unwrap();
                let object_value = match data_type {
                    "s" => Value::Text("TEST"), //TODO: Lifetime Error occurred for v. I have to find solution.
                    "i8" => Value::ThinNumber(v.parse::<i8>().unwrap()),
                    "i16" => Value::MidNumber(v.parse::<i16>().unwrap()),
                    "i32" => Value::LargeNumber(v.parse::<i32>().unwrap()),
                    "f32" => Value::ThinFloat(v.parse::<f32>().unwrap()),
                    "f64" => Value::LargeFloat(v.parse::<f64>().unwrap()),
                    "l" => Value::Logical(v.parse::<bool>().unwrap()),
                    _ => Value::Empty,
                };
                Ok(Message::new(
                    Command::Add,
                    key.to_string(),
                    Some(object_value),
                ))
            }
            "DEL" => Ok(Message::new(Command::Del, key.to_string(), None)),
            "GET" => Ok(Message::new(Command::Get, key.to_string(), None)),
            _ => Err(MessageParseError::Command),
        }
    }
}

#[cfg(test)]
mod test {
    /*
        ADD|ServerName|STRING|LOCALHOST|
        GET|ServerName|
        DEL|ConnectionString|
        ADD|Logs|BOOLEAN|0|
        ADD|DefaultPi|U32|3.1415|
    */

    use crate::derror::message_parse_error::MessageParseError;
    use crate::model::command::Command;
    use crate::model::message::Message;
    use crate::Value;

    #[test]
    fn should_add_messages_could_be_parse() {
        let message = "ADD|ServerName|s|localhost|";
        let bytes = message.as_bytes();
        let result = Message::try_from(bytes).unwrap();
        assert_eq!(result.command, Command::Add);
        assert_eq!(result.key, "ServerName".to_string());
        assert_eq!(result.value, Some(Value::Text("")));

        let message = "ADD|Logs|l|true|";
        let bytes = message.as_bytes();
        let result = Message::try_from(bytes).unwrap();
        assert_eq!(result.command, Command::Add);
        assert_eq!(result.key, "Logs".to_string());
        assert_eq!(result.value, Some(Value::Logical(true)));

        let message = "ADD|DefaultPi|f32|3.1415|";
        let bytes = message.as_bytes();
        let result = Message::try_from(bytes).unwrap();
        assert_eq!(result.command, Command::Add);
        assert_eq!(result.key, "DefaultPi".to_string());
        assert_eq!(result.value, Some(Value::ThinFloat(3.1415)));
    }

    #[test]
    fn should_get_message_could_be_parse() {
        let message = "GET|ServerName|";
        let bytes = message.as_bytes();
        let result = Message::try_from(bytes).unwrap();
        assert_eq!(result.command, Command::Get);
        assert_eq!(result.key, "ServerName".to_string());

        let message = "DEL|ConnectionString|";
        let bytes = message.as_bytes();
        let result = Message::try_from(bytes).unwrap();
        assert_eq!(result.command, Command::Del);
        assert_eq!(result.key, "ConnectionString".to_string());
    }

    #[test]
    fn should_long_key_name_raise_an_error() {
        let message = "GET|This is your server name|";
        let bytes = message.as_bytes();
        let result = Message::try_from(bytes);
        assert_eq!(result, Err(MessageParseError::KeyNameTooLong));
    }

    #[test]
    fn should_empty_message_raise_an_message_empty_error() {
        let message = "";
        assert_eq!(message.len(), 0);
        let bytes = message.as_bytes();
        let result = Message::try_from(bytes);
        assert_eq!(result, Err(MessageParseError::Empty));
    }
}
