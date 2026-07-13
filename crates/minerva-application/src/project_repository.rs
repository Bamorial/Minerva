use minerva_domain::{MinervaError, Project};
use std::path::{Path, PathBuf};

pub trait ProjectRepository {
    fn locate_project_root(&self, start: &Path) -> Result<PathBuf, MinervaError>;

    fn is_initialized(&self, root: &Path) -> bool;

    fn load_project(&self, root: &Path) -> Result<Project, MinervaError>;

    fn save_project(&self, root: &Path, project: &Project) -> Result<(), MinervaError>;

    fn read_project_instructions(&self, root: &Path) -> Result<String, MinervaError>;

    fn write_project_instructions(
        &self,
        root: &Path,
        contents: &str,
    ) -> Result<(), MinervaError>;
}
