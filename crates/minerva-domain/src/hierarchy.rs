use crate::{MinervaError, Task, TaskId};
use std::collections::BTreeMap;

pub fn validate_task_hierarchy(tasks: &[Task]) -> Result<(), MinervaError> {
    let tasks = tasks.iter().map(|task| (task.id, task)).collect::<BTreeMap<_, _>>();
    let mut state = BTreeMap::new();
    for id in tasks.keys().copied() {
        visit(id, &tasks, &mut state)?;
    }
    Ok(())
}

fn visit(
    id: TaskId,
    tasks: &BTreeMap<TaskId, &Task>,
    state: &mut BTreeMap<TaskId, Visit>,
) -> Result<(), MinervaError> {
    match state.get(&id) {
        Some(Visit::Done | Visit::Active) => return Ok(()),
        None => {}
    }
    state.insert(id, Visit::Active);
    if let Some(parent_id) = tasks[&id].parent_id {
        if state.get(&parent_id) == Some(&Visit::Active) {
            return Err(MinervaError::HierarchyCycle {
                parent: parent_id.to_string(),
                child: id.to_string(),
            });
        }
        if !tasks.contains_key(&parent_id) {
            return Err(MinervaError::TaskNotFound { task_ref: parent_id.to_string() });
        }
        visit(parent_id, tasks, state)?;
    }
    state.insert(id, Visit::Done);
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Visit {
    Active,
    Done,
}
