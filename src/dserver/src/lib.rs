use log::warn;
use std::fmt::{Display, Formatter};
use thiserror::Error;
use uuid::Uuid;

pub const MAX_ITEM: u16 = 1000;
pub const MAX_KEY_LEN: usize = 16;
pub const MAX_VALUE_LEN: usize = 64;

#[derive(Debug, Error)]
pub enum NewItemError {
    #[error("Key name is too long.")]
    InvalidKeyLen,
    #[error("Value is too long.")]
    InvalidValueLen,
}

/// It is the key:value object that holds primitive data types by marking them with the unique key.
#[derive(Copy, Clone)]
pub struct Item {
    pub key: &'static str,
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

/// Indicates the type of value in the Item object.
///
/// It is designed to work with light weight and low cost values.
/// For example, with ThinNumber, it is specified to keep an 8-bit integer.
/// Logical kept boolean values.
/// Text type that can carry large data is subject to length validation in the Item::new function.
///
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum Value {
    ThinNumber(i8),
    MidNumber(i16),
    LargeNumber(i32),
    ThinFloat(f32),
    LargeFloat(f64),
    Text(&'static str),
    Logical(bool),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// The package object that holds the Item collection.
///
/// # Examples
///
/// ```
/// // Create a new empty pack
/// use dserver::{Pack,Item,Value};
///
/// let mut pack = Pack {
///     id: 23,
///     ..Default::default()
///  };
///
/// // Add some item into this pack
/// let item = Item::new("server", Value::Text("london")).unwrap();
/// pack.add(item);
///
/// assert!(pack.get_head() == 1);
/// ```
#[derive(Default)]
pub struct Pack {
    pub id: u32,
    pub items: Vec<Item>,
    pub head: u16,
}

#[allow(dead_code)]
impl Pack {
    /// Adds a new item to the pack
    ///
    /// # Warning
    ///
    /// PackState::CapacityFull is returned if the package content has reached the maximum number of elements.
    /// Otherwise, the item is added to the package.
    pub fn add(&mut self, item: Item) -> Option<PackState> {
        self.head += 1;
        match &self.head {
            0..=MAX_ITEM => {
                self.items.push(item);
                Some(PackState::Added(item.uuid))
            }
            _ => {
                warn!("Capacity is full for Pack #{}", self.id);
                Some(PackState::CapacityFull)
            }
        }
    }

    /// Empties the pack contents and returns the head to the initial position.
    pub fn drop(&mut self) -> &Self {
        warn!("Pack #{} dropped", self.id);
        self.items = Vec::new();
        self.head = 0;
        self
    }

    /// Retrieves the value of a key from within the pack.
    pub fn get(&self, key: &str) -> Option<&Item> {
        self.items.iter().find(|i| i.key == key)
    }

    /// Returns the current position of the head.
    pub fn get_head(&self) -> u16 {
        self.head
    }
}

/// It gives information about the pack status.
#[derive(Debug, PartialEq)]
pub enum PackState {
    Added(Uuid),
    CapacityFull,
}
