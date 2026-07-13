use crate::{
    CreateTaskRequest, ProjectRepository, TaskCreateRecord, TaskCreationResult,
    TaskRepository, task_slug_builder::build_slug,
};
use minerva_domain::{
    ArchiveState, DeclarationActor, DeclarationDocument, DeclarationMetadata,
    MinervaError, Task, TaskFacts, TaskVersion,
};
use std::{path::Path, time::SystemTime};

pub struct TaskCreationService;

impl TaskCreationService {
    pub fn create(
        project_repo: &impl ProjectRepository,
        task_repo: &impl TaskRepository,
        root: &Path,
        request: CreateTaskRequest,
    ) -> Result<TaskCreationResult, MinervaError> {
        let CreateTaskRequest { title, task_type, parent_id, priority, tags } = request;
        let project = project_repo.load_project(root)?;
        let config = project_repo.load_project_config(root)?;
        let task_types = project_repo.load_task_types(root)?;
        let task_type = task_type.unwrap_or(project.default_task_type.clone());
        let definition = task_types
            .into_iter()
            .find(|item| item.name == task_type)
            .ok_or_else(|| unknown_type(&task_type.to_string()))?;
        if let Some(parent_id) = parent_id {
            task_repo.read_task(root, parent_id)?;
        }
        let now = SystemTime::now();
        let task = Task::new(Task {
            schema_version: 1,
            id: task_repo.next_task_id(root)?,
            slug: build_slug(&title).or_else(|| build_slug("task")),
            title,
            task_type: definition.name.clone(),
            status: project.default_status,
            parent_id,
            priority: priority.unwrap_or(config.default_priority),
            tags: tags.unwrap_or(config.default_tags),
            created_at: now,
            updated_at: now,
            completed_at: None,
            version: TaskVersion::initial(),
            declaration: DeclarationMetadata {
                version: 1,
                updated_at: now,
                updated_by: DeclarationActor::System,
                commit_hash: None,
            },
            facts: TaskFacts::default(),
            archive_state: ArchiveState::Active,
        })?;
        let record = TaskCreateRecord {
            task: task.clone(),
            instructions: definition.instruction_template,
            declaration: DeclarationDocument::template(),
        };
        let write_result = task_repo.create_task(root, &record)?;
        Ok(TaskCreationResult { task, write_result })
    }
}

fn unknown_type(value: &str) -> MinervaError {
    MinervaError::InvalidConfiguration {
        key: "task_type".into(),
        reason: format!("unknown task type `{value}`"),
    }
}
