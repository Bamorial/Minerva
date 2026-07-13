use crate::{MinervaError, Project, StatusKey, Task};
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskTransitionOutcome {
    pub previous: Task,
    pub current: Task,
    pub changed: bool,
}

pub struct TaskTransitionService;

impl TaskTransitionService {
    pub fn apply(
        project: &Project,
        task: &Task,
        to: &StatusKey,
        at: SystemTime,
    ) -> Result<TaskTransitionOutcome, MinervaError> {
        if task.status == *to {
            return Ok(TaskTransitionOutcome {
                previous: task.clone(),
                current: task.clone(),
                changed: false,
            });
        }
        if !project.can_transition(&task.status, to) {
            return Err(MinervaError::InvalidStatusTransition {
                from: task.status.to_string(),
                to: to.to_string(),
            });
        }
        let current = Task::new(Task {
            status: to.clone(),
            updated_at: at,
            completed_at: completed_at(task, to, at),
            version: task.version.next(),
            ..task.clone()
        })?;
        current.validate_successor(task)?;
        Ok(TaskTransitionOutcome { previous: task.clone(), current, changed: true })
    }
}

fn completed_at(task: &Task, to: &StatusKey, at: SystemTime) -> Option<SystemTime> {
    if to.as_str() == "completed" {
        Some(at)
    } else if task.status.as_str() == "completed" {
        None
    } else {
        task.completed_at
    }
}
