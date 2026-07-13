use crate::{MoveTaskRequest, TaskMoveResult, TaskRepository};
use minerva_domain::MinervaError;
use std::path::Path;

pub struct TaskMovementService;

impl TaskMovementService {
    pub fn move_task(
        task_repo: &impl TaskRepository,
        root: &Path,
        request: &MoveTaskRequest,
    ) -> Result<TaskMoveResult, MinervaError> {
        if let Some(parent_id) = request.new_parent_id {
            task_repo.read_task(root, parent_id)?;
        }
        let task = task_repo.move_task(
            root,
            request.task_id,
            request.new_parent_id,
            request.version,
        )?;
        Ok(TaskMoveResult { task: task.0, write_result: task.1 })
    }
}
