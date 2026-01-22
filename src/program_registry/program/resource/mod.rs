use std::{any::Any, fmt::Debug};

pub mod resolved_resource;
pub mod casted_resource;
pub mod stored_resource;

pub struct Resource {
    inner: Box<dyn Any>,
}

impl Resource {
    pub fn new<T: 'static>(value: T) -> Self {
        Self {
            inner: Box::new(value)
        }
    }
}

impl Resource {
    pub unsafe fn as_ref<T: 'static>(&self) -> Option<&T> {
        self.inner.downcast_ref()
    }

    pub unsafe fn as_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.inner.downcast_mut()
    }

    pub unsafe fn is<T: 'static>(&self) -> bool {
        self.inner.is::<T>()
    }

    pub unsafe fn as_box<T: 'static>(self) -> Result<Box<T>, Self> {
        match self.inner.downcast() {
            Ok(boxed) => Ok(boxed),
            Err(inner) => Err(Self { inner }),
        }
    }
}

impl Debug for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Resource>")
    }
}
