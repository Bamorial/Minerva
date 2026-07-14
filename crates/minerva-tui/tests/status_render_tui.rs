use minerva_application::TuiErrorMessage;
use minerva_tui::AppState;
use std::path::PathBuf;

use crate::common::render::{normalize, render_screen};

mod common;

#[test]
fn draw_renders_error_message() {
    let mut state = AppState::new(PathBuf::from("."));
    state.error = Some(TuiErrorMessage {
        title: "Project not initialized",
        body: "run minerva init".into(),
    });
    let screen = normalize(&render_screen(&state));
    assert!(screen.contains(&normalize("Project not initialized: run minerva init")));
}
