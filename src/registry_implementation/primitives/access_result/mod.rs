use aion_state::prelude::AccessorResult;

use crate::prelude::{CastError, ValueTrait};

pub enum AccessResult<'a, T> {
    Shared(&'a T),
    Unique(&'a mut T),
    Owned(T)
}

impl<'a, T> AccessorResult<'a, T> for AccessResult<'a, T> {
    fn new_shared(value: &'a T) -> Self {
        AccessResult::Shared(value)
    }

    fn new_unique(value: &'a mut T) -> Self {
        AccessResult::Unique(value)
    }

    fn new_owned(value: T) -> Self {
        AccessResult::Owned(value)
    }
}

impl<'a, R: ValueTrait> AccessResult<'a, R> {
    pub fn cast<Y: 'static>(self) -> Result<AccessResult<'a, Y>, CastError> {
        match self {
            AccessResult::Shared(resource) => resource
                .as_ref::<Y>()
                .map(AccessResult::Shared)
                .map_err(CastError::Shared),

            AccessResult::Unique(resource) => resource
                .as_mut::<Y>()
                .map(AccessResult::Unique)
                .map_err(CastError::Unique),

            AccessResult::Owned(resource) => resource
                .into::<Y>()
                .map(AccessResult::Owned)
                .map_err(CastError::Owned),
        }
    }
}

impl<'a, T> AccessResult<'a, T> {
    pub fn as_ref(&self) -> Option<&'_ T> {
        match self {
            Self::Shared(inner) => Some(inner),
            Self::Unique(inner) => Some(inner),
            Self::Owned(inner) => Some(inner)
        }
    }

    pub fn as_mut(&mut self) -> Option<&'_ mut T> {
        match self {
            Self::Unique(inner) => Some(inner),
            Self::Shared(_) => panic!("Cannot use a shared resource as unique!"),
            Self::Owned(inner) => Some(inner)
        }
    }
}