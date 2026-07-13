use crate::TaskRepository;
use minerva_domain::{MinervaError, Relationship, RelationshipType, TaskId};
use std::{path::Path, time::SystemTime};

pub struct TaskRelationshipService;

impl TaskRelationshipService {
    pub fn create(
        task_repo: &impl TaskRepository,
        root: &Path,
        source_task: TaskId,
        target_task: TaskId,
        relationship_type: RelationshipType,
        reason: Option<String>,
    ) -> Result<Relationship, MinervaError> {
        task_repo.create_relationship(
            root,
            &Relationship::new(Relationship {
                schema_version: 1,
                id: minerva_domain::RelationshipId::new(),
                source_task,
                target_task,
                relationship_type,
                reason: normalize(reason),
                created_at: SystemTime::now(),
            })?,
        )
    }

    pub fn remove(
        task_repo: &impl TaskRepository,
        root: &Path,
        source_task: TaskId,
        target_task: TaskId,
        relationship_type: RelationshipType,
    ) -> Result<Relationship, MinervaError> {
        let key = relationship_type.semantic_key(source_task, target_task);
        let relationship = task_repo
            .list_relationships(root)?
            .into_iter()
            .find(|item| item.semantic_key() == key)
            .ok_or_else(|| missing(source_task, target_task, relationship_type))?;
        task_repo.remove_relationship(root, relationship.id)
    }
}

fn normalize(reason: Option<String>) -> Option<String> {
    reason.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

fn missing(
    source_task: TaskId,
    target_task: TaskId,
    relationship_type: RelationshipType,
) -> MinervaError {
    MinervaError::InvalidConfiguration {
        key: "relationships".into(),
        reason: format!(
            "relationship `{relationship_type:?}` between `{source_task}` and `{target_task}` was not found"
        ),
    }
}
