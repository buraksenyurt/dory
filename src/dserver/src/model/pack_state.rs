use uuid::Uuid;

/// It gives information about the pack status.
#[derive(Debug, PartialEq)]
pub enum PackState {
    Added(Uuid),
    CapacityFull,
}
