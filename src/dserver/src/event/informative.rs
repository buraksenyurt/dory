use super::super::model::Item;
use std::sync::Arc;
use uuid::Uuid;

/// It is the enum that contains the event definitions to be used in the receiver channel.
#[derive(Debug)]
pub enum InformativeEvent {
    Added(Uuid),
    AddError,
    NotFound,
    Found(Arc<Item>),
    GetError,
}
