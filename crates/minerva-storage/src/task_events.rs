use crate::{
    MinervaLayout, task_event_actor::TaskEventActor, task_event_data::TaskEventData,
    task_event_record::TaskEventRecord,
};
use minerva_domain::{
    ArchiveState, DeclarationActor, EventId, MinervaError, Relationship, StatusKey,
    Task, TaskEventKind, TaskId,
};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    time::SystemTime,
};

pub fn append_created_event(
    layout: &MinervaLayout,
    task: &Task,
) -> Result<EventId, MinervaError> {
    append_event(
        layout,
        &TaskEventRecord::system(
            task,
            TaskEventKind::TaskCreated,
            TaskEventData::TaskCreated {
                version: task.version,
                parent_id: task.parent_id,
                status: task.status.clone(),
            },
        ),
    )
}

pub fn append_parent_changed_event(
    layout: &MinervaLayout,
    task: &Task,
    from_parent_id: Option<TaskId>,
) -> Result<EventId, MinervaError> {
    append_event(
        layout,
        &TaskEventRecord::system(
            task,
            TaskEventKind::TaskParentChanged,
            TaskEventData::TaskParentChanged {
                version: task.version,
                from_parent_id,
                to_parent_id: task.parent_id,
            },
        ),
    )
}

pub fn append_instructions_updated_event(
    layout: &MinervaLayout,
    task: &Task,
) -> Result<EventId, MinervaError> {
    append_event(
        layout,
        &TaskEventRecord::system(
            task,
            TaskEventKind::TaskInstructionsUpdated,
            TaskEventData::TaskInstructionsUpdated { version: task.version },
        ),
    )
}

pub fn append_declaration_updated_event(
    layout: &MinervaLayout,
    task: &Task,
    actor: DeclarationActor,
) -> Result<EventId, MinervaError> {
    append_event(
        layout,
        &TaskEventRecord::new(
            TaskEventActor::from(actor.clone()),
            TaskEventKind::TaskDeclarationUpdated,
            task.id,
            task.updated_at,
            TaskEventData::TaskDeclarationUpdated {
                version: task.version,
                declaration_version: task.declaration.version,
                updated_by: actor,
                commit_hash: task.declaration.commit_hash.clone(),
            },
        ),
    )
}

pub fn append_status_changed_event(
    layout: &MinervaLayout,
    task: &Task,
    from_status: StatusKey,
    completion_override: bool,
) -> Result<EventId, MinervaError> {
    append_event(
        layout,
        &TaskEventRecord::system(
            task,
            TaskEventKind::TaskStatusChanged,
            TaskEventData::TaskStatusChanged {
                version: task.version,
                from_status,
                to_status: task.status.clone(),
                completion_override,
            },
        ),
    )
}

pub fn append_relationship_added_event(
    layout: &MinervaLayout,
    relationship: &Relationship,
) -> Result<EventId, MinervaError> {
    append_event(
        layout,
        &TaskEventRecord::new(
            TaskEventActor::System,
            TaskEventKind::TaskRelationshipAdded,
            relationship.source_task,
            SystemTime::now(),
            TaskEventData::TaskRelationshipAdded { relationship: relationship.clone() },
        ),
    )
}

pub fn append_relationship_removed_event(
    layout: &MinervaLayout,
    relationship: &Relationship,
) -> Result<EventId, MinervaError> {
    append_event(
        layout,
        &TaskEventRecord::new(
            TaskEventActor::System,
            TaskEventKind::TaskRelationshipRemoved,
            relationship.source_task,
            SystemTime::now(),
            TaskEventData::TaskRelationshipRemoved {
                relationship: relationship.clone(),
            },
        ),
    )
}

pub fn append_archived_event(
    layout: &MinervaLayout,
    task: &Task,
    from_archive_state: ArchiveState,
) -> Result<EventId, MinervaError> {
    append_event(
        layout,
        &TaskEventRecord::system(
            task,
            TaskEventKind::TaskArchived,
            TaskEventData::TaskArchived {
                version: task.version,
                from_archive_state,
                to_archive_state: task.archive_state,
            },
        ),
    )
}

fn append_event(
    layout: &MinervaLayout,
    event: &TaskEventRecord,
) -> Result<EventId, MinervaError> {
    let path = layout.events_file(event.task_id);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| schema(&path, err))?;
    }
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|err| schema(&path, err))?;
    serde_json::to_writer(&mut file, event).map_err(|err| schema(&path, err))?;
    file.write_all(b"\n").map_err(|err| schema(&path, err))?;
    file.sync_data().map_err(|err| schema(&path, err))?;
    Ok(event.id)
}

fn schema(path: &std::path::Path, err: impl std::fmt::Display) -> MinervaError {
    MinervaError::SchemaError {
        path: path.display().to_string(),
        reason: err.to_string(),
    }
}
