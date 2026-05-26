use std::{any::TypeId, sync::Arc};

use aion_program::prelude::{ProgramRegistry, ProgramRegistryResolveWithInsert, Resource, ResourceId, Shared, ProgramAccess};

use tracing::{span, event, Level};
use tracing_subscriber::{EnvFilter, fmt};
use std::sync::Once;

static INIT: Once = Once::new();

fn init_tracing() {
    INIT.call_once(|| {
        // let filter = EnvFilter::from_default_env()
        //     .add_directive(tracing::Level::DEBUG.into())
        //     .add_directive("aion_state=warn".parse().unwrap());

        fmt()
            .with_ansi(false)
            .compact()
            .pretty()
            .with_max_level(tracing::Level::TRACE)
            // .with_env_filter(filter)
            // .with_span_events(fmt::format::FmtSpan::ENTER | fmt::format::FmtSpan::EXIT)
            .with_target(false)
            .with_test_writer()           
            .init();
    });
    // .with_env_filter(EnvFilter::new("info,aion_state=info"))
}

#[test]
fn foo() {
    init_tracing();

    let program_registry = Arc::new(ProgramRegistry::default());

    let result = {
        let span = span!(tracing::Level::INFO, "Getting bool");
        let _enter = span.enter();
    
        program_registry.resolve_with_insert::<Shared<bool>>(None, vec![], ProgramRegistryResolveWithInsert {
            resource: Some(Box::new(|| Resource::new(true))),
            resource_id: Some(ResourceId::TypeId(TypeId::of::<bool>())),
            ..Default::default()
        })
    };

    match result {
        Ok(item) => {
            assert!(item.as_ref());
            drop(item);
            let access = program_registry.get_program_access(None);
            match access {
                Some(program_access) => {
                    event!(Level::INFO, %program_access, "program access");

                    match program_access {
                        ProgramAccess::Shared(0) => (),
                        _ => panic!("Expected shared to 0")
                    }
                },
                None => {
                    unreachable!()
                }
            }
        },
        Err(err) => panic!("{err}"),
    }
}