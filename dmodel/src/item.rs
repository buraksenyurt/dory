use std::fmt::{Display, Formatter};
use thiserror::Error;
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_created_item_has_uuid() {
        let sample = Item::new("server", "localhost");
        match sample {
            Ok(s) => {
                assert!(!s.uuid.is_nil());
                assert_eq!(s.to_string(), "{\"server\":\"localhost\"}");
            }
            _ => {}
        }
    }

    #[test]
    #[should_panic]
    fn should_long_key_name_throw_panic() {
        let _ = Item::new("server name is too long", "localhost").unwrap();
    }

    #[test]
    #[should_panic]
    fn should_long_value_throw_panic() {
        let _ = Item::new(
            "server",
            r#"This is the localhost name of the server but
        it is really toooo long name can you understand me body
        . Huaaaa!!! I think it's not too long isn't it?"#,
        )
        .unwrap();
    }
}

const MAX_KEY_LEN: usize = 16;
const MAX_VALUE_LEN: usize = 128;

#[derive(Copy, Clone)]
pub struct Item {
    pub key: &'static str,
    pub value: &'static str,
    pub uuid: Uuid,
}

#[allow(dead_code)]
impl Item {
    pub fn new(key: &'static str, value: &'static str) -> Result<Self, NewItemError> {
        if key.as_bytes().len() > MAX_KEY_LEN {
            return Err(NewItemError::InvalidKeyLen);
        }
        if value.as_bytes().len() > MAX_VALUE_LEN {
            return Err(NewItemError::InvalidValueLen);
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

#[derive(Debug, Error)]
pub enum NewItemError {
    #[error("Key name is too long.")]
    InvalidKeyLen,
    #[error("Value is too long.")]
    InvalidValueLen,
}
