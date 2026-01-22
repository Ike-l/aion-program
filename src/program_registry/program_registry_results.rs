use crate::prelude::{InjectionError, Injected};

pub enum ProgramRegistryResolveResult<'a, T: Injected> {
    Found(Result<T::Item<'a>, InjectionError>),
    AccessFailure,
    ExpectsGlobalProgram
}