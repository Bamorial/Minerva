use crate::{AppState, app_state::FocusPane};

impl AppState {
    pub fn select_next(&mut self, count: usize) {
        if self.focus == FocusPane::CurrentView {
            self.detail_scroll =
                self.detail_scroll.saturating_add(count.min(u16::MAX as usize) as u16);
            return;
        }
        self.selected = (self.selected + count).min(self.rows.len().saturating_sub(1));
        self.reset_detail_scroll();
    }

    pub fn select_previous(&mut self, count: usize) {
        if self.focus == FocusPane::CurrentView {
            self.detail_scroll =
                self.detail_scroll.saturating_sub(count.min(u16::MAX as usize) as u16);
            return;
        }
        self.selected = self.selected.saturating_sub(count);
        self.reset_detail_scroll();
    }

    pub fn jump_next_peer(&mut self) {
        if let Some(index) = self.next_root_index() {
            self.selected = index;
            self.reset_detail_scroll();
        }
    }

    pub fn jump_previous_peer(&mut self) {
        if let Some(index) = self.previous_root_index() {
            self.selected = index;
            self.reset_detail_scroll();
        }
    }

    pub fn expand_selected(&mut self) {
        if let Some(row) = self.rows.get(self.selected).filter(|row| row.has_children) {
            self.expanded.insert(row.task.id);
            self.refresh_rows(Some(row.task.id));
            self.reset_detail_scroll();
        }
    }

    #[must_use]
    pub fn selected_children_hidden(&self) -> bool {
        self.rows
            .get(self.selected)
            .is_some_and(|row| row.has_children && !row.expanded)
    }

    pub fn collapse_selected(&mut self) {
        if let Some(row) = self.rows.get(self.selected).filter(|row| row.expanded) {
            self.expanded.remove(&row.task.id);
            self.refresh_rows(Some(row.task.id));
            self.reset_detail_scroll();
        } else if let Some(parent) =
            self.rows.get(self.selected).and_then(|row| row.parent_id)
        {
            self.refresh_rows(Some(parent));
            self.reset_detail_scroll();
        }
    }

    pub fn scroll_detail_down(&mut self) {
        self.detail_scroll = self.detail_scroll.saturating_add(1);
    }

    pub fn scroll_detail_up(&mut self) {
        self.detail_scroll = self.detail_scroll.saturating_sub(1);
    }

    pub fn reset_detail_scroll(&mut self) {
        self.detail_scroll = 0;
    }

    fn next_root_index(&self) -> Option<usize> {
        let anchor = self.root_anchor(self.selected)?;
        let last = self.last_subtree_index(anchor);
        self.rows
            .iter()
            .enumerate()
            .skip(last + 1)
            .find(|(_, row)| row.depth == 0)
            .map(|(index, _)| index)
    }

    fn previous_root_index(&self) -> Option<usize> {
        let anchor = self.root_anchor(self.selected)?;
        self.rows
            .iter()
            .enumerate()
            .take(anchor)
            .rev()
            .find(|(_, row)| row.depth == 0)
            .map(|(index, _)| index)
    }

    fn root_anchor(&self, index: usize) -> Option<usize> {
        let current = self.rows.get(index)?;
        let mut task_id = current.task.id;
        while let Some(parent_id) = self
            .rows
            .iter()
            .find(|row| row.task.id == task_id)
            .and_then(|row| row.parent_id)
        {
            task_id = parent_id;
        }
        self.rows.iter().position(|row| row.task.id == task_id)
    }

    fn last_subtree_index(&self, anchor: usize) -> usize {
        let depth = self.rows[anchor].depth;
        self.rows
            .iter()
            .enumerate()
            .skip(anchor + 1)
            .find(|(_, row)| row.depth <= depth)
            .map_or(self.rows.len().saturating_sub(1), |(index, _)| index - 1)
    }
}
