use std::{any::Any, cell::UnsafeCell, fmt::Debug};

pub mod stored_resource;
pub mod resolved_resource;
pub mod casted_resource;

pub struct Resource {
    inner: UnsafeCell<Box<dyn Any>>,
}

impl Resource {
    pub unsafe fn as_ref<T: 'static>(&self) -> Option<&T> {
        let boxed = unsafe { & *self.inner.get() };
        boxed.downcast_ref()
    }

    pub unsafe fn as_mut<T: 'static>(&mut self) -> Option<&mut T> {
        let boxed = unsafe { &mut *self.inner.get() };
        boxed.downcast_mut()
    }

    pub fn is<T: 'static>(&self) -> bool {
        let boxed = unsafe { & *self.inner.get() };
        boxed.is::<T>()
    }

    pub fn clone<T: 'static + Clone>(&self) -> Option<T> {
        Some(unsafe { self.as_ref::<T>() }?.clone())
    } 
}

impl Debug for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Resource>")
    }
}
