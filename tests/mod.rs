use aion_program::program_registry::ProgramRegistry;
use aion_program::program_registry::injected::injected_primitives::shared::Shared;
use aion_program::program_registry::program::program_results::ProgramResolveResult;
use aion_program::program_registry::program_registry_results::ProgramRegistryResolveResult;

fn foo() {
    let program_registry = ProgramRegistry::default();

    let program_id = None;
    let resource_id = None;
    let program_reserver_id = None;
    let resource_reserver_id = None;
    let program_key_id = None;
    let resource_key_id = None;

    let shared = program_registry.resolve::<Shared<bool>>(program_id, resource_id, program_reserver_id, resource_reserver_id, program_key_id, resource_key_id);
    let ProgramRegistryResolveResult::Found(ProgramResolveResult::Found(Ok(shared))) = shared else { panic!() };
    let a = shared.as_ref();
}