use crate::MinervaLayout;
use minerva_application::{
    ProjectValidationResult, RebuildResult, TaskCreateRecord, TaskLogEvent,
    TaskLogIssue, TaskLogReadResult, TaskRepository, TaskWriteResult,
};
use minerva_domain::{
    DeclarationActor, DeclarationFreshnessProbe, MinervaError, Relationship,
    RelationshipId, Task, TaskId, TaskVersion,
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
    fn read_declaration_freshness(
        &self,
        root: &Path,
        task_id: TaskId,
    ) -> Result<DeclarationFreshnessProbe, MinervaError> {
        crate::task_freshness::read_declaration_freshness(
            &MinervaLayout::new(root),
            task_id,
        )
    }
    fn read_task_log(
        &self,
        root: &Path,
        task_id: TaskId,
    ) -> Result<TaskLogReadResult, MinervaError> {
        let log = crate::read_task_event_log(&MinervaLayout::new(root), task_id)?;
        Ok(TaskLogReadResult {
            events: log.events.into_iter().map(map_event).collect(),
            issues: log.issues.into_iter().map(map_issue).collect(),
        })
    }
    fn update_task(
        &self,
        root: &Path,
        task: &Task,
    ) -> Result<TaskWriteResult, MinervaError> {
        crate::task_repository_mutations::update_task(root, task)
    }
    fn transition_task(
        &self,
        root: &Path,
        task: &Task,
        completion_override: bool,
    ) -> Result<TaskWriteResult, MinervaError> {
        crate::task_repository_mutations::transition_task(
            root,
            task,
            completion_override,
        )
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
            &actor,
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
    fn rebuild_derived_state(
        &self,
        root: &Path,
        dry_run: bool,
    ) -> Result<RebuildResult, MinervaError> {
        crate::task_index_rebuild::rebuild_task_index(
            &MinervaLayout::new(root),
            dry_run,
        )
    }
    fn validate_project_state(
        &self,
        root: &Path,
    ) -> Result<ProjectValidationResult, MinervaError> {
        crate::project_validation::validate_project_state(root)
    }
}

fn map_event(event: crate::task_event_record::TaskEventRecord) -> TaskLogEvent {
    TaskLogEvent {
        id: event.id.to_string(),
        recorded_at: event.recorded_at,
        actor: actor(event.actor),
        kind: event.kind,
        details: details(event.data),
    }
}

fn map_issue(issue: crate::task_event_log::TaskEventLogIssue) -> TaskLogIssue {
    TaskLogIssue { line: issue.line, reason: issue.reason }
}

fn actor(actor: crate::task_event_actor::TaskEventActor) -> String {
    match actor {
        crate::task_event_actor::TaskEventActor::Human => "human".into(),
        crate::task_event_actor::TaskEventActor::System => "system".into(),
        crate::task_event_actor::TaskEventActor::Agent { name } => {
            format!("agent:{name}")
        }
    }
}

fn details(data: crate::task_event_data::TaskEventData) -> Vec<String> {
    use crate::task_event_data::TaskEventData::{
        TaskArchived, TaskCreated, TaskDeclarationUpdated, TaskInstructionsUpdated,
        TaskParentChanged, TaskRelationshipAdded, TaskRelationshipRemoved,
        TaskStatusChanged,
    };
    match data {
        TaskCreated { version, parent_id, status } => vec![
            format!("version={}", version.get()),
            format!("status={status}"),
            format!(
                "parent={}",
                parent_id.map_or_else(|| "none".into(), |id| id.to_string())
            ),
        ],
        TaskDeclarationUpdated {
            version,
            declaration_version,
            updated_by,
            commit_hash,
        } => vec![
            format!("version={}", version.get()),
            format!("declaration_version={declaration_version}"),
            format!("updated_by={updated_by:?}"),
            format!("commit={}", commit_hash.unwrap_or_else(|| "none".into())),
        ],
        TaskInstructionsUpdated { version } => {
            vec![format!("version={}", version.get())]
        }
        TaskParentChanged { version, from_parent_id, to_parent_id } => vec![
            format!("version={}", version.get()),
            format!(
                "from={}",
                from_parent_id.map_or_else(|| "none".into(), |id| id.to_string())
            ),
            format!(
                "to={}",
                to_parent_id.map_or_else(|| "none".into(), |id| id.to_string())
            ),
        ],
        TaskStatusChanged { version, from_status, to_status, completion_override } => {
            vec![
                format!("version={}", version.get()),
                format!("from={from_status}"),
                format!("to={to_status}"),
                format!("completion_override={completion_override}"),
            ]
        }
        TaskRelationshipAdded { relationship }
        | TaskRelationshipRemoved { relationship } => vec![
            format!(
                "relationship={}",
                relationship_type(relationship.relationship_type)
            ),
            format!("source={}", relationship.source_task),
            format!("target={}", relationship.target_task),
            format!("reason={}", relationship.reason.unwrap_or_else(|| "none".into())),
        ],
        TaskArchived { version, from_archive_state, to_archive_state } => vec![
            format!("version={}", version.get()),
            format!("from={}", archive_state(from_archive_state)),
            format!("to={}", archive_state(to_archive_state)),
        ],
    }
}

fn relationship_type(value: minerva_domain::RelationshipType) -> &'static str {
    match value {
        minerva_domain::RelationshipType::Parent => "parent",
        minerva_domain::RelationshipType::DependsOn => "depends-on",
        minerva_domain::RelationshipType::Blocks => "blocks",
        minerva_domain::RelationshipType::RelatedTo => "related-to",
        minerva_domain::RelationshipType::Duplicates => "duplicates",
        minerva_domain::RelationshipType::Implements => "implements",
        minerva_domain::RelationshipType::References => "references",
    }
}

fn archive_state(value: minerva_domain::ArchiveState) -> &'static str {
    match value {
        minerva_domain::ArchiveState::Active => "active",
        minerva_domain::ArchiveState::Archived => "archived",
    }
}
