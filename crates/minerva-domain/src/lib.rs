mod error;
mod error_code;
mod error_detail;
mod error_details;

pub use error::MinervaError;
pub use error_code::ErrorCode;
pub use error_detail::{ErrorDetail, ErrorValue};

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
    use super::WorkspaceBlueprint;
    use super::{ErrorCode, ErrorValue, InterfaceKind, MinervaError, WORKSPACE_CRATES};

    #[test]
    fn blueprint_lists_all_workspace_crates() {
        let blueprint = WorkspaceBlueprint::new();
        assert_eq!(blueprint.crates(), &WORKSPACE_CRATES);
    }

    #[test]
    fn interface_kinds_remain_transport_specific() {
        assert_ne!(InterfaceKind::Cli, InterfaceKind::Mcp);
    }

    #[test]
    fn errors_expose_stable_codes_and_details() {
        let error = MinervaError::AmbiguousTaskReference {
            task_ref: "TSK-1".into(),
            matches: vec!["TSK-10".into(), "TSK-11".into()],
        };
        let details = error.details();
        assert_eq!(error.code(), ErrorCode::AmbiguousTaskReference);
        assert_eq!(error.code().as_str(), "ambiguous_task_reference");
        assert_eq!(details[0].key, "task_ref");
        assert_eq!(
            details[1].value,
            ErrorValue::List(vec!["TSK-10".into(), "TSK-11".into()])
        );
    }
}
