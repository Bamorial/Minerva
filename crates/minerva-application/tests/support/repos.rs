use minerva_application::{
    ProjectRepository, TaskCreateRecord, TaskRepository, TaskWriteResult,
};
use minerva_domain::{
    EventId, MinervaError, Project, ProjectConfig, Task, TaskId, TaskTypeDefinition,
    TaskVersion,
};
use std::cell::RefCell;
use std::path::{Path, PathBuf};

pub struct FakeProjectRepo {
    pub project: Project,
    pub config: ProjectConfig,
    pub task_types: Vec<TaskTypeDefinition>,
}

impl ProjectRepository for FakeProjectRepo {
    fn locate_project_root(&self, _: &Path) -> Result<PathBuf, MinervaError> { unreachable!() }
    fn is_initialized(&self, _: &Path) -> bool { true }
    fn initialize_project(&self, _: &Path, _: bool) -> Result<Project, MinervaError> { unreachable!() }
    fn load_project(&self, _: &Path) -> Result<Project, MinervaError> { Ok(self.project.clone()) }
    fn load_project_config(&self, _: &Path) -> Result<ProjectConfig, MinervaError> { Ok(self.config.clone()) }
    fn load_task_types(&self, _: &Path) -> Result<Vec<TaskTypeDefinition>, MinervaError> { Ok(self.task_types.clone()) }
    fn save_project(&self, _: &Path, _: &Project) -> Result<(), MinervaError> { unreachable!() }
    fn read_project_instructions(&self, _: &Path) -> Result<String, MinervaError> { unreachable!() }
    fn write_project_instructions(&self, _: &Path, _: &str) -> Result<(), MinervaError> { unreachable!() }
}

pub struct FakeTaskRepo {
    pub next_id: TaskId,
    pub tasks: Vec<Task>,
    pub created: RefCell<Option<TaskCreateRecord>>,
}

impl TaskRepository for FakeTaskRepo {
    fn next_task_id(&self, _: &Path) -> Result<TaskId, MinervaError> { Ok(self.next_id) }
    fn create_task(&self, _: &Path, record: &TaskCreateRecord) -> Result<TaskWriteResult, MinervaError> {
        self.created.replace(Some(record.clone()));
        Ok(TaskWriteResult {
            previous_version: None,
            current_version: record.task.version,
            event_id: Some(EventId::new()),
        })
    }
    fn read_task(&self, _: &Path, task_id: TaskId) -> Result<Task, MinervaError> {
        self.tasks.iter().find(|task| task.id == task_id).cloned().ok_or_else(|| {
            MinervaError::TaskNotFound { task_ref: task_id.to_string() }
        })
    }
    fn update_task(&self, _: &Path, _: &Task) -> Result<TaskWriteResult, MinervaError> { unreachable!() }
    fn list_tasks(&self, _: &Path) -> Result<Vec<Task>, MinervaError> { Ok(self.tasks.clone()) }
    fn archive_task(&self, _: &Path, _: TaskId, _: TaskVersion) -> Result<TaskWriteResult, MinervaError> { unreachable!() }
    fn resolve_task(&self, _: &Path, _: &str) -> Result<Task, MinervaError> { unreachable!() }
    fn search_tasks(&self, _: &Path, _: &str) -> Result<Vec<Task>, MinervaError> { Ok(Vec::new()) }
}
