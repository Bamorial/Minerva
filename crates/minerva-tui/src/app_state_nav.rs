use crate::AppState;

impl AppState {
    pub fn select_next(&mut self) {
        self.selected = (self.selected + 1).min(self.rows.len().saturating_sub(1));
    }

    pub fn select_previous(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    pub fn expand_selected(&mut self) {
        if let Some(row) = self.rows.get(self.selected).filter(|row| row.has_children) {
            self.expanded.insert(row.task.id);
            self.refresh_rows(Some(row.task.id));
        }
    }

    pub fn collapse_selected(&mut self) {
        if let Some(row) = self.rows.get(self.selected).filter(|row| row.expanded) {
            self.expanded.remove(&row.task.id);
            self.refresh_rows(Some(row.task.id));
        } else if let Some(parent) =
            self.rows.get(self.selected).and_then(|row| row.parent_id)
        {
            self.refresh_rows(Some(parent));
        }
    }

    pub fn toggle_archived(&mut self) {
        self.show_archived = !self.show_archived;
        self.refresh_rows(self.selected_task_id());
    }
}
