use crate::common::{
    render::{normalize, render_screen},
    sample_state::sample_state,
};

mod common;

#[test]
fn draw_renders_selected_task_details() {
    let state = sample_state("Render TUI tree");
    let screen = normalize(&render_screen(&state));
    assert!(screen.contains(&normalize("Render TUI tree")));
    assert!(screen.contains(&normalize("Parent: No parent.")));
    assert!(screen.contains(&normalize("No dependencies.")));
    assert!(screen.contains(&normalize("related-to outgoing TSK-0002 Other task")));
    assert!(screen.contains(&normalize("Modules: minerva-tui")));
    assert!(screen.contains(&normalize("[backlog|feature]")));
}

#[test]
fn draw_renders_scrolled_detail_sections() {
    let mut state = sample_state("Render TUI tree");
    state.detail_scroll = 10;
    let screen = normalize(&render_screen(&state));
    assert!(screen.contains(&normalize("task-metadata-updated-after-declaration")));
    assert!(screen.contains(&normalize("Detail pane is implemented.")));
}

#[test]
fn draw_renders_invalid_declaration_message() {
    let mut state = sample_state("Broken declaration");
    state.detail_scroll = 10;
    state.detail.as_mut().unwrap().declaration =
        Some("# Declaration\n\n## Current State\nbroken".into());
    let screen = normalize(&render_screen(&state));
    assert!(screen.contains(&normalize("Invalid declaration")));
}
