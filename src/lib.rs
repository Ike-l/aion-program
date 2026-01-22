pub mod program_registry;
pub mod ids;
pub mod access;

pub mod prelude {
    pub use crate::{
        access::{
            access_result::AccessResult,
            borrow_type::BorrowType,
            program_access::ProgramAccess,
            resource_access::ResourceAccess,
        },
        ids::{
            program_id::ProgramId,
            program_key_id::ProgramKeyId,
            program_reserver_id::ProgramReserverId,
            resource_id::ResourceId,
            resource_key_id::ResourceKeyId,
            resource_reserver_id::ResourceReserverId,
        },
        program_registry::{
            ProgramRegistry,
            access_parameter::{
                AccessParameter
            },
            program_registry_results::ProgramRegistryResolveResult,
            injected::{
                Injected, 
                injection_error::InjectionError
            },
            program::{
                Program, StoredProgram,
                resource::{
                    Resource, 
                    resolved_resource::ResolvedResource,
                    casted_resource::CastedResource,
                    stored_resource::StoredResource,
                }
            }
        }
    };
}