use crate::{
    app_command::AppCommand,
    app_confirm::ConfirmState,
    app_prompt::{PromptKind, PromptState},
    app_select::SelectState,
    tree_filter,
    tree_row::TreeRow,
};
use minerva_application::{TaskShowResult, TaskTreeNode, TuiErrorMessage};
use minerva_domain::{RelationshipType, TaskId};
use std::{collections::BTreeSet, path::PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusPane {
    CurrentView,
    Tree,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentView {
    Details,
    Context,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PendingSequence {
    CreateOrNextTask,
    PreviousTask,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateField {
    Title,
    TaskType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateModal {
    pub title: String,
    pub task_types: Vec<String>,
    pub selected_type: usize,
    pub parent_id: Option<TaskId>,
    pub field: CreateField,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkField {
    Query,
    Relationship,
    Results,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkCandidate {
    pub task_id: TaskId,
    pub task_ref: String,
    pub title: String,
    pub depth: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkModal {
    pub query: String,
    pub relationship_type: RelationshipType,
    pub field: LinkField,
    pub candidates: Vec<LinkCandidate>,
    pub selected: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteModal {
    pub task_ref: String,
    pub title: String,
    pub descendants: usize,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub root: PathBuf,
    pub tree: Vec<TaskTreeNode>,
    pub rows: Vec<TreeRow>,
    pub expanded: BTreeSet<TaskId>,
    pub selected: usize,
    pub detail_scroll: u16,
    pub detail: Option<TaskShowResult>,
    pub context: Option<String>,
    pub error: Option<TuiErrorMessage>,
    pub notice: Option<String>,
    pub search: String,
    pub search_mode: bool,
    pub statuses: Vec<String>,
    pub task_types: Vec<String>,
    pub prompt: Option<PromptState>,
    pub select: Option<SelectState>,
    pub confirm: Option<ConfirmState>,
    pub focus: FocusPane,
    pub current_view: CurrentView,
    pub count_buffer: String,
    pub pending_sequence: Option<PendingSequence>,
    pub create: Option<CreateModal>,
    pub link: Option<LinkModal>,
    pub delete: Option<DeleteModal>,
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
            context: None,
            error: None,
            notice: None,
            search: String::new(),
            search_mode: false,
            statuses: Vec::new(),
            task_types: Vec::new(),
            prompt: None,
            select: None,
            confirm: None,
            focus: FocusPane::Tree,
            current_view: CurrentView::Details,
            count_buffer: String::new(),
            pending_sequence: None,
            create: None,
            link: None,
            delete: None,
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
        self.refresh_link_candidates();
    }

    pub fn refresh_rows(&mut self, selected: Option<TaskId>) {
        self.rows = tree_filter::rows(&self.tree, &self.expanded, &self.search, false);
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

    pub fn set_task_types(&mut self, task_types: Vec<String>) {
        self.task_types = task_types;
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

    pub fn begin_create(&mut self, parent_id: Option<TaskId>) {
        self.clear_feedback();
        let selected_type =
            self.task_types.iter().position(|item| item == "feature").unwrap_or(0);
        self.create = Some(CreateModal {
            title: String::new(),
            task_types: self.task_types.clone(),
            selected_type,
            parent_id,
            field: CreateField::Title,
        });
    }

    pub fn begin_link(&mut self) {
        self.clear_feedback();
        self.link = Some(LinkModal {
            query: String::new(),
            relationship_type: RelationshipType::DependsOn,
            field: LinkField::Query,
            candidates: Vec::new(),
            selected: 0,
        });
        self.refresh_link_candidates();
    }

    pub fn begin_delete(&mut self) {
        let Some(row) = self.rows.get(self.selected) else {
            return;
        };
        let task_id = row.task.id;
        let title = row.task.title.clone();
        self.clear_feedback();
        let descendants = self
            .tree_candidates()
            .into_iter()
            .filter(|candidate| is_descendant(&self.tree, task_id, candidate.task_id))
            .count();
        self.delete =
            Some(DeleteModal { task_ref: task_id.to_string(), title, descendants });
    }

    pub fn show_details(&mut self) {
        self.focus = FocusPane::CurrentView;
        self.current_view = CurrentView::Details;
        self.reset_detail_scroll();
    }

    pub fn show_context(&mut self, context: String) {
        self.focus = FocusPane::CurrentView;
        self.current_view = CurrentView::Context;
        self.context = Some(context);
        self.reset_detail_scroll();
    }

    pub fn focus_tree(&mut self) {
        self.focus = FocusPane::Tree;
        self.clear_count();
    }

    pub fn focus_current(&mut self) {
        self.focus = FocusPane::CurrentView;
        self.clear_count();
    }

    pub fn cancel_action(&mut self) {
        self.prompt = None;
        self.select = None;
        self.confirm = None;
        self.create = None;
        self.link = None;
        self.delete = None;
        self.clear_count();
        self.pending_sequence = None;
    }

    pub fn clear_feedback(&mut self) {
        self.error = None;
        self.notice = None;
    }

    pub fn push_count(&mut self, value: char) {
        self.count_buffer.push(value);
    }

    pub fn take_count(&mut self) -> usize {
        let count = self.count_buffer.parse::<usize>().ok().filter(|value| *value > 0);
        self.clear_count();
        count.unwrap_or(1)
    }

    pub fn clear_count(&mut self) {
        self.count_buffer.clear();
    }

    pub fn refresh_link_candidates(&mut self) {
        let selected = self.selected_task_id();
        let items = self.tree_candidates();
        let Some(link) = self.link.as_mut() else {
            return;
        };
        let query = link.query.trim().to_ascii_lowercase();
        link.candidates = items
            .into_iter()
            .filter(|candidate| Some(candidate.task_id) != selected)
            .filter(|candidate| {
                query.is_empty()
                    || candidate.task_ref.to_ascii_lowercase().contains(&query)
                    || candidate.title.to_ascii_lowercase().contains(&query)
            })
            .collect();
        link.selected = link.selected.min(link.candidates.len().saturating_sub(1));
    }

    #[must_use]
    pub fn tree_candidates(&self) -> Vec<LinkCandidate> {
        let mut items = Vec::new();
        flatten(&self.tree, 0, &mut items);
        items
    }
}

fn flatten(tree: &[TaskTreeNode], depth: usize, items: &mut Vec<LinkCandidate>) {
    for node in tree {
        items.push(LinkCandidate {
            task_id: node.task.id,
            task_ref: node.task.id.to_string(),
            title: node.task.title.clone(),
            depth,
        });
        flatten(&node.children, depth + 1, items);
    }
}

fn is_descendant(tree: &[TaskTreeNode], root_id: TaskId, candidate_id: TaskId) -> bool {
    tree.iter().any(|node| contains(node, root_id, candidate_id))
}

fn contains(node: &TaskTreeNode, root_id: TaskId, candidate_id: TaskId) -> bool {
    if node.task.id == root_id {
        return subtree_contains(node, candidate_id);
    }
    node.children.iter().any(|child| contains(child, root_id, candidate_id))
}

fn subtree_contains(node: &TaskTreeNode, candidate_id: TaskId) -> bool {
    node.task.id == candidate_id
        || node.children.iter().any(|child| subtree_contains(child, candidate_id))
}
