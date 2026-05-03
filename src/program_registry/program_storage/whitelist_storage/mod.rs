use std::collections::HashMap;

use tracing::{Level, event};

use crate::prelude::{Access, ResourceId};

pub struct WhitelistStorage {
    inner: HashMap<ResourceId, Vec<Access>>
}

impl Default for WhitelistStorage {
    fn default() -> Self {
        Self { inner: Default::default() }
    }
}

impl aion_state::prelude::WhitelistStorage for WhitelistStorage {
    type Id = ResourceId;
    type Access = Access;

    fn check_access(
        &self,
        id: &Self::Id,
        access: &Self::Access 
    ) -> bool {
        event!(Level::TRACE, "Whitelist check access");

        let Some(allowed_accesses) = self.inner.get(id) else { return false };
        allowed_accesses.iter().any(|allowed_access| allowed_access == access)
    }

    fn allow(
        &mut self,
        id: Self::Id,
        access: Self::Access
    ) -> bool {
        event!(Level::TRACE, "Whitelist allow");

        self.inner.entry(id).or_default().push(access);

        true
    }

    fn release(
        &mut self,
        id: &Self::Id
    ) -> bool {
        event!(Level::TRACE, "Whitelist release");

        self.inner.remove(id).is_some()
    }

    fn release_all<'a>(
        &mut self,
        mut ids: impl Iterator<Item = &'a Self::Id>
    ) -> bool where <Self as aion_state::prelude::WhitelistStorage>::Id: 'a {
        event!(Level::TRACE, "Whitelist release all");

        !ids.any(|resource_id| !self.release(resource_id))
    }

    fn unallow(
        &mut self,
        id: &Self::Id,
        access: &Self::Access
    ) -> bool {
        event!(Level::TRACE, "Whitelist unallow");

        let Some(allowed_accesses) = self.inner.get_mut(id) else { return false };

        let Some(position) = allowed_accesses.iter().position(|allowed_access| allowed_access == access) else { return false };

        allowed_accesses.remove(position);

        true
    }
}