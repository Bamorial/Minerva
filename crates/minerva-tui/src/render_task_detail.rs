use crate::{AppState, task_detail_lines::build_lines};
use ratatui::{
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};

pub fn task_detail(state: &AppState) -> Paragraph<'_> {
    let text = state
        .detail
        .as_ref()
        .map_or_else(|| vec!["No task selected.".into()], build_lines);
    Paragraph::new(text)
        .scroll((state.detail_scroll, 0))
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().title("Details").borders(Borders::ALL).title_style(
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        ))
}
