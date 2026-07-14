use crate::{tree_filter, tree_row::TreeRow};
use minerva_application::{TaskShowResult, TaskTreeNode, TuiErrorMessage};
use minerva_domain::TaskId;
use std::{collections::BTreeSet, path::PathBuf};

#[derive(Debug, Clone)]
pub struct AppState {
    pub root: PathBuf,
    pub tree: Vec<TaskTreeNode>,
    pub rows: Vec<TreeRow>,
    pub expanded: BTreeSet<TaskId>,
    pub selected: usize,
    pub detail_scroll: u16,
    pub detail: Option<TaskShowResult>,
    pub error: Option<TuiErrorMessage>,
    pub show_archived: bool,
    pub search: String,
    pub search_mode: bool,
}

impl AppState {
    #[must_use]
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            tree: Vec::new(),
            rows: Vec::new(),
            expanded: BTreeSet::new(),
            selected: 0,
            detail_scroll: 0,
            detail: None,
            error: None,
            show_archived: false,
            search: String::new(),
            search_mode: false,
        }
    }

    pub fn set_tree(&mut self, tree: Vec<TaskTreeNode>) {
        if self.expanded.is_empty() {
            self.expanded.extend(tree.iter().map(|node| node.task.id));
        }
        self.tree = tree;
        self.refresh_rows(None);
    }

    pub fn refresh_rows(&mut self, selected: Option<TaskId>) {
        self.rows = tree_filter::rows(
            &self.tree,
            &self.expanded,
            &self.search,
            self.show_archived,
        );
        self.selected =
            tree_filter::selected_index(&self.rows, selected, self.selected);
    }

    #[must_use]
    pub fn selected_task_id(&self) -> Option<TaskId> {
        self.rows.get(self.selected).map(|row| row.task.id)
    }

    #[must_use]
    pub fn selected_task_ref(&self) -> Option<String> {
        self.selected_task_id().map(|id| id.to_string())
    }
}
