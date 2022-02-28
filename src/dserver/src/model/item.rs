use super::Value;
use crate::constant::constant::{MAX_KEY_LEN, MAX_VALUE_LEN};
use crate::derror::new_item_error::NewItemError;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

/// It is the key:value object that holds primitive data types by marking them with the unique key.
#[derive(Debug, Clone)]
pub struct Item {
    pub key: String,
    pub value: Value,
    pub uuid: Uuid,
}

#[allow(dead_code)]
impl Item {
    //! [`Item::new`] function
    //!
    /// Generates a new key:value item
    ///
    /// # Panics
    ///
    /// If the key or value is greater than the allowed length, panic occurs.
    ///
    /// # Examples
    ///
    /// ```
    ///  use dserver::{Item,Value};
    ///
    ///  let sample = Item::new("server", Value::Text("localhost"));
    ///  match sample {
    ///   Ok(s) => {
    ///      assert!(!s.uuid.is_nil());
    ///      assert_eq!(s.to_string(), "{\"server\":\"Text(\"localhost\")\"}");
    ///     }
    ///     _ => {}
    ///   }
    ///
    /// ```
    pub fn new(key: String, value: Value) -> Result<Self, NewItemError> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_created_item_has_uuid() {
        let sample = Item::new("server".to_string(), Value::Text("localhost"));
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
        let _ = Item::new("server name is too long".to_string(), Value::Text("localhost")).unwrap();
    }

    #[test]
    #[should_panic]
    fn should_long_value_throw_panic() {
        let _ = Item::new(
            "server".to_string(),
            Value::Text(
                r#"This is the localhost name of the server but
        it is really toooo long name can you understand me body."#,
            ),
        )
        .unwrap();
    }

    #[test]
    fn should_primitive_values_works() {
        let logson = Item::new("logs_on".to_string(), Value::Logical(true)).unwrap();
        assert_eq!(logson.value, Value::Logical(true));

        let max_player = Item::new("maxplayer".to_string(), Value::ThinNumber(8)).unwrap();
        assert_eq!(max_player.value, Value::ThinNumber(8));

        let default_value = Item::new("defaultvalue".to_string(), Value::ThinFloat(3.22)).unwrap();
        assert_eq!(default_value.value, Value::ThinFloat(3.22));

        let edge_of_tomorrow =
            Item::new("pi".to_string(), Value::LargeFloat(24.342343243423423423431415)).unwrap();
        assert_eq!(
            edge_of_tomorrow.value,
            Value::LargeFloat(24.342343243423423423431415)
        );
    }
}
