use std::any::TypeId;

#[derive(PartialEq, Clone)]
pub enum UserPassword {
    Label(String),
    Number(u64),
    TypeId(TypeId)
}