use minerva_domain::ErrorCode;
use std::process::ExitCode;

pub const INTERNAL_FAILURE: u8 = 1;
pub const REBUILD_FAILURE: u8 = 22;
pub const VALIDATION_WARNING: u8 = 23;
pub const VALIDATION_ERROR: u8 = 24;

pub const fn for_domain(code: ErrorCode) -> u8 {
    match code {
        ErrorCode::ProjectNotInitialized => 10,
        ErrorCode::ProjectAlreadyInitialized => 11,
        ErrorCode::TaskNotFound => 12,
        ErrorCode::AmbiguousTaskReference => 13,
        ErrorCode::InvalidStatusTransition => 14,
        ErrorCode::HierarchyCycle => 15,
        ErrorCode::DependencyCycle => 16,
        ErrorCode::SchemaError => 17,
        ErrorCode::VersionConflict => 18,
        ErrorCode::LockConflict => 19,
        ErrorCode::InvalidConfiguration => 20,
        ErrorCode::EditorLaunchFailure => 21,
    }
}

pub fn code(value: u8) -> ExitCode {
    ExitCode::from(value)
}
