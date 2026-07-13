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

#[cfg(test)]
mod tests {
    use super::{InterfaceKind, WORKSPACE_CRATES, WorkspaceBlueprint};

    #[test]
    fn blueprint_lists_all_workspace_crates() {
        let blueprint = WorkspaceBlueprint::new();
        assert_eq!(blueprint.crates(), &WORKSPACE_CRATES);
    }

    #[test]
    fn interface_kinds_remain_transport_specific() {
        assert_ne!(InterfaceKind::Cli, InterfaceKind::Mcp);
    }
}
