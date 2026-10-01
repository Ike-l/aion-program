use std::hash::Hash;

use crate::prelude::InnerContainer;

#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
#[serde(bound(
    deserialize = "ResourceId: Eq + Hash, InnerContainer<ResourceId, Id>: serde::Deserialize<'de>"
))]
pub struct ControlStorage<ResourceId, Id> {
    inner: InnerContainer<ResourceId, Id>
}

impl<Id, ResourceId> aion_state::prelude::ControlStorage for ControlStorage<ResourceId, Id> 
    where
        Id: PartialEq,
        ResourceId: Eq + Hash
{
    type Id = Id;
    type ResourceId = ResourceId;

    fn check_owner(
        &self,
        id: &Self::Id,
        resource_id: &Self::ResourceId
    ) -> bool {
        self.inner.get(resource_id).is_some_and(|owner| owner == id)
    }

    fn release(
        &mut self,
        resource_id: &Self::ResourceId
    ) -> bool {
        self.inner.remove(resource_id).is_some()
    }

    fn own(
        &mut self,
        id: Self::Id,
        resource_id: Self::ResourceId
    ) -> bool {
        self.inner.insert(resource_id, id);

        true
    }

    fn is_owned(
        &self,
        resource_id: &Self::ResourceId
    ) -> bool {
        self.inner.contains_key(resource_id)
    }

    fn release_id(
        &mut self,
        id: &Self::Id
    ) -> impl Iterator<Item = Self::ResourceId> {
        self.inner.extract_if(move |_, owner| owner == id).map(|(resource_id, _)| resource_id)
    }
}