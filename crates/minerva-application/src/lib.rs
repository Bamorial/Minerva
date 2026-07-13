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
    use super::BootstrapService;

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
}
