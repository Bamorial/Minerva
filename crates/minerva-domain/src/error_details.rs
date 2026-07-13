use crate::{ErrorDetail, MinervaError};

impl MinervaError {
    #[must_use]
    pub fn details(&self) -> Vec<ErrorDetail> {
        match self {
            Self::ProjectNotInitialized | Self::ProjectAlreadyInitialized => Vec::new(),
            Self::TaskNotFound { task_ref } => {
                vec![ErrorDetail::text("task_ref", task_ref)]
            }
            Self::AmbiguousTaskReference { task_ref, matches } => vec![
                ErrorDetail::text("task_ref", task_ref),
                ErrorDetail::list("matches", matches.clone()),
            ],
            Self::InvalidStatusTransition { from, to } => {
                vec![ErrorDetail::text("from", from), ErrorDetail::text("to", to)]
            }
            Self::HierarchyCycle { parent, child } => {
                vec![
                    ErrorDetail::text("parent", parent),
                    ErrorDetail::text("child", child),
                ]
            }
            Self::DependencyCycle { task, depends_on } => {
                vec![
                    ErrorDetail::text("task", task),
                    ErrorDetail::text("depends_on", depends_on),
                ]
            }
            Self::SchemaError { path, reason } => {
                vec![
                    ErrorDetail::text("path", path),
                    ErrorDetail::text("reason", reason),
                ]
            }
            Self::VersionConflict { path, expected, actual } => vec![
                ErrorDetail::text("path", path),
                ErrorDetail::text("expected", expected),
                ErrorDetail::text("actual", actual),
            ],
            Self::LockConflict { path } => vec![ErrorDetail::text("path", path)],
            Self::InvalidConfiguration { key, reason } => {
                vec![ErrorDetail::text("key", key), ErrorDetail::text("reason", reason)]
            }
            Self::EditorLaunchFailure { editor, reason } => {
                vec![
                    ErrorDetail::text("editor", editor),
                    ErrorDetail::text("reason", reason),
                ]
            }
        }
    }
}
