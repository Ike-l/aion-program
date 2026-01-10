use crate::program_registry::{injected::{Injected, injection_error::InjectionError}, program::program_results::ProgramResolveResult};

pub enum ProgramRegistryResolveResult<'a, T: Injected> {
    Found(ProgramResolveResult<'a, T>),
    AccessFailure
}