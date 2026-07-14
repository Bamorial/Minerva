use crate::AppState;

impl AppState {
    pub fn begin_search(&mut self) {
        self.search_mode = true;
    }

    pub fn append_search(&mut self, value: char) {
        self.search.push(value);
        self.refresh_rows(self.selected_task_id());
    }

    pub fn pop_search(&mut self) {
        self.search.pop();
        self.refresh_rows(self.selected_task_id());
    }

    pub fn clear_search(&mut self) {
        self.search.clear();
        self.refresh_rows(self.selected_task_id());
    }

    pub fn finish_search(&mut self) {
        self.search_mode = false;
    }
}
