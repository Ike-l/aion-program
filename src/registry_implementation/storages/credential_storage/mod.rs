use std::collections::HashMap;

use tracing::{Level, event};

pub struct CredentialStorage {
    inner: HashMap<UserId, UserPassword>
}

impl Default for CredentialStorage {
    fn default() -> Self {
        Self { inner: Default::default() }
    }
}

impl aion_state::prelude::CredentialStorage for CredentialStorage {
    type Id = UserId;
    type Password = UserPassword;

    fn verify(
        &self,
        id: &Self::Id, 
        password: &Self::Password
    ) -> bool {
        event!(Level::TRACE, "CredentialStorage verify");

        self.inner.get(id).is_some_and(|registered_password| registered_password == password)
    }

    fn register(
        &mut self,
        id: Self::Id,
        password: Self::Password
    ) -> bool {
        event!(Level::TRACE, "CredentialStorage register");

        if self.inner.contains_key(&id) {
            return false
        }
        
        self.inner.insert(id, password).is_none()
    }

    fn update_password(
        &mut self,
        id: &Self::Id,
        new_password: Self::Password
    ) -> bool {
        event!(Level::TRACE, "CredentialStorage update password");

        let Some(old_password) = self.inner.get_mut(id) else { return false };

        *old_password = new_password;

        true
    }

    fn unregister(
        &mut self,
        id: &Self::Id
    ) -> bool {
        event!(Level::TRACE, "CredentialStorage unregister");

        self.inner.remove(id).is_some()
    }
}