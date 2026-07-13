use crate::new_prompt;
use minerva_application::TaskRepository;
use minerva_domain::{
    MinervaError, TaskId, TaskPriority, TaskTag, TaskTypeDefinition, TaskTypeKey,
};
use std::{collections::BTreeSet, path::Path};

pub fn task_type(
    value: Option<&str>,
    task_types: &[TaskTypeDefinition],
    default: &TaskTypeKey,
) -> Result<Option<TaskTypeKey>, MinervaError> {
    match value {
        Some(value) => value.parse().map(Some),
        None => new_prompt::choose_type(task_types, default),
    }
}

pub fn parent(
    task_repo: &impl TaskRepository,
    root: &Path,
    value: Option<&str>,
) -> Result<Option<TaskId>, MinervaError> {
    let Some(query) = value.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok(None);
    };
    query.parse::<TaskId>().map(Some).or_else(|_| search(task_repo, root, query))
}

pub fn priority(value: Option<&str>) -> Result<Option<TaskPriority>, MinervaError> {
    value.map(parse_priority).transpose()
}

pub fn tags(values: &[String]) -> Result<Option<BTreeSet<TaskTag>>, MinervaError> {
    (!values.is_empty())
        .then(|| values.iter().cloned().map(TaskTag::new).collect())
        .transpose()
}

pub fn tag_string(tag: TaskTag) -> String {
    serde_json::to_value(tag).unwrap().as_str().unwrap().to_string()
}

fn search(
    task_repo: &impl TaskRepository,
    root: &Path,
    query: &str,
) -> Result<Option<TaskId>, MinervaError> {
    let matches = task_repo.search_tasks(root, query)?;
    match matches.as_slice() {
        [] => Err(MinervaError::TaskNotFound { task_ref: query.into() }),
        [task] => Ok(Some(task.id)),
        _ => new_prompt::choose_parent(query, &matches).map(Some),
    }
}

fn parse_priority(value: &str) -> Result<TaskPriority, MinervaError> {
    match value.trim().to_ascii_lowercase().as_str() {
        "low" => Ok(TaskPriority::Low),
        "medium" => Ok(TaskPriority::Medium),
        "high" => Ok(TaskPriority::High),
        "urgent" => Ok(TaskPriority::Urgent),
        _ => Err(invalid("priority", "must be low, medium, high, or urgent")),
    }
}

fn invalid(key: &str, reason: &str) -> MinervaError {
    MinervaError::InvalidConfiguration { key: key.into(), reason: reason.into() }
}
