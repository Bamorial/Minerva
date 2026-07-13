#![allow(dead_code)]

use minerva_application::{
    MoveTaskRequest, ProjectRepository, TaskCreateRecord, TaskRepository,
    TaskWriteResult,
};
use minerva_domain::{
    ArchiveState, DeclarationActor, DeclarationDocument, DeclarationFreshnessProbe,
    DeclarationMetadata, EventId, MinervaError, Project, ProjectConfig, Relationship,
    RelationshipId, Task, TaskId, TaskIdAllocator, TaskPriority, TaskSlug,
    TaskTypeDefinition, TaskTypeKey, TaskVersion,
};
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

pub struct FakeProjectRepo {
    pub project: Project,
    pub config: ProjectConfig,
    pub task_types: Vec<TaskTypeDefinition>,
}

impl ProjectRepository for FakeProjectRepo {
    fn locate_project_root(&self, _: &Path) -> Result<PathBuf, MinervaError> {
        unreachable!()
    }
    fn is_initialized(&self, _: &Path) -> bool {
        true
    }
    fn initialize_project(&self, _: &Path, _: bool) -> Result<Project, MinervaError> {
        unreachable!()
    }
    fn load_project(&self, _: &Path) -> Result<Project, MinervaError> {
        Ok(self.project.clone())
    }
    fn load_project_config(&self, _: &Path) -> Result<ProjectConfig, MinervaError> {
        Ok(self.config.clone())
    }
    fn load_task_types(
        &self,
        _: &Path,
    ) -> Result<Vec<TaskTypeDefinition>, MinervaError> {
        Ok(self.task_types.clone())
    }
    fn save_project(&self, _: &Path, _: &Project) -> Result<(), MinervaError> {
        unreachable!()
    }
    fn read_project_instructions(&self, _: &Path) -> Result<String, MinervaError> {
        unreachable!()
    }
    fn write_project_instructions(
        &self,
        _: &Path,
        _: &str,
    ) -> Result<(), MinervaError> {
        unreachable!()
    }
    fn prepare_project_instructions(&self, _: &Path) -> Result<PathBuf, MinervaError> {
        unreachable!()
    }
}

pub struct FakeTaskRepo {
    pub next_id: TaskId,
    pub tasks: Vec<Task>,
    pub created: RefCell<Option<TaskCreateRecord>>,
    pub moved: RefCell<Option<MoveTaskRequest>>,
}

impl FakeTaskRepo {
    pub fn new(last_id: u32, tasks: Vec<Task>) -> Self {
        let next_id = TaskIdAllocator::new(last_id).next_id();
        Self { next_id, tasks, created: RefCell::new(None), moved: RefCell::new(None) }
    }
}

impl TaskRepository for FakeTaskRepo {
    fn next_task_id(&self, _: &Path) -> Result<TaskId, MinervaError> {
        Ok(self.next_id)
    }
    fn create_task(
        &self,
        _: &Path,
        record: &TaskCreateRecord,
    ) -> Result<TaskWriteResult, MinervaError> {
        self.created.replace(Some(record.clone()));
        Ok(TaskWriteResult {
            previous_version: None,
            current_version: record.task.version,
            event_id: Some(EventId::new()),
        })
    }
    fn read_task(&self, _: &Path, task_id: TaskId) -> Result<Task, MinervaError> {
        self.tasks
            .iter()
            .find(|task| task.id == task_id)
            .cloned()
            .ok_or_else(|| MinervaError::TaskNotFound { task_ref: task_id.to_string() })
    }
    fn read_task_instructions(
        &self,
        _: &Path,
        _: TaskId,
    ) -> Result<String, MinervaError> {
        unreachable!()
    }
    fn read_task_declaration(
        &self,
        _: &Path,
        _: TaskId,
    ) -> Result<String, MinervaError> {
        Ok(DeclarationDocument::template())
    }
    fn read_declaration_freshness(
        &self,
        _: &Path,
        task_id: TaskId,
    ) -> Result<DeclarationFreshnessProbe, MinervaError> {
        let task = self.read_task(Path::new("."), task_id)?;
        Ok(DeclarationFreshnessProbe {
            declaration_updated_at: task.declaration.updated_at,
            task_updated_at: task.updated_at,
            instructions_updated_at: None,
            relationships_updated_at: None,
            covered_commit_hash: task.declaration.commit_hash,
            current_commit_hash: None,
        })
    }
    fn update_task(&self, _: &Path, _: &Task) -> Result<TaskWriteResult, MinervaError> {
        unreachable!()
    }
    fn update_task_instructions(
        &self,
        _: &Path,
        _: TaskId,
        _: TaskVersion,
        _: &str,
    ) -> Result<TaskWriteResult, MinervaError> {
        unreachable!()
    }
    fn update_task_declaration(
        &self,
        _: &Path,
        _: TaskId,
        _: TaskVersion,
        _: DeclarationActor,
        _: Option<String>,
        _: &str,
    ) -> Result<TaskWriteResult, MinervaError> {
        unreachable!()
    }
    fn list_tasks(&self, _: &Path) -> Result<Vec<Task>, MinervaError> {
        Ok(self.tasks.clone())
    }
    fn archive_task(
        &self,
        _: &Path,
        _: TaskId,
        _: TaskVersion,
    ) -> Result<TaskWriteResult, MinervaError> {
        unreachable!()
    }
    fn move_task(
        &self,
        _: &Path,
        task_id: TaskId,
        new_parent_id: Option<TaskId>,
        version: TaskVersion,
    ) -> Result<(Task, TaskWriteResult), MinervaError> {
        self.moved.replace(Some(MoveTaskRequest { task_id, new_parent_id, version }));
        let mut task = self.read_task(Path::new("."), task_id)?;
        task.parent_id = new_parent_id;
        task.version = task.version.next();
        Ok((
            task.clone(),
            TaskWriteResult {
                previous_version: Some(version),
                current_version: task.version,
                event_id: Some(EventId::new()),
            },
        ))
    }
    fn create_relationship(
        &self,
        _: &Path,
        _: &Relationship,
    ) -> Result<Relationship, MinervaError> {
        unreachable!()
    }
    fn remove_relationship(
        &self,
        _: &Path,
        _: RelationshipId,
    ) -> Result<Relationship, MinervaError> {
        unreachable!()
    }
    fn list_relationships(&self, _: &Path) -> Result<Vec<Relationship>, MinervaError> {
        Ok(Vec::new())
    }
    fn list_relationships_from(
        &self,
        _: &Path,
        _: TaskId,
    ) -> Result<Vec<Relationship>, MinervaError> {
        Ok(Vec::new())
    }
    fn list_relationships_to(
        &self,
        _: &Path,
        _: TaskId,
    ) -> Result<Vec<Relationship>, MinervaError> {
        Ok(Vec::new())
    }
    fn resolve_task(&self, _: &Path, _: &str) -> Result<Task, MinervaError> {
        unreachable!()
    }
    fn prepare_task_instructions(
        &self,
        _: &Path,
        _: TaskId,
    ) -> Result<PathBuf, MinervaError> {
        unreachable!()
    }
    fn prepare_task_declaration(
        &self,
        _: &Path,
        _: TaskId,
    ) -> Result<PathBuf, MinervaError> {
        unreachable!()
    }
    fn search_tasks(&self, _: &Path, _: &str) -> Result<Vec<Task>, MinervaError> {
        Ok(Vec::new())
    }
}

pub fn task(sequence: u32, title: &str) -> Task {
    let allocator = TaskIdAllocator::new(sequence - 1);
    Task::new(Task {
        schema_version: 1,
        id: allocator.next_id(),
        title: title.into(),
        slug: Some(TaskSlug::new(title.to_lowercase().replace(' ', "-")).unwrap()),
        task_type: TaskTypeKey::new("feature").unwrap(),
        status: minerva_domain::StatusKey::new("backlog").unwrap(),
        parent_id: None,
        priority: TaskPriority::Medium,
        tags: BTreeSet::new(),
        created_at: UNIX_EPOCH,
        updated_at: UNIX_EPOCH,
        completed_at: None,
        version: TaskVersion::initial(),
        declaration: DeclarationMetadata {
            version: 1,
            updated_at: UNIX_EPOCH,
            updated_by: DeclarationActor::Human,
            commit_hash: Some("abc123".into()),
        },
        archive_state: ArchiveState::Active,
    })
    .unwrap()
}
