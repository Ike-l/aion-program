use std::{collections::HashMap};

use tracing::{Level, event};

use crate::prelude::{ResourceId, StoredResource};

pub mod resource_id;
pub mod stored_resource;

pub struct RegistryStorage {
    inner: HashMap<ResourceId, StoredResource>
}

impl Default for RegistryStorage {
    fn default() -> Self {
        Self { inner: Default::default() }
    }
}

impl aion_state::prelude::RegistryStorage for RegistryStorage {
    type ValueId = ResourceId;
    type Value = StoredResource;

    fn get_mut(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<&mut Self::Value> {
        event!(Level::TRACE, "RegistryStorage get mut");

        self.inner.get_mut(value_id)
    }

    fn insert(
        &mut self, 
        value_id: Self::ValueId, 
        value: Self::Value
    ) -> Option<Self::Value> {
        event!(Level::TRACE, "RegistryStorage insert");

        self.inner.insert(value_id, value)
    }

    fn remove(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<Self::Value> {
        event!(Level::TRACE, "RegistryStorage remove");

        self.inner.remove(value_id)
    }

    fn contains_key(
        &self, 
        value_id: &Self::ValueId
    ) -> bool {
        event!(Level::TRACE, "RegistryStorage contains key");

        self.inner.contains_key(value_id)
    }
}