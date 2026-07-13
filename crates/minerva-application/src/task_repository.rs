use crate::TaskCreateRecord;
use minerva_domain::{
    EventId, MinervaError, Relationship, RelationshipId, Task, TaskId, TaskVersion,
};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaskWriteResult {
    pub previous_version: Option<TaskVersion>,
    pub current_version: TaskVersion,
    pub event_id: Option<EventId>,
}

pub trait TaskRepository {
    fn next_task_id(&self, root: &Path) -> Result<TaskId, MinervaError>;
    fn create_task(
        &self,
        root: &Path,
        record: &TaskCreateRecord,
    ) -> Result<TaskWriteResult, MinervaError>;
    fn read_task(&self, root: &Path, task_id: TaskId) -> Result<Task, MinervaError>;
    fn update_task(
        &self,
        root: &Path,
        task: &Task,
    ) -> Result<TaskWriteResult, MinervaError>;
    fn list_tasks(&self, root: &Path) -> Result<Vec<Task>, MinervaError>;
    fn archive_task(
        &self,
        root: &Path,
        task_id: TaskId,
        version: TaskVersion,
    ) -> Result<TaskWriteResult, MinervaError>;
    fn move_task(
        &self,
        root: &Path,
        task_id: TaskId,
        new_parent_id: Option<TaskId>,
        version: TaskVersion,
    ) -> Result<(Task, TaskWriteResult), MinervaError>;
    fn create_relationship(
        &self,
        root: &Path,
        relationship: &Relationship,
    ) -> Result<Relationship, MinervaError>;
    fn remove_relationship(
        &self,
        root: &Path,
        relationship_id: RelationshipId,
    ) -> Result<Relationship, MinervaError>;
    fn list_relationships(
        &self,
        root: &Path,
    ) -> Result<Vec<Relationship>, MinervaError>;
    fn list_relationships_from(
        &self,
        root: &Path,
        task_id: TaskId,
    ) -> Result<Vec<Relationship>, MinervaError>;
    fn list_relationships_to(
        &self,
        root: &Path,
        task_id: TaskId,
    ) -> Result<Vec<Relationship>, MinervaError>;
    fn resolve_task(&self, root: &Path, task_ref: &str) -> Result<Task, MinervaError>;
    fn search_tasks(&self, root: &Path, query: &str)
    -> Result<Vec<Task>, MinervaError>;
}
