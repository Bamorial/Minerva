use crate::{
    MinervaLayout, initialize_project, instructions_md, read_project,
    read_project_config, read_project_instructions, read_task_types, write_project,
    write_project_instructions,
};
use minerva_application::ProjectRepository;
use minerva_domain::{MinervaError, Project, ProjectConfig, TaskTypeDefinition};
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FilesystemProjectRepository;

impl ProjectRepository for FilesystemProjectRepository {
    fn locate_project_root(&self, start: &Path) -> Result<PathBuf, MinervaError> {
        start
            .ancestors()
            .find(|path| MinervaLayout::new(path).project_file().is_file())
            .map(Path::to_path_buf)
            .ok_or(MinervaError::ProjectNotInitialized)
    }

    fn is_initialized(&self, root: &Path) -> bool {
        MinervaLayout::new(root).project_file().is_file()
    }

    fn initialize_project(
        &self,
        root: &Path,
        force: bool,
    ) -> Result<Project, MinervaError> {
        initialize_project(root, force)
    }

    fn load_project(&self, root: &Path) -> Result<Project, MinervaError> {
        read_project(&MinervaLayout::new(root))
    }

    fn load_project_config(&self, root: &Path) -> Result<ProjectConfig, MinervaError> {
        read_project_config(&MinervaLayout::new(root))
    }

    fn load_task_types(
        &self,
        root: &Path,
    ) -> Result<Vec<TaskTypeDefinition>, MinervaError> {
        read_task_types(&MinervaLayout::new(root))
    }

    fn save_project(&self, root: &Path, project: &Project) -> Result<(), MinervaError> {
        write_project(&MinervaLayout::new(root), project)
    }

    fn read_project_instructions(&self, root: &Path) -> Result<String, MinervaError> {
        read_project_instructions(&MinervaLayout::new(root))
    }

    fn write_project_instructions(
        &self,
        root: &Path,
        contents: &str,
    ) -> Result<(), MinervaError> {
        write_project_instructions(&MinervaLayout::new(root), contents)
    }

    fn prepare_project_instructions(
        &self,
        root: &Path,
    ) -> Result<PathBuf, MinervaError> {
        let layout = MinervaLayout::new(root);
        let path = layout.instructions_file();
        if !path.is_file() {
            write_project_instructions(&layout, instructions_md())?;
        }
        Ok(path)
    }
}
