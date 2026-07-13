use crate::{
    MinervaLayout, ProjectLock, TASK_TYPES, agents_md, atomic_replace, default_config,
    default_project, instructions_md, write_project, write_project_config,
    write_project_instructions,
};
use minerva_domain::{MinervaError, Project};
use std::{fs, path::Path};

pub fn initialize_project(root: &Path, force: bool) -> Result<Project, MinervaError> {
    let layout = MinervaLayout::new(root);
    check_targets(&layout, force)?;
    let _lock = ProjectLock::acquire(&layout)?;
    check_targets(&layout, force)?;
    create_dirs(&layout)?;
    let project = default_project(root);
    write_project(&layout, &project)?;
    write_project_config(&layout, &default_config())?;
    write_project_instructions(&layout, instructions_md())?;
    write_file(&layout.schema_version_file(), crate::SCHEMA_VERSION)?;
    write_agents_file(root)?;
    for (name, contents) in TASK_TYPES {
        write_file(&layout.task_types_dir().join(name), contents)?;
    }
    Ok(project)
}

fn check_targets(layout: &MinervaLayout, force: bool) -> Result<(), MinervaError> {
    if layout.project_file().is_file() && !force {
        return Err(MinervaError::ProjectAlreadyInitialized);
    }
    Ok(())
}

fn create_dirs(layout: &MinervaLayout) -> Result<(), MinervaError> {
    for path in dirs(layout) {
        fs::create_dir_all(&path).map_err(|err| schema(&path, err))?;
    }
    Ok(())
}

fn write_file(path: &Path, contents: &str) -> Result<(), MinervaError> {
    atomic_replace(path, contents.as_bytes()).map_err(|err| schema(path, err))
}

fn write_agents_file(root: &Path) -> Result<(), MinervaError> {
    let path = root.join("AGENTS.md");
    if path.exists() {
        return Ok(());
    }
    write_file(&path, agents_md())
}

fn schema(path: &Path, err: impl std::fmt::Display) -> MinervaError {
    MinervaError::SchemaError {
        path: path.display().to_string(),
        reason: err.to_string(),
    }
}

fn dirs(layout: &MinervaLayout) -> [std::path::PathBuf; 7] {
    [
        layout.task_types_dir(),
        layout.tasks_dir(),
        layout.indexes_dir(),
        layout.contexts_dir(),
        layout.sessions_dir(),
        layout.locks_dir(),
        layout.minerva_dir(),
    ]
}
