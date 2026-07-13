mod error_cli;
mod error_mcp;
mod error_tui;
mod project_repository;
mod task_create_record;
mod task_creation_request;
mod task_creation_result;
mod task_creation_service;
mod task_move_request;
mod task_move_result;
mod task_movement_service;
mod task_repository;
mod task_slug_builder;

pub use error_cli::{CliErrorReport, render_cli};
pub use error_mcp::{McpErrorData, McpErrorResponse, render_mcp};
pub use error_tui::{TuiErrorMessage, render_tui};
pub use project_repository::ProjectRepository;
pub use task_create_record::TaskCreateRecord;
pub use task_creation_request::CreateTaskRequest;
pub use task_creation_result::TaskCreationResult;
pub use task_creation_service::TaskCreationService;
pub use task_move_request::MoveTaskRequest;
pub use task_move_result::TaskMoveResult;
pub use task_movement_service::TaskMovementService;
pub use task_repository::{TaskRepository, TaskWriteResult};

use minerva_domain::{InterfaceKind, WorkspaceBlueprint};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterfaceDescription {
    pub kind: InterfaceKind,
    pub crate_name: &'static str,
    pub entrypoint: &'static str,
}

pub struct BootstrapService;

impl BootstrapService {
    #[must_use]
    pub fn workspace_blueprint() -> WorkspaceBlueprint {
        WorkspaceBlueprint::new()
    }

    #[must_use]
    pub fn interface_descriptions() -> [InterfaceDescription; 3] {
        [
            InterfaceDescription {
                kind: InterfaceKind::Cli,
                crate_name: "minerva-cli",
                entrypoint: "src/main.rs",
            },
            InterfaceDescription {
                kind: InterfaceKind::Tui,
                crate_name: "minerva-tui",
                entrypoint: "src/main.rs",
            },
            InterfaceDescription {
                kind: InterfaceKind::Mcp,
                crate_name: "minerva-mcp",
                entrypoint: "src/main.rs",
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::{BootstrapService, render_cli, render_mcp, render_tui};
    use minerva_domain::MinervaError;

    #[test]
    fn interfaces_are_reported_without_embedding_business_rules() {
        let crates = BootstrapService::workspace_blueprint().crates().len();
        let interfaces = BootstrapService::interface_descriptions();
        assert_eq!(crates, 7);
        assert_eq!(
            interfaces.map(|item| item.crate_name),
            ["minerva-cli", "minerva-tui", "minerva-mcp",]
        );
    }

    #[test]
    fn interface_error_renderers_share_one_domain_error() {
        let error = MinervaError::InvalidConfiguration {
            key: "editor".into(),
            reason: "empty".into(),
        };
        let cli = render_cli(&error);
        let tui = render_tui(&error);
        let mcp = render_mcp(&error);
        assert_eq!(cli.code, "invalid_configuration");
        assert_eq!(tui.title, "Invalid configuration");
        assert_eq!(mcp.data.minerva_code, "invalid_configuration");
    }
}
