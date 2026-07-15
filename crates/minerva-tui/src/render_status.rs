use crate::{
    AppState,
    app_state::{CreateField, CurrentView, FocusPane, LinkField},
};
use minerva_domain::AgentPromptMode;
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
    if state.settings.is_some() {
        return "settings: choose prompt mode".into();
    }
    if let Some(create) = &state.create {
        let field = match create.field {
            CreateField::Title => "title",
            CreateField::TaskType => "type",
        };
        return format!("create task: editing {field}");
    }
    if let Some(link) = &state.link {
        let field = match link.field {
            LinkField::Query => "query",
            LinkField::Relationship => "relationship",
            LinkField::Results => "results",
        };
        return format!("add relation: editing {field}");
    }
    if let Some(delete) = &state.delete {
        return format!("delete {} {}?", delete.task_ref, delete.title);
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
    let focus = match state.focus {
        FocusPane::CurrentView => "0 current",
        FocusPane::Tree => "1 tree",
    };
    let current = match state.current_view {
        CurrentView::Details => "details",
        CurrentView::Context => "context",
    };
    let count = if state.count_buffer.is_empty() {
        String::new()
    } else {
        format!(" count:{} ", state.count_buffer)
    };
    format!(
        "{focus} {current} mode:{}{count} j/k move  h/l fold  0/enter current  1 tree  n new  a child  A sibling  nt/Nt jump tasks  e task instructions  I project instructions  c context  y copy context  @ link  d delete  s settings  S status  m move  / search  r reload  q quit",
        prompt_mode(state.prompt_mode),
    )
}

fn prompt_mode(mode: AgentPromptMode) -> &'static str {
    match mode {
        AgentPromptMode::Static => "static",
        AgentPromptMode::Exploration => "exploration",
    }
}

fn prompt_line(kind: crate::app_prompt::PromptKind, value: &str) -> String {
    let label = match kind {
        crate::app_prompt::PromptKind::MoveTask => "Move task to parent ref or root",
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
