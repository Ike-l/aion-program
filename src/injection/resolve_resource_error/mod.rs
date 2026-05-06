#[derive(Debug, PartialEq)]
pub enum ResolveResourceError {
    Casting,
    Resolving,
    NotEnoughResults,
    TooManyResults,
}