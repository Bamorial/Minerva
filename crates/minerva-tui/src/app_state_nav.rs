use crate::AppState;

impl AppState {
    pub fn select_next(&mut self) {
        self.selected = (self.selected + 1).min(self.rows.len().saturating_sub(1));
        self.reset_detail_scroll();
    }

    pub fn select_previous(&mut self) {
        self.selected = self.selected.saturating_sub(1);
        self.reset_detail_scroll();
    }

    pub fn expand_selected(&mut self) {
        if let Some(row) = self.rows.get(self.selected).filter(|row| row.has_children) {
            self.expanded.insert(row.task.id);
            self.refresh_rows(Some(row.task.id));
            self.reset_detail_scroll();
        }
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

    pub fn toggle_archived(&mut self) {
        self.show_archived = !self.show_archived;
        self.refresh_rows(self.selected_task_id());
        self.reset_detail_scroll();
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
}
