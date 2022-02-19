use crate::constant::constant::MAX_KEY_LEN;
use crate::derror::message_parse_error::MessageParseError;
use crate::model::command::Command;
use std::str::from_utf8;

/// Data model representing incoming messages to the TCP line
#[derive(Debug, PartialEq)]
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
        if c == '|' {
            return Some((&text[..i], &text[i + 1..]));
        }
    }
    None
}

impl TryFrom<&[u8]> for Message {
    type Error = MessageParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            return Err(MessageParseError::Empty);
        }
        let s = from_utf8(value)?;
        let (command, s) = get_part(s).unwrap();
        let (key, s) = get_part(s).unwrap();

        if key.chars().count() > MAX_KEY_LEN {
            return Err(MessageParseError::KeyNameTooLong);
        }

        match command {
            "ADD" => {
                //TODO: I have to write a converter for parsing from value to Value Struct
                let (_data_type, s) = get_part(s).unwrap();
                let (v, _) = get_part(s).unwrap();

                Ok(Message::new(
                    Command::Add,
                    key.to_string(),
                    Some(v.to_string()),
                ))
            }
            "DEL" => Ok(Message::new(Command::Del, key.to_string(), None)),
            "GET" => Ok(Message::new(Command::Get, key.to_string(), None)),
            _ => return Err(MessageParseError::Command),
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

    #[test]
    fn should_add_messages_could_be_parse() {
        let message = "ADD|ServerName|STRING|localhost|";
        let bytes = message.as_bytes();
        let result = Message::try_from(bytes).unwrap();
        assert_eq!(result.command, Command::Add);
        assert_eq!(result.key, "ServerName".to_string());
        assert_eq!(result.value, Some("localhost".to_string()));

        let message = "ADD|Logs|BOOLEAN|0|";
        let bytes = message.as_bytes();
        let result = Message::try_from(bytes).unwrap();
        assert_eq!(result.command, Command::Add);
        assert_eq!(result.key, "Logs".to_string());
        assert_eq!(result.value, Some("0".to_string()));

        let message = "ADD|DefaultPi|U32|3.1415|";
        let bytes = message.as_bytes();
        let result = Message::try_from(bytes).unwrap();
        assert_eq!(result.command, Command::Add);
        assert_eq!(result.key, "DefaultPi".to_string());
        assert_eq!(result.value, Some("3.1415".to_string()));
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
