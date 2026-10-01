use aion_state::prelude::StoredValueTrait;
use stable_deref_trait::StableDeref;

use crate::prelude::Resource;

pub mod resource;

#[derive(small_derive_deref::Deref)]
pub struct StoredResource { 
    resource: Box<Resource>
}

unsafe impl StableDeref for StoredResource {}

impl StoredValueTrait for StoredResource {
    type Value = Resource;

    fn new(value: Self::Value) -> Self {
        Self {
            resource: Box::new(value)
        }
    }

    fn as_shared(&self) -> &Self::Value {
        &self.resource
    }

    fn as_unique(&mut self) -> &mut Self::Value {
        &mut self.resource
    }

    fn into_inner(self) -> Self::Value {
        *self.resource
    }
}