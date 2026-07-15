use crate::common::{
    render::{normalize, render_screen},
    sample_state::sample_state,
};

mod common;

#[test]
fn draw_applies_detail_scroll() {
    let mut state = sample_state("Scroll detail");
    state.detail_scroll = 9;
    let screen = normalize(&render_screen(&state));
    assert!(!screen.contains(&normalize("Parent: No parent.")));
    assert!(screen.contains(&normalize("covered-commit-differs")));
}
