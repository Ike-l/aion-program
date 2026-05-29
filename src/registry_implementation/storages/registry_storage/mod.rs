use std::{collections::HashMap, hash::Hash};

use tracing::{Level, event};

pub struct RegistryStorage<ValueId, StoredValue> {
    inner: HashMap<ValueId, StoredValue>
}

impl<ValueId, StoragedValue> Default for RegistryStorage<ValueId, StoragedValue> {
    fn default() -> Self {
        Self { inner: Default::default() }
    }
}

impl<ValueId: Eq + Hash, StoredValue> aion_state::prelude::RegistryStorage for RegistryStorage<ValueId, StoredValue> {
    type ValueId = ValueId;
    type Value = StoredValue;

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

    fn len(&self) -> usize {
        event!(Level::TRACE, "Registry Storage Len");

        self.inner.len()
    }
}