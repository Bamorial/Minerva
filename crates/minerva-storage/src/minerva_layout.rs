use minerva_domain::TaskId;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MinervaLayout {
    root: PathBuf,
}

impl MinervaLayout {
    #[must_use]
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self { root: root.as_ref().to_path_buf() }
    }

    #[must_use]
    pub fn minerva_dir(&self) -> PathBuf {
        self.root.join(".minerva")
    }
    #[must_use]
    pub fn project_file(&self) -> PathBuf {
        self.minerva_dir().join("project.yaml")
    }
    #[must_use]
    pub fn config_file(&self) -> PathBuf {
        self.minerva_dir().join("config.yaml")
    }
    #[must_use]
    pub fn instructions_file(&self) -> PathBuf {
        self.minerva_dir().join("instructions.md")
    }
    #[must_use]
    pub fn schema_version_file(&self) -> PathBuf {
        self.minerva_dir().join("schema-version")
    }
    #[must_use]
    pub fn task_types_dir(&self) -> PathBuf {
        self.minerva_dir().join("task-types")
    }
    #[must_use]
    pub fn tasks_dir(&self) -> PathBuf {
        self.minerva_dir().join("tasks")
    }
    #[must_use]
    pub fn indexes_dir(&self) -> PathBuf {
        self.minerva_dir().join("indexes")
    }
    #[must_use]
    pub fn contexts_dir(&self) -> PathBuf {
        self.minerva_dir().join("contexts")
    }
    #[must_use]
    pub fn sessions_dir(&self) -> PathBuf {
        self.minerva_dir().join("sessions")
    }
    #[must_use]
    pub fn locks_dir(&self) -> PathBuf {
        self.minerva_dir().join("locks")
    }
    #[must_use]
    pub fn task_dir(&self, task_id: TaskId) -> PathBuf {
        self.tasks_dir().join(task_id.to_string())
    }
    #[must_use]
    pub fn task_file(&self, task_id: TaskId) -> PathBuf {
        self.task_dir(task_id).join("task.yaml")
    }
    #[must_use]
    pub fn task_instructions_file(&self, task_id: TaskId) -> PathBuf {
        self.task_dir(task_id).join("instructions.md")
    }
    #[must_use]
    pub fn declaration_file(&self, task_id: TaskId) -> PathBuf {
        self.task_dir(task_id).join("declaration.md")
    }
    #[must_use]
    pub fn notes_file(&self, task_id: TaskId) -> PathBuf {
        self.task_dir(task_id).join("notes.md")
    }
    #[must_use]
    pub fn events_file(&self, task_id: TaskId) -> PathBuf {
        self.task_dir(task_id).join("events.jsonl")
    }
    #[must_use]
    pub fn task_lock_file(&self, task_id: TaskId) -> PathBuf {
        self.locks_dir().join(format!("{task_id}.lock"))
    }
}
