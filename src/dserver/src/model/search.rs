use super::Pack;
use std::sync::{Arc, Mutex};

/// Search data for item's get operation.
#[derive(Debug)]
pub struct Search {
    pub pack: Arc<Mutex<Pack>>,
    pub key: String,
}
