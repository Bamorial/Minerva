use crate::{
    CompleteTaskRequest, ProjectRepository, TaskCompletionResult, TaskRepository,
    TaskStatusService,
};
use minerva_domain::{DeclarationDocument, MinervaError, StatusKey};
use std::path::Path;

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
        let result = TaskStatusService::apply(
            &project,
            task_repo,
            root,
            &task,
            &StatusKey::new("completed").unwrap(),
            request.allow_declaration_override,
        )?;
        Ok(TaskCompletionResult {
            task: result.task,
            write_result: result.write_result,
        })
    }
}
