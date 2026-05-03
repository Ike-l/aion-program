pub mod injection;
pub mod program_registry;
pub mod registry_implementation;

pub mod prelude {
    pub use super::{
        injection::{
            Injection,
            access_builder::{
                AccessBuilder
            },
            finalised_access::{
                FinalisedAccess
            },
            resolve_resource_error::{
                ResolveResourceError
            },
            derived_resource::{
                DerivedResource
            },
        },
        program_registry::{
            ProgramRegistry,
            prompted_accesses::{
                PromptedAccesses
            },
        },
        registry_implementation::{
            access_storage::{
                AccessStorage,
            },
            blacklist_storage::{
                BlacklistStorage,
                value_password::{
                    ValuePassword
                }
            }
        }
    };
}