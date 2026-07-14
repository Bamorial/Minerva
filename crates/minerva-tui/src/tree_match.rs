use crate::tree_graph::Graph;
use minerva_domain::{ArchiveState, Task, TaskId};
use std::collections::BTreeSet;

pub fn visible(
    graph: &Graph,
    expanded: &BTreeSet<TaskId>,
    search: &str,
    show_archived: bool,
) -> BTreeSet<TaskId> {
    let direct = graph
        .tasks
        .values()
        .filter(|task| matches(task, search, show_archived))
        .map(|task| task.id)
        .collect::<Vec<_>>();
    let mut visible = BTreeSet::new();
    for task_id in direct {
        visible.insert(task_id);
        visible.extend(ancestors(graph, task_id));
        if search.trim().is_empty() {
            visible.extend(descendants(graph, task_id, expanded));
        }
    }
    visible
}

fn ancestors(graph: &Graph, task_id: TaskId) -> Vec<TaskId> {
    let mut ids = Vec::new();
    let mut current = graph.parent(task_id);
    while let Some(parent) = current {
        ids.push(parent);
        current = graph.parent(parent);
    }
    ids
}

fn descendants(
    graph: &Graph,
    task_id: TaskId,
    expanded: &BTreeSet<TaskId>,
) -> Vec<TaskId> {
    let mut ids = Vec::new();
    let mut stack =
        if expanded.contains(&task_id) { graph.children(task_id) } else { Vec::new() };
    while let Some(child) = stack.pop() {
        ids.push(child);
        if expanded.contains(&child) {
            stack.extend(graph.children(child));
        }
    }
    ids
}

fn matches(task: &Task, search: &str, show_archived: bool) -> bool {
    archive(task, show_archived) && query(task, search)
}

fn archive(task: &Task, show_archived: bool) -> bool {
    show_archived || task.archive_state == ArchiveState::Active
}

fn query(task: &Task, search: &str) -> bool {
    let query = search.trim().to_ascii_lowercase();
    query.is_empty()
        || task.id.to_string().to_ascii_lowercase().contains(&query)
        || task.title.to_ascii_lowercase().contains(&query)
        || task.status.as_str().contains(&query)
        || task.task_type.as_str().contains(&query)
        || task.tags.iter().any(|tag| tag.as_str().contains(&query))
}
