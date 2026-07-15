use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use minerva_application::TaskTreeNode;
use minerva_tui::{AppState, Dispatch, dispatch};
use std::path::PathBuf;

mod common;

#[test]
fn tree_navigation_supports_expand_search_and_peer_jumps() {
    let root_task = common::sample_state::sample_task("Root task", None, false);
    let child = TaskTreeNode {
        task: common::sample_state::sample_task(
            "Child match",
            Some(root_task.id),
            false,
        ),
        children: Vec::new(),
    };
    let sibling = TaskTreeNode {
        task: common::sample_state::sample_task("Sibling task", None, false),
        children: Vec::new(),
    };
    let root = TaskTreeNode { task: root_task, children: vec![child.clone()] };
    let mut state = AppState::new(PathBuf::from("."));
    state.set_tree(vec![root, sibling]);
    assert_eq!(titles(&state), vec!["Root task", "Child match", "Sibling task"]);
    state.collapse_selected();
    assert_eq!(titles(&state), vec!["Root task", "Sibling task"]);
    state.expand_selected();
    assert_eq!(titles(&state), vec!["Root task", "Child match", "Sibling task"]);
    state.begin_search();
    for value in "child".chars() {
        state.append_search(value);
    }
    assert_eq!(titles(&state), vec!["Root task", "Child match"]);
    state.finish_search();
    state.clear_search();
    state.jump_next_peer();
    assert_eq!(state.rows[state.selected].task.title, "Sibling task");
    state.jump_previous_peer();
    assert_eq!(state.rows[state.selected].task.title, "Root task");
}

#[test]
fn enter_expands_collapsed_children_before_switching_focus() {
    let root_task = common::sample_state::sample_task("Root task", None, false);
    let child = TaskTreeNode {
        task: common::sample_state::sample_task(
            "Child task",
            Some(root_task.id),
            false,
        ),
        children: Vec::new(),
    };
    let root = TaskTreeNode { task: root_task, children: vec![child] };
    let mut state = AppState::new(PathBuf::from("."));
    state.set_tree(vec![root]);
    state.collapse_selected();
    assert_eq!(titles(&state), vec!["Root task"]);

    assert_eq!(press(&mut state, KeyCode::Enter), Dispatch::None);
    assert_eq!(titles(&state), vec!["Root task", "Child task"]);
    assert_eq!(state.focus, minerva_tui::FocusPane::Tree);

    assert_eq!(press(&mut state, KeyCode::Enter), Dispatch::None);
    assert_eq!(state.focus, minerva_tui::FocusPane::CurrentView);
}

#[test]
fn create_modal_prioritizes_feature_and_wraps_task_types() {
    let mut state = AppState::new(PathBuf::from("."));
    state.set_task_types(vec!["bug".into(), "feature".into(), "chore".into()]);
    state.begin_create(None);
    let create = state.create.as_ref().unwrap();
    assert_eq!(create.task_types, vec!["feature", "bug", "chore"]);
    assert_eq!(create.selected_type, 0);

    assert_eq!(press(&mut state, KeyCode::Tab), Dispatch::None);
    assert_eq!(press(&mut state, KeyCode::Left), Dispatch::None);
    let create = state.create.as_ref().unwrap();
    assert_eq!(create.task_types[create.selected_type], "chore");
    assert_eq!(press(&mut state, KeyCode::Right), Dispatch::None);
    let create = state.create.as_ref().unwrap();
    assert_eq!(create.task_types[create.selected_type], "feature");
}

fn titles(state: &AppState) -> Vec<String> {
    state.rows.iter().map(|row| row.task.title.clone()).collect()
}

fn press(state: &mut AppState, code: KeyCode) -> Dispatch {
    dispatch(state, Event::Key(KeyEvent::new(code, KeyModifiers::NONE)))
}
