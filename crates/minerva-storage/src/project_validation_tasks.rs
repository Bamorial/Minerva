use crate::{
    MinervaLayout, project_validation_task_data::TaskValidationData,
    read_relationships, read_task,
};
use minerva_application::ProjectValidationFinding;
use minerva_domain::{StatusKey, TaskId, TaskTypeKey};
use std::{collections::HashSet, fs};

pub fn validate_tasks(
    layout: &MinervaLayout,
    findings: &mut Vec<ProjectValidationFinding>,
    task_types: &HashSet<TaskTypeKey>,
    statuses: &HashSet<StatusKey>,
) -> TaskValidationData {
    task_ids(layout).into_iter().fold(
        TaskValidationData::new(),
        |mut state, task_id| {
            validate_task(layout, findings, task_types, statuses, task_id, &mut state);
            state
        },
    )
}

fn task_ids(layout: &MinervaLayout) -> Vec<TaskId> {
    fs::read_dir(layout.tasks_dir())
        .ok()
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .filter_map(parse_id)
        .collect()
}

fn parse_id(entry: fs::DirEntry) -> Option<TaskId> {
    entry
        .file_type()
        .ok()?
        .is_dir()
        .then_some(entry)?
        .file_name()
        .to_str()?
        .parse()
        .ok()
}

fn validate_task(
    layout: &MinervaLayout,
    findings: &mut Vec<ProjectValidationFinding>,
    task_types: &HashSet<TaskTypeKey>,
    statuses: &HashSet<StatusKey>,
    task_id: TaskId,
    state: &mut TaskValidationData,
) {
    let task_ref = task_id.to_string();
    crate::project_validation_task_files::validate_task_files(
        layout, findings, task_id, &task_ref,
    );
    validate_relationships(layout, findings, task_id, &task_ref, state);
    match read_task(layout, task_id) {
        Ok(task) => {
            crate::project_validation_task_rules::validate_task_model(
                findings, task_types, statuses, &task,
            );
            state.tasks.push(task);
        }
        Err(err) => findings.push(crate::project_validation_task_helpers::task_error(
            layout,
            task_id,
            "malformed_yaml",
            &task_ref,
            &err,
        )),
    }
}

fn validate_relationships(
    layout: &MinervaLayout,
    findings: &mut Vec<ProjectValidationFinding>,
    task_id: TaskId,
    task_ref: &str,
    state: &mut TaskValidationData,
) {
    match read_relationships(layout, task_id) {
        Ok(items) => state.relationships.extend(items),
        Err(err) => findings.push(crate::project_validation_task_helpers::task_error(
            layout,
            task_id,
            "malformed_yaml",
            task_ref,
            &err,
        )),
    }
}
