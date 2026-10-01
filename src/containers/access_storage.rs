use std::hash::Hash;

use crate::prelude::InnerContainer;

#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
#[serde(bound(
    deserialize = "ResourceId: Eq + Hash, InnerContainer<ResourceId, Access>: serde::Deserialize<'de>"
))]
pub struct AccessStorage<ResourceId, Access> {
    inner: InnerContainer<ResourceId, Access>
}

impl<ResourceId, Access> aion_state::prelude::AccessStorage for AccessStorage<ResourceId, Access> 
    where 
        ResourceId: Eq + Hash
{
    type ValueId = ResourceId;
    type Access = Access;

    fn get_mut(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<&mut Self::Access> {
        self.inner.get_mut(value_id)
    }

    fn get(
        &self, 
        value_id: &Self::ValueId
    ) -> Option<&Self::Access> {
        self.inner.get(value_id)
    }

    fn insert(
        &mut self,
        value_id: Self::ValueId,
        access: Self::Access
    ) -> Option<Self::Access> {
        self.inner.insert(value_id, access)
    }

    fn drain(&mut self) -> impl Iterator<Item = (Self::ValueId, Self::Access)> {
        self.inner.drain()
    }
}