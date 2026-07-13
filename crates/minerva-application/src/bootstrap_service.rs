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
            entry(InterfaceKind::Cli, "minerva-cli"),
            entry(InterfaceKind::Tui, "minerva-tui"),
            entry(InterfaceKind::Mcp, "minerva-mcp"),
        ]
    }
}

const fn entry(kind: InterfaceKind, crate_name: &'static str) -> InterfaceDescription {
    InterfaceDescription { kind, crate_name, entrypoint: "src/main.rs" }
}
