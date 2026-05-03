use std::collections::HashMap;

use rand::{Rng, rngs::ThreadRng};
use tracing::{Level, event};

use crate::prelude::{Access, ResourceId, ResourcePassword};

pub mod resource_password;

pub struct BlacklistStorage {
    inner: HashMap<ResourceId, Vec<(Access, ResourcePassword)>>,
    rng: ThreadRng
}

impl Default for BlacklistStorage {
    fn default() -> Self {
        Self { inner: Default::default(), rng: ThreadRng::default() }
    }
}

impl BlacklistStorage
    where u64: Into<ResourcePassword>
{
    pub fn generate_password(&mut self) -> ResourcePassword {
        self.rng.next_u64().into()
    }
}

impl aion_state::prelude::BlacklistStorage for BlacklistStorage {
    type Id = ResourceId;
    type Access = Access;
    type Password = ResourcePassword;

    fn check_access(
        &self,
        id: &Self::Id,
        access: &Self::Access,
        password: &Self::Password
    ) -> bool {
        event!(Level::TRACE, "Blacklist check access");

        let Some(allowed_accesses) = self.inner.get(id) else { return false };
        allowed_accesses.iter().any(|(allowed_access, access_password)| allowed_access == access && access_password == password)
    }

    fn allow(
        &mut self,
        id: Self::Id,
        access: Self::Access
    ) -> Option<Self::Password> {
        event!(Level::TRACE, "Blacklist allow");

        let generated_password = self.generate_password();

        self.inner.entry(id).or_default().push((access, generated_password.clone()));

        Some(generated_password)
    }

    fn unallow(
        &mut self,
        id: &Self::Id,
        access: &Self::Access
    ) -> bool {
        event!(Level::TRACE, "Blacklist unallow");

        let Some(allowed_accesses) = self.inner.get_mut(id) else { return false };

        let Some(position) = allowed_accesses.iter().position(|(allowed_access, _)| allowed_access == access) else { return false };

        allowed_accesses.remove(position);

        true
    }
    
    fn release(
        &mut self,
        id: &Self::Id
    ) -> bool {
        event!(Level::TRACE, "Blacklist release");

        self.inner.remove(id).is_some()
    }

    fn release_all<'a>(
        &mut self,
        mut ids: impl Iterator<Item = &'a Self::Id>
    ) -> bool where <Self as aion_state::prelude::BlacklistStorage>::Id: 'a {
        event!(Level::TRACE, "Blacklist release all");

        !ids.any(|resource_id| !self.release(resource_id))
    }
}
