use std::any::TypeId;

#[derive(Debug, PartialEq, Clone)]
pub enum UserPassword {
    Label(String),
    Number(u64),
    TypeId(TypeId)
}