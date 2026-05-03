use crate::prelude::Resource;

pub enum AccessResult<'a> {
    Shared(&'a Resource),
    Unique(&'a mut Resource),
    Owned(Resource)
}