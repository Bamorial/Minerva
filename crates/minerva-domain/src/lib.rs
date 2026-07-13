mod error;
mod error_code;
mod error_detail;
mod error_details;
mod event_id;
mod identifier_error;
mod project_id;
mod relationship_id;
mod task_id;
mod task_identity;

pub use error::MinervaError;
pub use error_code::ErrorCode;
pub use error_detail::{ErrorDetail, ErrorValue};
pub use event_id::EventId;
pub use identifier_error::IdentifierError;
pub use project_id::ProjectId;
pub use relationship_id::RelationshipId;
pub use task_id::{TaskId, TaskIdAllocator};
pub use task_identity::TaskIdentity;

pub const WORKSPACE_CRATES: [&str; 7] = [
    "minerva-domain",
    "minerva-application",
    "minerva-storage",
    "minerva-context",
    "minerva-cli",
    "minerva-tui",
    "minerva-mcp",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceKind {
    Cli,
    Tui,
    Mcp,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceBlueprint {
    crates: &'static [&'static str],
}

impl WorkspaceBlueprint {
    #[must_use]
    pub const fn new() -> Self {
        Self { crates: &WORKSPACE_CRATES }
    }

    #[must_use]
    pub const fn crates(self) -> &'static [&'static str] {
        self.crates
    }
}

impl Default for WorkspaceBlueprint {
    fn default() -> Self {
        Self::new()
    }
}
