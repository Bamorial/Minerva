use crate::{MinervaLayout, relationship_document::RelationshipDocument, yaml_codec};
use minerva_domain::{MinervaError, Relationship, TaskId};

pub fn read_relationships(
    layout: &MinervaLayout,
    task_id: TaskId,
) -> Result<Vec<Relationship>, MinervaError> {
    let path = layout.relationships_file(task_id);
    if !path.exists() {
        return Ok(Vec::new());
    }
    yaml_codec::read_yaml::<Vec<RelationshipDocument>>(&path)?
        .into_iter()
        .map(TryInto::try_into)
        .collect()
}

pub fn write_relationships(
    layout: &MinervaLayout,
    task_id: TaskId,
    relationships: &[Relationship],
) -> Result<(), MinervaError> {
    let path = layout.relationships_file(task_id);
    if relationships.is_empty() {
        if path.exists() {
            std::fs::remove_file(&path).map_err(|err| schema(&path, err))?;
        }
        return Ok(());
    }
    let docs: Vec<_> = relationships.iter().map(RelationshipDocument::from).collect();
    yaml_codec::write_yaml(&path, &docs)
}

fn schema(path: &std::path::Path, err: impl std::fmt::Display) -> MinervaError {
    MinervaError::SchemaError {
        path: path.display().to_string(),
        reason: err.to_string(),
    }
}
