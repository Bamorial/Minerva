use crate::{
    CompleteTaskRequest, ProjectRepository, TaskCompletionResult, TaskRepository,
};
use minerva_domain::{
    DeclarationDocument, MinervaError, StatusKey, TaskTransitionService,
};
use std::{path::Path, time::SystemTime};

pub struct TaskCompletionService;

impl TaskCompletionService {
    pub fn complete(
        project_repo: &impl ProjectRepository,
        task_repo: &impl TaskRepository,
        root: &Path,
        request: CompleteTaskRequest,
    ) -> Result<TaskCompletionResult, MinervaError> {
        let project = project_repo.load_project(root)?;
        let task = task_repo.read_task(root, request.task_id)?;
        if task.version != request.version {
            return Err(MinervaError::VersionConflict {
                path: request.task_id.to_string(),
                expected: task.version.get().to_string(),
                actual: request.version.get().to_string(),
            });
        }
        if !request.allow_declaration_override {
            let declaration = task_repo.read_task_declaration(root, request.task_id)?;
            DeclarationDocument::validate_completion(&declaration)?;
        }
        let completed = TaskTransitionService::apply(
            &project,
            &task,
            StatusKey::new("completed").unwrap(),
            SystemTime::now(),
        )?;
        if !completed.changed {
            return Ok(TaskCompletionResult {
                task: completed.current.clone(),
                write_result: crate::TaskWriteResult {
                    previous_version: Some(completed.previous.version),
                    current_version: completed.current.version,
                    event_id: None,
                },
            });
        }
        let write_result = task_repo.transition_task(
            root,
            &completed.current,
            request.allow_declaration_override,
        )?;
        Ok(TaskCompletionResult { task: completed.current, write_result })
    }
}
