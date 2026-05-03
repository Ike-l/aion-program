use std::collections::HashMap;

use tracing::{Level, event};

use crate::prelude::{ResourceId, UserId};

pub struct ControlStorage {
    inner: HashMap<ResourceId, UserId>
}

impl Default for ControlStorage {
    fn default() -> Self {
        Self { inner: Default::default() }
    }
}

impl aion_state::prelude::ControlStorage for ControlStorage {
    type Id = UserId;
    type ResourceId = ResourceId;

    fn check_owner(
        &self,
        id: &Self::Id,
        resource_id: &Self::ResourceId
    ) -> bool {
        event!(Level::TRACE, "ControlStorage check owner");

        self.inner.get(resource_id).is_some_and(|owner| owner == id)
    }

    fn release(
        &mut self,
        resource_id: &Self::ResourceId
    ) -> bool {
        event!(Level::TRACE, "ControlStorage release");

        self.inner.remove(resource_id).is_some()
    }

    fn own(
        &mut self,
        id: Self::Id,
        resource_id: Self::ResourceId
    ) -> bool {
        event!(Level::TRACE, "ControlStorage own");

        self.inner.insert(resource_id, id);

        true
    }

    fn is_owned(
        &self,
        resource_id: &Self::ResourceId
    ) -> bool {
        event!(Level::TRACE, "ControlStorage is owned");

        self.inner.contains_key(resource_id)
    }

    fn release_id(
        &mut self,
        id: &Self::Id
    ) -> impl Iterator<Item = Self::ResourceId> {
        event!(Level::TRACE, "ControlStorage release id");

        self.inner.extract_if(move |_, owner| owner == id).map(|(resource_id, _)| resource_id)
    }
}