use crate::{AppState, app_state::FocusPane};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
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
        ListItem::new(Line::from(vec![
            Span::styled(
                format!(
                    "{cursor} {indent}{branch} {} - {} - ",
                    row.task.id, row.task.title
                ),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                row.task.status.to_string(),
                Style::default().fg(status_color(row.task.status.as_str())),
            ),
            Span::raw(" - "),
            Span::styled(
                row.task.task_type.to_string(),
                Style::default().fg(type_color(row.task.task_type.as_str())),
            ),
        ]))
    });
    List::new(items).block(
        Block::default().title("Tree").borders(Borders::ALL).border_style(
            if state.focus == FocusPane::Tree {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            },
        ),
    )
}

fn status_color(value: &str) -> Color {
    match value {
        "backlog" => Color::Gray,
        "in-progress" => Color::LightYellow,
        "done" => Color::LightGreen,
        "blocked" => Color::LightRed,
        _ => Color::Cyan,
    }
}

fn type_color(value: &str) -> Color {
    match value {
        "feature" => Color::LightCyan,
        "bug" => Color::LightRed,
        "refactor" => Color::Magenta,
        "research" => Color::LightBlue,
        "documentation" => Color::Green,
        "chore" => Color::Yellow,
        _ => Color::White,
    }
}
