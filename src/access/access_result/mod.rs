use crate::program_registry::program::resource::Resource;

pub enum AccessResult<'a, T> {
    Shared(&'a T),
    Unique(&'a mut T),
    // Option purely to Take
    // CANNOT be None on Init
    Owned(Option<T>)
}

impl<'a> AccessResult<'a, Resource> {
    pub unsafe fn resolve<Y: 'static>(self) -> Result<AccessResult<'a, Y>, Self> {
        unsafe { match self {
            AccessResult::Shared(inner) => inner.as_ref().map(AccessResult::Shared).ok_or(self),
            AccessResult::Unique(inner) => {
                if inner.is::<Y>() {
                    Ok(AccessResult::Unique(inner.as_mut().unwrap()))
                } else {
                    Err(AccessResult::Unique(inner))
                }
            },
            AccessResult::Owned(_) => Err(self),
        } }
    }

    pub fn resolve_clone<Y: 'static + Clone>(self) -> Result<AccessResult<'a, Y>, Self> {
        match self {
            AccessResult::Owned(inner) => {
                if let Some(inner) = inner {
                    Ok(AccessResult::Owned(inner.clone().unwrap()))
                } else {
                    Err(AccessResult::Owned(None))
                }
            },
            _ => return Err(self)
        }
    }
}

impl<'a, T> AccessResult<'a, T> {
    pub fn as_ref(&self) -> Option<&'_ T> {
        match self {
            AccessResult::Shared(inner) => Some(inner),
            _ => None
        }
    }

    pub fn as_mut(&mut self) -> Option<&'_ mut T> {
        match self {
            AccessResult::Unique(inner) => Some(inner),
            _ => None
        }
    }

    pub fn take(&mut self) -> Option<Option<T>> {
        match self {
            AccessResult::Owned(inner) => Some(inner.take()),
            _ => None
        }
    }
}