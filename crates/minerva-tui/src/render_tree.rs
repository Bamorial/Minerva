use crate::AppState;
use minerva_domain::ArchiveState;
use ratatui::{
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
};

pub fn task_tree(state: &AppState) -> List<'_> {
    let items = state.rows.iter().enumerate().map(|(index, row)| {
        let cursor = if index == state.selected { ">" } else { " " };
        let branch = if row.has_children {
            if row.expanded { "▾" } else { "▸" }
        } else {
            "•"
        };
        let indent = "  ".repeat(row.depth);
        let archived = if row.task.archive_state == ArchiveState::Archived {
            " archived"
        } else {
            ""
        };
        let line = format!(
            "{cursor} {indent}{branch} {} [{}|{}{}]",
            row.task.title, row.task.status, row.task.task_type, archived
        );
        ListItem::new(line)
    });
    List::new(items)
        .style(Style::default().add_modifier(Modifier::BOLD))
        .block(Block::default().title("Tree").borders(Borders::ALL))
}
