use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_item_test() {
        let sample = Item::new("server", "localhost");
        let uuid = sample.uuid;
        assert!(!uuid.is_nil());
        assert_eq!(sample.to_string(), "{\"server\":\"localhost\"}");
    }
}

pub struct Item {
    pub key: &'static str,
    pub value: &'static str,
    pub uuid: Uuid,
}

impl Item {
    pub fn new(key: &'static str, value: &'static str) -> Self {
        let id = Uuid::new_v4();
        Item {
            key,
            value,
            uuid: id,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\"{}\":\"{}\"}}", &self.key, &self.value)
    }
}
