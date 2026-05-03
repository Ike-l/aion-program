#[derive(PartialEq, Clone)]
pub struct ResourcePassword {
    password: u64
}

impl From<u64> for ResourcePassword {
    fn from(value: u64) -> Self {
        Self { password: value }
    }
}