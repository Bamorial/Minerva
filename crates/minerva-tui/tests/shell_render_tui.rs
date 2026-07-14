use minerva_application::{
    TaskShowFreshness, TaskShowResult, TaskShowTimestamps, TaskTreeNode,
    TuiErrorMessage,
};
use minerva_tui::{AppState, draw};
use ratatui::{Terminal, backend::TestBackend};
use std::path::PathBuf;

mod common;

#[test]
fn draw_renders_selected_task_details() {
    let mut state = AppState::new(PathBuf::from("."));
    let task = common::sample_task("Render TUI tree", None, false);
    state.set_tree(vec![TaskTreeNode { task: task.clone(), children: Vec::new() }]);
    state.detail = Some(TaskShowResult {
        task,
        parent: None,
        dependencies: Vec::new(),
        relationships: Vec::new(),
        freshness: TaskShowFreshness { status: "fresh".into(), reasons: Vec::new() },
        timestamps: TaskShowTimestamps {
            created_at: "1970-01-01T00:00:00Z".into(),
            updated_at: "1970-01-01T00:00:00Z".into(),
            completed_at: None,
            declaration_updated_at: "1970-01-01T00:00:00Z".into(),
        },
        instructions: None,
        declaration: None,
    });
    let screen = render_screen(&state);
    assert!(screen.contains("Views"));
    assert!(screen.contains("Tree"));
    assert!(screen.contains("Details"));
    assert!(screen.contains("Render TUI tree"));
    assert!(screen.contains("[backlog|feature]"));
}

#[test]
fn draw_renders_error_message() {
    let mut state = AppState::new(PathBuf::from("."));
    state.error = Some(TuiErrorMessage {
        title: "Project not initialized",
        body: "run minerva init".into(),
    });
    let screen = render_screen(&state);
    assert!(screen.contains("Project not initialized: run minerva init"));
}

fn render_screen(state: &AppState) -> String {
    let backend = TestBackend::new(90, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|frame| draw(frame, state)).unwrap();
    terminal.backend().buffer().content().iter().map(|cell| cell.symbol()).collect()
}
