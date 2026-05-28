use crate::prelude::{CastError, Resource};

pub enum AccessResult<'a, T> {
    Shared(&'a T),
    Unique(&'a mut T),
}

impl<'a> AccessResult<'a, Resource> {
    pub fn cast<Y: 'static>(self) -> Result<AccessResult<'a, Y>, CastError> {
        match self {
            AccessResult::Shared(inner) => {
                match inner.as_ref::<Y>() {
                    Ok(resource) => Ok(AccessResult::Shared(resource)),
                    Err(downcast_error) => Err(CastError::Shared(downcast_error)),
                }
            },
            AccessResult::Unique(inner) => {
                match inner.as_mut::<Y>() {
                    Ok(resource) => Ok(AccessResult::Unique(resource)),
                    Err(downcast_error) => Err(CastError::Unique(downcast_error)),
                }
            },
        }
    }
}

impl<'a, T> AccessResult<'a, T> {
    pub fn as_ref(&self) -> Option<&'_ T> {
        match self {
            Self::Shared(inner) => Some(inner),
            Self::Unique(inner) => Some(inner),
        }
    }

    pub fn as_mut(&mut self) -> Option<&'_ mut T> {
        match self {
            Self::Unique(inner) => Some(inner),
            Self::Shared(_) => panic!("Cannot use a shared resource as unique!")
        }
    }
}