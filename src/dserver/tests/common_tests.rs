#[cfg(test)]
mod tests {
    use dserver::{Item, Pack, PackState, Value};

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

    #[test]
    fn should_primitive_values_works() {
        let logson = Item::new("logs_on", Value::Logical(true)).unwrap();
        assert_eq!(logson.value, Value::Logical(true));

        let max_player = Item::new("maxplayer", Value::ThinNumber(8)).unwrap();
        assert_eq!(max_player.value, Value::ThinNumber(8));

        let default_value = Item::new("defaultvalue", Value::ThinFloat(3.22)).unwrap();
        assert_eq!(default_value.value, Value::ThinFloat(3.22));

        let edge_of_tomorrow =
            Item::new("pi", Value::LargeFloat(24.342343243423423423431415)).unwrap();
        assert_eq!(
            edge_of_tomorrow.value,
            Value::LargeFloat(24.342343243423423423431415)
        );
    }

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


