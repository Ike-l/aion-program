use std::{any::Any, fmt::Debug};

pub struct Resource {
    inner: Box<dyn Any + Send + Sync>,
}

impl Resource {
    pub fn new<T: 'static + Sync + Send>(value: T) -> Self {
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
}

impl Debug for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Resource>")
    }
}