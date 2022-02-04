use crate::item::Item;
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_pack_test() {
        let mut pack = Pack {
            id: 23,
            ..Default::default()
        };
        assert_eq!(pack.head, 0);
        let item = Item::new("server", "localhost");
        assert!(!item.uuid.is_nil());
        let state = pack.add(item);
        assert_eq!(pack.head, 1);
        match state {
            Some(PackState::Added(v)) => assert!(!v.is_nil()),
            _ => {}
        }
    }

    #[test]
    fn drop_pack_test() {
        let mut pack = Pack {
            id: 23,
            ..Default::default()
        };
        let item = Item::new("server", "localhost");
        pack.add(item);
        let item = Item::new("logs_on", "true");
        pack.add(item);
        assert!(pack.head == 2);
        pack.drop();
        assert!(pack.head == 0);
    }
}

const MAX_ITEM: u16 = 1000;

#[derive(Default)]
pub struct Pack {
    pub id: u32,
    pub items: Vec<Item>,
    pub head: u16,
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
            _ => Some(PackState::CapacityFull),
        }
    }

    pub fn drop(&mut self) -> &Self {
        self.items = Vec::new();
        self.head = 0;
        self
    }
}

pub enum PackState {
    Added(Uuid),
    CapacityFull,
}
