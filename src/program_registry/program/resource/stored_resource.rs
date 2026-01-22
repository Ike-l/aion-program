use std::cell::UnsafeCell;

use crate::prelude::Resource;

pub struct StoredResource {
    resource: UnsafeCell<Resource>
}

impl StoredResource {
    pub unsafe fn as_ref(&self) -> &Resource {
        unsafe { &*self.resource.get() }
    }

    pub unsafe fn as_mut(&self) -> &mut Resource {
        unsafe { &mut *self.resource.get() }
    }
}
