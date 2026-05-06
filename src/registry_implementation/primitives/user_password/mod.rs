use std::any::TypeId;

#[derive(Debug, PartialEq, Clone)]
pub enum UserPassword {
    Label(String),
    Number(u64),
    TypeId(TypeId)
}

impl UserPassword {
    pub fn type_id<T: 'static>() -> Self {
        Self::TypeId(TypeId::of::<T>())
    }
}