use crate::{MinervaError, Relationship, RelationshipType, Task, TaskId};
use std::collections::{HashMap, HashSet};

pub fn validate_relationships(
    tasks: &[Task],
    relationships: &[Relationship],
) -> Result<(), MinervaError> {
    let task_ids = tasks.iter().map(|task| task.id).collect::<HashSet<_>>();
    let mut seen = HashSet::new();
    let mut dependencies = HashMap::new();
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
        if !seen.insert(relationship.semantic_key()) {
            return invalid("relationships", "contains a duplicate relationship");
        }
        if let Some((source, target)) = relationship
            .relationship_type
            .dependency_edge(relationship.source_task, relationship.target_task)
        {
            dependencies.entry(source).or_insert_with(Vec::new).push(target);
        }
    }
    for task in dependencies.keys() {
        let mut visiting = HashSet::new();
        let mut visited = HashSet::new();
        detect_cycle(*task, *task, &dependencies, &mut visiting, &mut visited)?;
    }
    Ok(())
}

fn detect_cycle(
    root: TaskId,
    current: TaskId,
    dependencies: &HashMap<TaskId, Vec<TaskId>>,
    visiting: &mut HashSet<TaskId>,
    visited: &mut HashSet<TaskId>,
) -> Result<(), MinervaError> {
    if !visiting.insert(current) || !visited.insert(current) {
        return Ok(());
    }
    if let Some(targets) = dependencies.get(&current) {
        for target in targets {
            if *target == root {
                return Err(MinervaError::DependencyCycle {
                    task: root.to_string(),
                    depends_on: current.to_string(),
                });
            }
            detect_cycle(root, *target, dependencies, visiting, visited)?;
        }
    }
    visiting.remove(&current);
    Ok(())
}

fn invalid<T>(key: &str, reason: &str) -> Result<T, MinervaError> {
    Err(MinervaError::InvalidConfiguration { key: key.into(), reason: reason.into() })
}
