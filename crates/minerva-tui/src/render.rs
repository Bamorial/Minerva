use crate::{
    AppState, render_status::status_line, render_task_detail::task_detail,
    render_tree::task_tree,
};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Tabs},
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
    frame.render_widget(tabs(), vertical[0]);
    frame.render_widget(task_tree(state), horizontal[0]);
    frame.render_widget(task_detail(state), horizontal[1]);
    frame.render_widget(status_line(state), vertical[2]);
}

fn tabs() -> Tabs<'static> {
    Tabs::new(vec!["1 Tree"])
        .select(0)
        .block(Block::default().title("Views").borders(Borders::ALL))
}
