use std::any::TypeId;

use aion_state::prelude::{AccessKey, ResourceKey};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum ResourceId {
    Labelled(String),
    Raw(TypeId)
}

impl ResourceKey for ResourceId {}
impl AccessKey for ResourceId {}

impl ResourceId {
    pub fn labelled<T: Into<String>>(label: T) -> Self {
        Self::Labelled(label.into())
    }

    pub fn raw<T: 'static>() -> Self {
        Self::Raw(TypeId::of::<T>())
    }
}