use crate::{MinervaLayout, task_catalog};
use minerva_domain::{MinervaError, Task, validate_task_hierarchy};

pub fn validate_write(layout: &MinervaLayout, task: &Task) -> Result<(), MinervaError> {
    let mut tasks = task_catalog::list_tasks(layout)?;
    match tasks.iter_mut().find(|item| item.id == task.id) {
        Some(current) => *current = task.clone(),
        None => tasks.push(task.clone()),
    }
    validate_task_hierarchy(&tasks)
}
