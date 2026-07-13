use crate::{MinervaLayout, read_relationships, task_catalog};
use minerva_domain::{MinervaError, Relationship, TaskId};

pub fn list_relationships(
    layout: &MinervaLayout,
) -> Result<Vec<Relationship>, MinervaError> {
    task_catalog::list_tasks(layout)?.into_iter().try_fold(
        Vec::new(),
        |mut all, task| {
            all.extend(read_relationships(layout, task.id)?);
            Ok(all)
        },
    )
}

pub fn list_relationships_from(
    layout: &MinervaLayout,
    task_id: TaskId,
) -> Result<Vec<Relationship>, MinervaError> {
    task_catalog::resolve_task(layout, &task_id.to_string())?;
    read_relationships(layout, task_id)
}

pub fn list_relationships_to(
    layout: &MinervaLayout,
    task_id: TaskId,
) -> Result<Vec<Relationship>, MinervaError> {
    task_catalog::resolve_task(layout, &task_id.to_string())?;
    Ok(list_relationships(layout)?
        .into_iter()
        .filter(|item| item.target_task == task_id)
        .collect())
}
