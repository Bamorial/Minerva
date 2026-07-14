use crate::AppState;
use ratatui::{
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

pub fn status_line(state: &AppState) -> Paragraph<'_> {
    let body = body(state);
    Paragraph::new(Line::from(body))
        .style(style(state))
        .block(Block::default().borders(Borders::TOP))
}

fn body(state: &AppState) -> String {
    if let Some(confirm) = &state.confirm {
        return confirm.message.clone();
    }
    if let Some(select) = &state.select {
        return select_line(select);
    }
    if let Some(prompt) = &state.prompt {
        return prompt_line(prompt.kind, &prompt.value);
    }
    if state.search_mode {
        return format!("/{}", state.search);
    }
    if let Some(error) = &state.error {
        return format!("{}: {}", error.title, error.body);
    }
    if let Some(notice) = &state.notice {
        return notice.clone();
    }
    help(state)
}

fn help(state: &AppState) -> String {
    if state.search_mode {
        return format!("/{}", state.search);
    }
    format!(
        "j/k move  h/l fold  n new  s status  m move  i/e edit  d add dep  x rm dep  / search  a archived:{}  r reload  q quit",
        if state.show_archived { "on" } else { "off" }
    )
}

fn prompt_line(kind: crate::app_prompt::PromptKind, value: &str) -> String {
    let label = match kind {
        crate::app_prompt::PromptKind::CreateTask => "New child task title",
        crate::app_prompt::PromptKind::MoveTask => "Move task to parent ref or root",
        crate::app_prompt::PromptKind::AddDependency => "Add dependency on task ref",
        crate::app_prompt::PromptKind::RemoveDependency => {
            "Remove dependency on task ref"
        }
    };
    format!("{label}: {value}")
}

fn select_line(select: &crate::app_select::SelectState) -> String {
    let options = select
        .options
        .iter()
        .enumerate()
        .map(|(index, option)| {
            let marker = if index == select.selected { "*" } else { "" };
            format!("{}{} {}", index + 1, marker, option)
        })
        .collect::<Vec<_>>()
        .join("  ");
    format!("{}: {}  enter apply  esc cancel", select.title, options)
}

fn style(state: &AppState) -> Style {
    if state.error.is_some() {
        return Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD);
    }
    if state.notice.is_some() {
        return Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD);
    }
    Style::default().add_modifier(Modifier::BOLD)
}
