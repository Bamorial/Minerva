use minerva_application::BootstrapService;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct WorkspaceLayout {
    root: PathBuf,
}

impl WorkspaceLayout {
    #[must_use]
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self { root: root.as_ref().to_path_buf() }
    }

    #[must_use]
    pub fn crate_dir(&self, crate_name: &str) -> PathBuf {
        self.root.join("crates").join(crate_name)
    }

    #[must_use]
    pub fn expected_crate_dirs(&self) -> Vec<PathBuf> {
        BootstrapService::workspace_blueprint()
            .crates()
            .iter()
            .map(|name| self.crate_dir(name))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::WorkspaceLayout;

    #[test]
    fn layout_maps_workspace_members_to_crate_directories() {
        let layout = WorkspaceLayout::new("/tmp/minerva");
        let expected = layout.expected_crate_dirs();
        assert_eq!(expected.len(), 7);
        assert!(expected[0].ends_with("crates/minerva-domain"));
    }
}
