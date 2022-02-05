use crate::constant::{MAX_KEY_LEN, MAX_VALUE_LEN};
use crate::errors::NewItemError;
use crate::value::Value;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_created_item_has_uuid() {
        let sample = Item::new("server", Value::Text("localhost"));
        match sample {
            Ok(s) => {
                assert!(!s.uuid.is_nil());
                assert_eq!(s.to_string(), "{\"server\":\"Text(\"localhost\")\"}");
            }
            _ => {}
        }
    }

    #[test]
    #[should_panic]
    fn should_long_key_name_throw_panic() {
        let _ = Item::new("server name is too long", Value::Text("localhost")).unwrap();
    }

    #[test]
    #[should_panic]
    fn should_long_value_throw_panic() {
        let _ = Item::new(
            "server",
            Value::Text(
                r#"This is the localhost name of the server but
        it is really toooo long name can you understand me body."#,
            ),
        )
        .unwrap();
    }
}

#[derive(Copy, Clone)]
pub struct Item {
    pub key: &'static str,
    pub value: Value,
    pub uuid: Uuid,
}

#[allow(dead_code)]
impl Item {
    pub fn new(key: &'static str, value: Value) -> Result<Self, NewItemError> {
        if key.as_bytes().len() > MAX_KEY_LEN {
            return Err(NewItemError::InvalidKeyLen);
        }

        if let Value::Text(s) = &value {
            if s.as_bytes().len() > MAX_VALUE_LEN {
                return Err(NewItemError::InvalidValueLen);
            }
        }

        let id = Uuid::new_v4();
        Ok(Item {
            key,
            value,
            uuid: id,
        })
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\"{}\":\"{}\"}}", &self.key, &self.value)
    }
}
