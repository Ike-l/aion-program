use std::sync::Arc;

use aion_state::prelude::StoredValueTrait;
use stable_deref_trait::StableDeref;

use crate::prelude::{Program};

pub mod program;

#[derive(Default, small_derive_deref::Deref)]
pub struct StoredProgram { 
    program: Arc<Program>
}

unsafe impl StableDeref for StoredProgram {}

impl StoredValueTrait for StoredProgram {
    type Value = StoredProgram;

    fn new(value: Self::Value) -> Self {
        value
    }

    fn as_shared(&self) -> &Self::Value {
        &self
    }

    fn as_unique(&mut self) -> &mut Self::Value { unimplemented!() }

    fn into_inner(self) -> Self::Value { unimplemented!() }
}