use crate::AppState;
use ratatui::{
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

pub fn status_line(state: &AppState) -> Paragraph<'_> {
    let body = state.error.as_ref().map_or_else(
        || help(state),
        |error| format!("{}: {}", error.title, error.body),
    );
    Paragraph::new(Line::from(body))
        .style(Style::default().add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::TOP))
}

fn help(state: &AppState) -> String {
    if state.search_mode {
        return format!("/{}", state.search);
    }
    format!(
        "j/k move  h/l fold  ctrl-u/d detail  / search  c clear  a archived:{}  r reload  q quit",
        if state.show_archived { "on" } else { "off" }
    )
}
