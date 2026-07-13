use minerva_application::{TaskCreateRecord, TaskRepository, TaskWriteResult};
use minerva_domain::{
    DeclarationActor, MinervaError, Relationship, RelationshipId, Task, TaskId,
    TaskVersion,
};
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FilesystemTaskRepository;

impl TaskRepository for FilesystemTaskRepository {
    fn next_task_id(&self, root: &Path) -> Result<TaskId, MinervaError> {
        crate::task_repository_queries::next_task_id(root)
    }
    fn create_task(
        &self,
        root: &Path,
        record: &TaskCreateRecord,
    ) -> Result<TaskWriteResult, MinervaError> {
        crate::task_repository_mutations::create_task(root, record)
    }
    fn read_task(&self, root: &Path, task_id: TaskId) -> Result<Task, MinervaError> {
        crate::task_repository_queries::read_task(root, task_id)
    }
    fn read_task_instructions(
        &self,
        root: &Path,
        task_id: TaskId,
    ) -> Result<String, MinervaError> {
        crate::task_repository_queries::read_task_instructions(root, task_id)
    }
    fn read_task_declaration(
        &self,
        root: &Path,
        task_id: TaskId,
    ) -> Result<String, MinervaError> {
        crate::task_repository_queries::read_task_declaration(root, task_id)
    }
    fn update_task(
        &self,
        root: &Path,
        task: &Task,
    ) -> Result<TaskWriteResult, MinervaError> {
        crate::task_repository_mutations::update_task(root, task)
    }
    fn update_task_instructions(
        &self,
        root: &Path,
        task_id: TaskId,
        version: TaskVersion,
        contents: &str,
    ) -> Result<TaskWriteResult, MinervaError> {
        crate::task_repository_mutations::update_task_instructions(
            root, task_id, version, contents,
        )
    }
    fn update_task_declaration(
        &self,
        root: &Path,
        task_id: TaskId,
        version: TaskVersion,
        actor: DeclarationActor,
        commit_hash: Option<String>,
        contents: &str,
    ) -> Result<TaskWriteResult, MinervaError> {
        crate::task_repository_mutations::update_task_declaration(
            root,
            task_id,
            version,
            actor,
            commit_hash,
            contents,
        )
    }
    fn list_tasks(&self, root: &Path) -> Result<Vec<Task>, MinervaError> {
        crate::task_repository_queries::list_tasks(root)
    }
    fn archive_task(
        &self,
        root: &Path,
        task_id: TaskId,
        version: TaskVersion,
    ) -> Result<TaskWriteResult, MinervaError> {
        crate::task_repository_mutations::archive_task(root, task_id, version)
    }
    fn move_task(
        &self,
        root: &Path,
        task_id: TaskId,
        new_parent_id: Option<TaskId>,
        version: TaskVersion,
    ) -> Result<(Task, TaskWriteResult), MinervaError> {
        crate::task_repository_mutations::move_task(
            root,
            task_id,
            new_parent_id,
            version,
        )
    }
    fn create_relationship(
        &self,
        root: &Path,
        relationship: &Relationship,
    ) -> Result<Relationship, MinervaError> {
        crate::task_repository_mutations::create_relationship(root, relationship)
    }
    fn remove_relationship(
        &self,
        root: &Path,
        relationship_id: RelationshipId,
    ) -> Result<Relationship, MinervaError> {
        crate::task_repository_mutations::remove_relationship(root, relationship_id)
    }
    fn list_relationships(
        &self,
        root: &Path,
    ) -> Result<Vec<Relationship>, MinervaError> {
        crate::task_repository_queries::list_relationships(root)
    }
    fn list_relationships_from(
        &self,
        root: &Path,
        task_id: TaskId,
    ) -> Result<Vec<Relationship>, MinervaError> {
        crate::task_repository_queries::list_relationships_from(root, task_id)
    }
    fn list_relationships_to(
        &self,
        root: &Path,
        task_id: TaskId,
    ) -> Result<Vec<Relationship>, MinervaError> {
        crate::task_repository_queries::list_relationships_to(root, task_id)
    }
    fn resolve_task(&self, root: &Path, task_ref: &str) -> Result<Task, MinervaError> {
        crate::task_repository_queries::resolve_task(root, task_ref)
    }
    fn prepare_task_instructions(
        &self,
        root: &Path,
        task_id: TaskId,
    ) -> Result<PathBuf, MinervaError> {
        crate::task_repository_mutations::prepare_task_instructions(root, task_id)
    }
    fn prepare_task_declaration(
        &self,
        root: &Path,
        task_id: TaskId,
    ) -> Result<PathBuf, MinervaError> {
        crate::task_repository_mutations::prepare_task_declaration(root, task_id)
    }
    fn search_tasks(
        &self,
        root: &Path,
        query: &str,
    ) -> Result<Vec<Task>, MinervaError> {
        crate::task_repository_queries::search_tasks(root, query)
    }
}
