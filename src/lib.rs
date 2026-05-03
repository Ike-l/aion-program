pub mod program;
pub mod injection;

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
        program::{
            Program,
            storage::{
                Storage,
                registry_storage::{
                    RegistryStorage,
                    resource_id::{
                        ResourceId
                    },
                    stored_resource::{
                        StoredResource,
                        resource::{
                            Resource
                        }
                    },
                },
                access_storage::{
                    AccessStorage,
                    access::{
                        Access,
                        access_result::{
                            AccessResult
                        },
                        borrow_type::{
                            BorrowType
                        }
                    }
                }
            }
        }
    };
}