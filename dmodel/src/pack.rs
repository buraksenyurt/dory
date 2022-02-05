use crate::constant::MAX_ITEM;
use crate::item::Item;
use log::warn;
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;

    #[test]
    fn should_we_can_add_items_to_pack() {
        let mut pack = Pack {
            id: 23,
            ..Default::default()
        };
        assert_eq!(pack.get_head(), 0);
        let item = Item::new("server", Value::Text("localhost")).unwrap();
        assert!(!item.uuid.is_nil());
        let state = pack.add(item);
        assert_eq!(pack.get_head(), 1);
        match state {
            Some(PackState::Added(v)) => assert!(!v.is_nil()),
            _ => {}
        }
    }

    #[test]
    fn should_packs_items_are_empty_after_drop() {
        let mut pack = Pack {
            id: 23,
            ..Default::default()
        };
        let item = Item::new("server", Value::Text("localhost")).unwrap();
        pack.add(item);
        let item = Item::new("logs_on", Value::Text("true")).unwrap();
        pack.add(item);
        assert!(pack.get_head() == 2);
        pack.drop();
        assert!(pack.get_head() == 0);
    }

    #[test]
    fn should_capacity_is_full_if_item_add_after_max() {
        let mut pack = Pack {
            id: 23,
            ..Default::default()
        };
        for _ in 0..=999 {
            let item = Item::new("lorem", Value::Text("ipsum")).unwrap();
            pack.add(item);
        }
        assert!(pack.get_head() == 1000);
        let item = Item::new("lorem", Value::Text("ipsum")).unwrap();
        let state = pack.add(item).unwrap();
        assert_eq!(state, PackState::CapacityFull);
    }

    #[test]
    fn should_we_can_find_added_item() {
        let mut pack = Pack {
            id: 23,
            ..Default::default()
        };
        let item = Item::new("server", Value::Text("london")).unwrap();
        pack.add(item);
        let item = Item::new("debug", Value::Text("on")).unwrap();
        pack.add(item);

        let item = pack.get("debug").unwrap();
        assert_eq!(item.value, Value::Text("on"));
    }
}

#[derive(Default)]
pub struct Pack {
    pub id: u32,
    items: Vec<Item>,
    head: u16,
}

#[allow(dead_code)]
impl Pack {
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

    pub fn drop(&mut self) -> &Self {
        warn!("Pack #{} dropped", self.id);
        self.items = Vec::new();
        self.head = 0;
        self
    }

    pub fn get(&self, key: &str) -> Option<&Item> {
        self.items.iter().find(|i| i.key == key)
    }

    pub fn get_head(&self) -> u16 {
        self.head
    }
}

#[derive(Debug, PartialEq)]
pub enum PackState {
    Added(Uuid),
    CapacityFull,
}
