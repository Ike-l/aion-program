use std::hash::Hash;

use rand::prelude::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;

use crate::prelude::InnerContainer;

type Inner<ResourceId, Access, Password> = InnerContainer<ResourceId, Vec<(Access, Password)>>;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(bound(
    serialize = "Inner<ResourceId, Access, Password>: serde::Serialize",
    deserialize = "ResourceId: Eq + Hash, Inner<ResourceId, Access, Password>: serde::Deserialize<'de>"
))]
pub struct BlacklistStorage<ResourceId, Access, Password> {
    inner: Inner<ResourceId, Access, Password>,
    rng: ChaChaRng
}

impl<ResourceId, Access, Password> BlacklistStorage<ResourceId, Access, Password> {
    pub fn new(seed: [u8; 32]) -> Self {
        Self {
            inner: Default::default(),
            rng: ChaChaRng::from_seed(seed)
        }
    }
}

impl<ResourceId, Access, Password> BlacklistStorage<ResourceId, Access, Password> 
    where 
        u64: Into<Password>
{
    pub fn generate_next_password(&mut self) -> Password {
        self.rng.next_u64().into()
    }
}

impl<ResourceId, Access, Password> aion_state::prelude::BlacklistStorage for BlacklistStorage<ResourceId, Access, Password>
    where 
        ResourceId: Eq + Hash,
        Access: PartialEq,
        Password: PartialEq + Clone,
        u64: Into<Password>
{
    type Id = ResourceId;
    type Access = Access;
    type Password = Password;

    fn check_access(
        &self,
        id: &Self::Id,
        access: &Self::Access,
        password: &Self::Password
    ) -> bool {
        let Some(allowed_accesses) = self.inner.get(id) else { return false };
        
        allowed_accesses.iter().any(|(allowed_access, access_password)| allowed_access == access && access_password == password)
    }

    fn allow(
        &mut self,
        id: Self::Id,
        access: Self::Access
    ) -> Option<Self::Password> {
        let generated_password = self.generate_next_password();
        self.inner.entry(id).or_default().push((access, generated_password.clone()));
        Some(generated_password)
    }

    fn unallow(
        &mut self,
        id: &Self::Id,
        access: &Self::Access
    ) -> bool {
        let Some(allowed_accesses) = self.inner.get_mut(id) else { return false };
        let Some(position) = allowed_accesses.iter().position(|(allowed_access, _)| allowed_access == access) else { return false };
        allowed_accesses.remove(position);

        true
    }

    fn release(
        &mut self,
        id: &Self::Id
    ) -> bool {
        self.inner.remove(id).is_some()
    }

    fn release_all<'a>(
        &mut self,
        mut ids: impl Iterator<Item = &'a Self::Id>
    ) -> bool where <Self as aion_state::prelude::BlacklistStorage>::Id: 'a {
        !ids.any(|resource_id| !self.release(resource_id))
    }
}
