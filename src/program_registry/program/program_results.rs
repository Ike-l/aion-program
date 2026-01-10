use crate::program_registry::injected::{Injected, injection_error::InjectionError};

pub enum ProgramResolveResult<'a, T: Injected> {
    Found(Result<T::Item<'a>, InjectionError>),
    AccessFailure
}