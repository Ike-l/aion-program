use std::{any::Any, fmt::Debug};

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
    pub fn as_ref<T: 'static>(&self) -> Option<&T> {
        self.inner.downcast_ref()
    }

    pub fn as_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.inner.downcast_mut()
    }

    pub fn is<T: 'static>(&self) -> bool {
        self.inner.is::<T>()
    }

    pub fn as_box<T: 'static>(self) -> Result<Box<T>, Self> {
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