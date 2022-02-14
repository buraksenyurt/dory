use super::{Item, Pack};
use std::sync::{Arc, Mutex};

/// Carries candidate object information to be added to the package.
#[derive(Debug)]
pub struct Candidate {
    pub pack: Arc<Mutex<Pack>>,
    pub object: Item,
}
