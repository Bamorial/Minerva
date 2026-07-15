use minerva_application::TaskTreeNode;
use minerva_tui::AppState;
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

fn titles(state: &AppState) -> Vec<String> {
    state.rows.iter().map(|row| row.task.title.clone()).collect()
}
