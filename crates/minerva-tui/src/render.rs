use crate::{
    AppState,
    app_state::{CreateField, FocusPane, LinkField},
    render_status::status_line,
    render_task_detail::task_detail,
    render_tree::task_tree,
};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Tabs},
};

pub fn draw(frame: &mut Frame<'_>, state: &AppState) {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(2)])
        .split(frame.area());
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(52), Constraint::Percentage(48)])
        .split(vertical[1]);
    frame.render_widget(tabs(state), vertical[0]);
    frame.render_widget(task_detail(state), horizontal[0]);
    frame.render_widget(task_tree(state), horizontal[1]);
    frame.render_widget(status_line(state), vertical[2]);
    render_overlay(frame, state);
}

fn tabs(state: &AppState) -> Tabs<'static> {
    let selected = match state.focus {
        FocusPane::CurrentView => 0,
        FocusPane::Tree => 1,
    };
    Tabs::new(vec!["0 Current View", "1 Tree"])
        .select(selected)
        .highlight_style(
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )
        .block(Block::default().title("Views").borders(Borders::ALL))
}

fn render_overlay(frame: &mut Frame<'_>, state: &AppState) {
    if let Some(create) = &state.create {
        let area = centered(frame.area(), 60, 8);
        frame.render_widget(Clear, area);
        let body = vec![
            Line::from(format!(
                "Title{}: {}",
                marker(create.field == CreateField::Title),
                create.title
            )),
            Line::from(String::new()),
            Line::from(vec![
                Span::raw(format!(
                    "Type{}: ",
                    marker(create.field == CreateField::TaskType)
                )),
                Span::styled(
                    create
                        .task_types
                        .get(create.selected_type)
                        .cloned()
                        .unwrap_or_else(|| "none".into()),
                    Style::default().fg(Color::LightCyan),
                ),
            ]),
            Line::from(String::new()),
            Line::from("Tab switch  Left/Right type  Enter submit  Esc cancel"),
        ];
        frame.render_widget(
            Paragraph::new(body)
                .block(Block::default().title("Create Task").borders(Borders::ALL)),
            area,
        );
    }
    if let Some(link) = &state.link {
        let area = centered(frame.area(), 70, 12);
        frame.render_widget(Clear, area);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Min(0),
            ])
            .split(area);
        frame.render_widget(
            Paragraph::new(vec![
                Line::from(format!(
                    "Search{}: {}",
                    marker(link.field == LinkField::Query),
                    link.query
                )),
                Line::from(format!(
                    "Type{}: {}",
                    marker(link.field == LinkField::Relationship),
                    relationship_label(link.relationship_type)
                )),
            ])
            .block(Block::default().title("Add Relationship").borders(Borders::ALL)),
            chunks[0],
        );
        frame.render_widget(
            Paragraph::new("Tab switch  Left/Right type  Up/Down results  Enter submit  Esc cancel")
                .block(Block::default().borders(Borders::LEFT | Borders::RIGHT)),
            chunks[1],
        );
        let items = link.candidates.iter().enumerate().map(|(index, candidate)| {
            let marker = if index == link.selected { ">" } else { " " };
            let indent = "  ".repeat(candidate.depth);
            ListItem::new(format!(
                "{marker} {indent}{} - {}",
                candidate.task_ref, candidate.title
            ))
        });
        frame.render_widget(
            List::new(items).block(Block::default().borders(Borders::ALL)),
            chunks[2],
        );
    }
    if let Some(delete) = &state.delete {
        let area = centered(frame.area(), 50, 6);
        frame.render_widget(Clear, area);
        frame.render_widget(
            Paragraph::new(vec![
                Line::from(format!(
                    "Delete {} {} and {} task(s)?",
                    delete.task_ref, delete.title, delete.descendants
                )),
                Line::from(String::new()),
                Line::from("Enter/y confirm  Esc/n cancel"),
            ])
            .block(Block::default().title("Delete Task").borders(Borders::ALL)),
            area,
        );
    }
}

fn marker(active: bool) -> &'static str {
    if active { "*" } else { "" }
}

fn relationship_label(value: minerva_domain::RelationshipType) -> &'static str {
    match value {
        minerva_domain::RelationshipType::DependsOn => "depends-on",
        minerva_domain::RelationshipType::References => "references",
        _ => "depends-on",
    }
}

fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(height),
            Constraint::Fill(1),
        ])
        .split(area);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(width),
            Constraint::Fill(1),
        ])
        .split(vertical[1])[1]
}
