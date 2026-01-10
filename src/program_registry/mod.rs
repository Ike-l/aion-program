use std::sync::Arc;

use aion_state::prelude::{Registry, RegistryAccessResult};

pub mod program;
pub mod injected;
pub mod program_registry_results;

use crate::access::access_result::AccessResult;
use crate::ids::program_key_id::ProgramKeyId;
use crate::ids::resource_id::ResourceId;
use crate::ids::resource_reserver_id::ResourceReserverId;
use crate::ids::{
    program_id::ProgramId, program_reserver_id::ProgramReserverId, resource_key_id::ResourceKeyId
};

use crate::access::Access;
use program::{
    Program, stored_program::StoredProgram,
};

use program_registry_results::{
    ProgramRegistryResolveResult
};

use injected::Injected;

pub struct ProgramRegistry {
    registry: Registry<ProgramId, ProgramReserverId, Access<StoredProgram, Arc<Program>>, ProgramId, ProgramKeyId, Box<StoredProgram>>,
    global_program_id: ProgramId
}

impl ProgramRegistry {
    pub fn resolve<T: Injected>(
        &self,
        program_id: Option<ProgramId>,
        resource_id: Option<ResourceId>,
        
        program_reserver_id: Option<&ProgramReserverId>,
        resource_reserver_id: Option<&ResourceReserverId>,

        program_key_id: Option<&ProgramKeyId>,
        resource_key_id: Option<&ResourceKeyId>,
    ) -> ProgramRegistryResolveResult<'_, T> {
        let program_id = program_id.unwrap_or(self.global_program_id.clone());

        // deaccesses happen when?
        // can manually deaccess by checking if there are no internal accesses
        let accessed_program = self.registry.access(program_id, Access::Shared(1), program_reserver_id, program_key_id);

        match accessed_program {
            RegistryAccessResult::Found(AccessResult::Shared(program)) => ProgramRegistryResolveResult::Found(
                    program.resolve(
                        resource_id,
                        resource_reserver_id,
                        resource_key_id,
                    )
                ),
            _ => ProgramRegistryResolveResult::AccessFailure
        }
    }
}

impl Default for ProgramRegistry {
    fn default() -> Self {
        let registry = Registry::default();

        let global_program_id = ProgramId::new("__Global__");
        registry.accessed_replacement(global_program_id.clone(), Access::Replace, None, None, Some(StoredProgram::default()));

        Self {
            registry,
            global_program_id,
        }
    }
}