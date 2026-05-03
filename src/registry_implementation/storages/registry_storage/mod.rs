use std::{collections::HashMap, hash::Hash};

use tracing::{Level, event};

pub struct RegistryStorage<ResourceId, StoredResource> {
    inner: HashMap<ResourceId, StoredResource>
}

impl<ResourceId, StoragedResource> Default for RegistryStorage<ResourceId, StoragedResource> {
    fn default() -> Self {
        Self { inner: Default::default() }
    }
}

impl<ResourceId: Eq + Hash, StoredResource> aion_state::prelude::RegistryStorage for RegistryStorage<ResourceId, StoredResource> {
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