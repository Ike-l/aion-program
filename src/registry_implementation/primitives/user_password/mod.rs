use std::any::TypeId;

#[derive(PartialEq)]
pub enum UserPassword {
    Label(String),
    Number(u64),
    TypeId(TypeId)
}