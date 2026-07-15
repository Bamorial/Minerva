use crate::{AppState, app_state::CurrentView, task_detail_lines::build_lines};
use ratatui::{
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

pub fn task_detail(state: &AppState) -> Paragraph<'_> {
    let text = match state.current_view {
        CurrentView::Details => state
            .detail
            .as_ref()
            .map_or_else(|| vec![Line::from("No task selected.")], build_lines),
        CurrentView::Context => state.context.as_deref().map_or_else(
            || vec![Line::from("No compiled context loaded.")],
            context_lines,
        ),
    };
    Paragraph::new(text)
        .scroll((state.detail_scroll, 0))
        .style(Style::default().fg(Color::Cyan))
        .block(
            Block::default().title("Current View").borders(Borders::ALL).title_style(
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        )
}

fn context_lines(value: &str) -> Vec<Line<'static>> {
    value.lines().map(|line| Line::from(line.to_string())).collect()
}
