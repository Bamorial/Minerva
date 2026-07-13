use crate::ErrorCode;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum MinervaError {
    #[error("project is not initialized")]
    ProjectNotInitialized,
    #[error("project is already initialized")]
    ProjectAlreadyInitialized,
    #[error("task `{task_ref}` was not found")]
    TaskNotFound { task_ref: String },
    #[error("task reference `{task_ref}` is ambiguous")]
    AmbiguousTaskReference { task_ref: String, matches: Vec<String> },
    #[error("cannot transition task from `{from}` to `{to}`")]
    InvalidStatusTransition { from: String, to: String },
    #[error("adding `{child}` under `{parent}` would create a hierarchy cycle")]
    HierarchyCycle { parent: String, child: String },
    #[error("dependency from `{task}` to `{depends_on}` would create a cycle")]
    DependencyCycle { task: String, depends_on: String },
    #[error("schema error in `{path}`")]
    SchemaError { path: String, reason: String },
    #[error("version conflict for `{path}`")]
    VersionConflict { path: String, expected: String, actual: String },
    #[error("could not acquire lock for `{path}`")]
    LockConflict { path: String },
    #[error("invalid configuration at `{key}`")]
    InvalidConfiguration { key: String, reason: String },
    #[error("failed to launch editor `{editor}`")]
    EditorLaunchFailure { editor: String, reason: String },
}

impl MinervaError {
    #[must_use]
    pub const fn code(&self) -> ErrorCode {
        match self {
            Self::ProjectNotInitialized => ErrorCode::ProjectNotInitialized,
            Self::ProjectAlreadyInitialized => ErrorCode::ProjectAlreadyInitialized,
            Self::TaskNotFound { .. } => ErrorCode::TaskNotFound,
            Self::AmbiguousTaskReference { .. } => ErrorCode::AmbiguousTaskReference,
            Self::InvalidStatusTransition { .. } => ErrorCode::InvalidStatusTransition,
            Self::HierarchyCycle { .. } => ErrorCode::HierarchyCycle,
            Self::DependencyCycle { .. } => ErrorCode::DependencyCycle,
            Self::SchemaError { .. } => ErrorCode::SchemaError,
            Self::VersionConflict { .. } => ErrorCode::VersionConflict,
            Self::LockConflict { .. } => ErrorCode::LockConflict,
            Self::InvalidConfiguration { .. } => ErrorCode::InvalidConfiguration,
            Self::EditorLaunchFailure { .. } => ErrorCode::EditorLaunchFailure,
        }
    }
}
