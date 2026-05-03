use crate::prelude::Resource;

pub enum AccessResult<'a, T> {
    Shared(&'a T),
    Unique(&'a mut T),
    Taken(T),
    Box(Box<T>),
}

impl<'a> AccessResult<'a, Resource> {
    pub fn cast<Y: 'static>(self) -> Result<AccessResult<'a, Y>, Self> {
        match self {
            AccessResult::Shared(inner) => inner.as_ref().map(AccessResult::Shared).ok_or(self),
            AccessResult::Unique(inner) => {
                if inner.is::<Y>() {
                    Ok(AccessResult::Unique(inner.as_mut().unwrap()))
                } else {
                    Err(AccessResult::Unique(inner))
                }
            },
            AccessResult::Taken(inner) => inner.as_box().map(AccessResult::Box).map_err(Self::Taken),
            AccessResult::Box(_) => panic!("Cannot Resolve AccessResult::Box")
        }
    }
}

impl<'a, T> AccessResult<'a, T> {
    pub fn as_ref(&self) -> Option<&'_ T> {
        match self {
            Self::Shared(inner) => Some(inner),
            Self::Unique(inner) => Some(inner),
            _ => None
        }
    }

    pub fn as_mut(&mut self) -> Option<&'_ mut T> {
        match self {
            Self::Unique(inner) => Some(inner),
            _ => None
        }
    }
}