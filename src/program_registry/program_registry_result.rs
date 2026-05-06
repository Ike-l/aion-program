pub enum ProgramRegistryReplaceResourceError {
    NotFound,
    AccessConflict,
    ReservationConflict,
    VerificationFailure,
    OwnershipDenied,
    WhitelistDenied,
    BlacklistDenied,
}