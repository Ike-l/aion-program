use std::{collections::HashMap, fmt::Debug, hash::Hash};

use rand::{Rng, rngs::ThreadRng};
use tracing::{Level, event, span};

use crate::prelude::ValuePassword;

pub struct BlacklistStorage<ValueId, Access> {
    inner: HashMap<ValueId, Vec<(Access, ValuePassword)>>,
    rng: ThreadRng
}

impl<ValueId, Access> Default for BlacklistStorage<ValueId, Access> {
    fn default() -> Self {
        Self { inner: Default::default(), rng: ThreadRng::default() }
    }
}

impl<ValueId, Access> BlacklistStorage<ValueId, Access> {
    pub fn generate_password(&mut self) -> ValuePassword {
        self.rng.next_u64().into()
    }
}

impl<ValueId, Access> aion_state::prelude::BlacklistStorage for BlacklistStorage<ValueId, Access>
    where 
        ValueId: Eq + Hash + Debug,
        Access: PartialEq,
{
    type Id = ValueId;
    type Access = Access;
    type Password = ValuePassword;

    fn check_access(
        &self,
        id: &Self::Id,
        access: &Self::Access,
        password: &Self::Password
    ) -> bool {
        event!(Level::TRACE, "Blacklist check access");

        let Some(allowed_accesses) = self.inner.get(id) else { 
            event!(Level::TRACE, "No Id Found");

            return true
        };

        allowed_accesses.iter().any(|(allowed_access, access_password)| {
            allowed_access == access && access_password == password
        })
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

        let Some(allowed_accesses) = self.inner.get_mut(id) else { 
            event!(Level::TRACE, "No Id Found");

            return false 
        };

        let Some(position) = allowed_accesses.iter().position(|(allowed_access, _)| allowed_access == access) else { 
            event!(Level::TRACE, "Access Does not Match an Allowed Access");

            return false 
        };

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

        !ids.any(|resource_id| {
            let span = span!(Level::TRACE, "Releasing", resource_id =? resource_id);
            let _enter = span.enter();
            
            !self.release(resource_id)
        })
    }
}
