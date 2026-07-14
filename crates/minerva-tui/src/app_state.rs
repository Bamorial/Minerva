use crate::{
    app_command::AppCommand,
    app_confirm::ConfirmState,
    app_prompt::{PromptKind, PromptState},
    app_select::SelectState,
    tree_filter,
    tree_row::TreeRow,
};
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
    pub notice: Option<String>,
    pub show_archived: bool,
    pub search: String,
    pub search_mode: bool,
    pub statuses: Vec<String>,
    pub prompt: Option<PromptState>,
    pub select: Option<SelectState>,
    pub confirm: Option<ConfirmState>,
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
            notice: None,
            show_archived: false,
            search: String::new(),
            search_mode: false,
            statuses: Vec::new(),
            prompt: None,
            select: None,
            confirm: None,
        }
    }

    pub fn set_tree(&mut self, tree: Vec<TaskTreeNode>) {
        self.set_tree_with_selected(tree, None);
    }

    pub fn set_tree_with_selected(
        &mut self,
        tree: Vec<TaskTreeNode>,
        selected: Option<TaskId>,
    ) {
        if self.expanded.is_empty() {
            self.expanded.extend(tree.iter().map(|node| node.task.id));
        }
        self.tree = tree;
        self.refresh_rows(selected);
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

    pub fn set_statuses(&mut self, statuses: Vec<String>) {
        self.statuses = statuses;
    }

    pub fn begin_prompt(&mut self, kind: PromptKind) {
        self.clear_feedback();
        self.prompt = Some(PromptState::new(kind));
        self.select = None;
        self.confirm = None;
    }

    pub fn begin_status_select(&mut self, current: &str) {
        self.clear_feedback();
        let selected = self
            .statuses
            .iter()
            .position(|status| status == current)
            .unwrap_or_default();
        self.select = Some(SelectState::new("Status", self.statuses.clone(), selected));
        self.prompt = None;
        self.confirm = None;
    }

    pub fn confirm(&mut self, message: String, command: AppCommand) {
        self.clear_feedback();
        self.confirm = Some(ConfirmState { message, command });
        self.prompt = None;
        self.select = None;
    }

    pub fn cancel_action(&mut self) {
        self.prompt = None;
        self.select = None;
        self.confirm = None;
    }

    pub fn clear_feedback(&mut self) {
        self.error = None;
        self.notice = None;
    }
}
