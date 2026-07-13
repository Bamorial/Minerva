use crate::{MinervaError, Relationship, RelationshipType, Task};
use std::collections::HashSet;

pub fn validate_relationships(
    tasks: &[Task],
    relationships: &[Relationship],
) -> Result<(), MinervaError> {
    let task_ids = tasks.iter().map(|task| task.id).collect::<HashSet<_>>();
    let mut seen = HashSet::new();
    for relationship in relationships {
        relationship.validate()?;
        if relationship.relationship_type == RelationshipType::Parent {
            return invalid(
                "relationship_type",
                "parent links must be stored on tasks",
            );
        }
        if !task_ids.contains(&relationship.source_task) {
            return invalid("source_task", "must reference an existing task");
        }
        if !task_ids.contains(&relationship.target_task) {
            return invalid("target_task", "must reference an existing task");
        }
        let key = (
            relationship.source_task,
            relationship.target_task,
            relationship.relationship_type,
        );
        if !seen.insert(key) {
            return invalid("relationships", "contains a duplicate relationship");
        }
    }
    Ok(())
}

fn invalid<T>(key: &str, reason: &str) -> Result<T, MinervaError> {
    Err(MinervaError::InvalidConfiguration { key: key.into(), reason: reason.into() })
}
