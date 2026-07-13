use crate::{
    MinervaLayout, ProjectLock, append_relationship_added_event,
    append_relationship_removed_event, read_relationships, task_catalog,
    write_relationships,
};
use minerva_domain::{
    MinervaError, Relationship, RelationshipId, validate_relationships,
};

pub fn create_relationship(
    layout: &MinervaLayout,
    relationship: &Relationship,
) -> Result<Relationship, MinervaError> {
    let _lock = ProjectLock::acquire(layout)?;
    let tasks = task_catalog::list_tasks(layout)?;
    let mut existing = read_relationships(layout, relationship.source_task)?;
    existing.push(relationship.clone());
    let mut graph = crate::relationship_catalog::list_relationships(layout)?;
    graph.push(relationship.clone());
    validate_relationships(&tasks, &graph)?;
    write_relationships(layout, relationship.source_task, &existing)?;
    append_relationship_added_event(layout, relationship)?;
    Ok(relationship.clone())
}

pub fn remove_relationship(
    layout: &MinervaLayout,
    relationship_id: RelationshipId,
) -> Result<Relationship, MinervaError> {
    let _lock = ProjectLock::acquire(layout)?;
    for task in task_catalog::list_tasks(layout)? {
        let mut relationships = read_relationships(layout, task.id)?;
        if let Some(index) =
            relationships.iter().position(|item| item.id == relationship_id)
        {
            let removed = relationships.remove(index);
            write_relationships(layout, task.id, &relationships)?;
            append_relationship_removed_event(layout, &removed)?;
            return Ok(removed);
        }
    }
    Err(MinervaError::InvalidConfiguration {
        key: "relationship_id".into(),
        reason: format!("relationship `{relationship_id}` was not found"),
    })
}
