use crate::{tree_graph::Graph, tree_match, tree_row::TreeRow};
use minerva_application::TaskTreeNode;
use minerva_domain::TaskId;
use std::collections::BTreeSet;

pub fn rows(
    tree: &[TaskTreeNode],
    expanded: &BTreeSet<TaskId>,
    search: &str,
    show_archived: bool,
) -> Vec<TreeRow> {
    let graph = Graph::build(tree);
    let visible = tree_match::visible(&graph, expanded, search, show_archived);
    let mut rows = Vec::new();
    let mut stack =
        graph.roots.iter().rev().map(|id| (*id, 0_usize)).collect::<Vec<_>>();
    while let Some((task_id, depth)) = stack.pop() {
        if !visible.contains(&task_id) {
            continue;
        }
        let children = graph.visible_children(task_id, &visible);
        let expanded = !search.trim().is_empty() || expanded.contains(&task_id);
        rows.push(TreeRow::new(
            &graph.tasks[&task_id],
            depth,
            graph.parent(task_id),
            !children.is_empty(),
            expanded,
        ));
        if expanded {
            stack.extend(children.into_iter().rev().map(|id| (id, depth + 1)));
        }
    }
    rows
}

pub fn selected_index(
    rows: &[TreeRow],
    selected: Option<TaskId>,
    fallback: usize,
) -> usize {
    selected
        .and_then(|task_id| rows.iter().position(|row| row.task.id == task_id))
        .unwrap_or_else(|| fallback.min(rows.len().saturating_sub(1)))
}
