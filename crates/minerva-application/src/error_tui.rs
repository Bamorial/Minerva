use minerva_domain::{ErrorCode, MinervaError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuiErrorMessage {
    pub title: &'static str,
    pub body: String,
}

#[must_use]
pub fn render_tui(error: &MinervaError) -> TuiErrorMessage {
    TuiErrorMessage { title: title_for(error.code()), body: error.to_string() }
}

const fn title_for(code: ErrorCode) -> &'static str {
    match code {
        ErrorCode::ProjectNotInitialized => "Project not initialized",
        ErrorCode::ProjectAlreadyInitialized => "Project already initialized",
        ErrorCode::TaskNotFound => "Task not found",
        ErrorCode::AmbiguousTaskReference => "Ambiguous task reference",
        ErrorCode::InvalidStatusTransition => "Invalid status transition",
        ErrorCode::HierarchyCycle => "Hierarchy cycle",
        ErrorCode::DependencyCycle => "Dependency cycle",
        ErrorCode::SchemaError => "Schema error",
        ErrorCode::VersionConflict => "Version conflict",
        ErrorCode::LockConflict => "Lock conflict",
        ErrorCode::InvalidConfiguration => "Invalid configuration",
        ErrorCode::EditorLaunchFailure => "Editor launch failure",
    }
}
