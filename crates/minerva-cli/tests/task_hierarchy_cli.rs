mod support;

use std::{fs, str};

use support::{create_task, run, task, temp_dir};

#[test]
fn hierarchy_commands_cover_move_relationships_and_queries() {
    let root = temp_dir("cli-task-hierarchy");
    assert!(run(&root, &["init"]).status.success());
    let top = task(1, "Top");
    let mut parent = task(2, "Parent");
    let mut child = task(3, "Child");
    let related = task(4, "Related");
    parent.parent_id = Some(top.id);
    child.parent_id = Some(parent.id);
    for task in [&top, &parent, &child, &related] {
        create_task(&root, task.clone());
    }
    let moved =
        run(&root, &["move", &child.id.to_string(), "--parent", &top.id.to_string()]);
    assert!(moved.status.success(), "{moved:?}");
    assert!(str::from_utf8(&moved.stdout).unwrap().contains("moved under"));
    let depend = run(
        &root,
        &[
            "depend",
            &child.id.to_string(),
            &related.id.to_string(),
            "--reason",
            "Needed first",
        ],
    );
    assert!(depend.status.success(), "{depend:?}");
    let relate = run(
        &root,
        &[
            "relate",
            &child.id.to_string(),
            &parent.id.to_string(),
            "references",
            "--reason",
            "Shared surface",
        ],
    );
    assert!(relate.status.success(), "{relate:?}");
    let children = run(&root, &["--json", "children", &top.id.to_string()]);
    assert!(children.status.success(), "{children:?}");
    let children_body: serde_json::Value =
        serde_json::from_slice(&children.stdout).unwrap();
    assert_eq!(children_body["result"]["items"].as_array().unwrap().len(), 2);
    let ancestors = run(&root, &["--json", "ancestors", &parent.id.to_string()]);
    assert!(ancestors.status.success(), "{ancestors:?}");
    let ancestors_body: serde_json::Value =
        serde_json::from_slice(&ancestors.stdout).unwrap();
    assert_eq!(ancestors_body["result"]["items"][0]["id"], top.id.to_string());
    let detached = run(&root, &["move", &parent.id.to_string(), "--to-root"]);
    assert!(detached.status.success(), "{detached:?}");
    let undepend =
        run(&root, &["undepend", &child.id.to_string(), &related.id.to_string()]);
    assert!(undepend.status.success(), "{undepend:?}");
    let unrelate = run(
        &root,
        &["unrelate", &child.id.to_string(), &parent.id.to_string(), "references"],
    );
    assert!(unrelate.status.success(), "{unrelate:?}");
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn hierarchy_commands_reject_cycles_and_invalid_relationship_requests() {
    let root = temp_dir("cli-task-hierarchy-errors");
    assert!(run(&root, &["init"]).status.success());
    let root_task = task(1, "Root");
    let mut child = task(2, "Child");
    let third = task(3, "Third");
    child.parent_id = Some(root_task.id);
    for task in [&root_task, &child, &third] {
        create_task(&root, task.clone());
    }
    let moved = run(
        &root,
        &["move", &root_task.id.to_string(), "--parent", &child.id.to_string()],
    );
    assert_eq!(moved.status.code(), Some(15), "{moved:?}");
    let depend =
        run(&root, &["depend", &root_task.id.to_string(), &child.id.to_string()]);
    assert!(depend.status.success(), "{depend:?}");
    let cycle =
        run(&root, &["depend", &child.id.to_string(), &root_task.id.to_string()]);
    assert_eq!(cycle.status.code(), Some(16), "{cycle:?}");
    let invalid = run(
        &root,
        &["relate", &root_task.id.to_string(), &third.id.to_string(), "blocks"],
    );
    assert_eq!(invalid.status.code(), Some(20), "{invalid:?}");
    fs::remove_dir_all(root).unwrap();
}
