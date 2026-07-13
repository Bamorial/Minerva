use minerva_domain::{MinervaError, Project, ProjectConfig, TaskTypeDefinition};
use std::path::{Path, PathBuf};

pub trait ProjectRepository {
    fn locate_project_root(&self, start: &Path) -> Result<PathBuf, MinervaError>;

    fn is_initialized(&self, root: &Path) -> bool;

    fn initialize_project(
        &self,
        root: &Path,
        force: bool,
    ) -> Result<Project, MinervaError>;

    fn load_project(&self, root: &Path) -> Result<Project, MinervaError>;

    fn load_project_config(&self, root: &Path) -> Result<ProjectConfig, MinervaError>;

    fn load_task_types(
        &self,
        root: &Path,
    ) -> Result<Vec<TaskTypeDefinition>, MinervaError>;

    fn save_project(&self, root: &Path, project: &Project) -> Result<(), MinervaError>;

    fn read_project_instructions(&self, root: &Path) -> Result<String, MinervaError>;

    fn write_project_instructions(
        &self,
        root: &Path,
        contents: &str,
    ) -> Result<(), MinervaError>;

    fn prepare_project_instructions(
        &self,
        root: &Path,
    ) -> Result<PathBuf, MinervaError>;
}
