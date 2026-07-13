#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    ProjectNotInitialized,
    ProjectAlreadyInitialized,
    TaskNotFound,
    AmbiguousTaskReference,
    InvalidStatusTransition,
    HierarchyCycle,
    DependencyCycle,
    SchemaError,
    VersionConflict,
    LockConflict,
    InvalidConfiguration,
    EditorLaunchFailure,
}

impl ErrorCode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectNotInitialized => "project_not_initialized",
            Self::ProjectAlreadyInitialized => "project_already_initialized",
            Self::TaskNotFound => "task_not_found",
            Self::AmbiguousTaskReference => "ambiguous_task_reference",
            Self::InvalidStatusTransition => "invalid_status_transition",
            Self::HierarchyCycle => "hierarchy_cycle",
            Self::DependencyCycle => "dependency_cycle",
            Self::SchemaError => "schema_error",
            Self::VersionConflict => "version_conflict",
            Self::LockConflict => "lock_conflict",
            Self::InvalidConfiguration => "invalid_configuration",
            Self::EditorLaunchFailure => "editor_launch_failure",
        }
    }
}
