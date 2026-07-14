use crate::AppState;
use ratatui::{
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

pub fn task_detail(state: &AppState) -> Paragraph<'_> {
    let text = state.detail.as_ref().map_or_else(
        || vec![Line::from("No task selected.")],
        |detail| {
            vec![
                Line::from(detail.task.title.clone()),
                Line::from(format!(
                    "{} | {} | {:?}",
                    detail.task.id, detail.task.status, detail.task.priority
                )),
                Line::from(format!("type: {}", detail.task.task_type)),
                Line::from(format!("freshness: {}", detail.freshness.status)),
                Line::from(format!("updated: {}", detail.timestamps.updated_at)),
            ]
        },
    );
    Paragraph::new(text).block(Block::default().title("Details").borders(Borders::ALL))
}
