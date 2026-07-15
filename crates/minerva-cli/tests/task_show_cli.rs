mod support;

use minerva_application::TaskRepository;
use minerva_domain::{Relationship, RelationshipId, RelationshipType};
use minerva_storage::FilesystemTaskRepository;
use std::{fs, str, time::UNIX_EPOCH};
use support::{create_task, run, task, temp_dir};

#[test]
fn show_command_renders_full_task_summary() {
    let root = temp_dir("cli-task-show");
    let repo = FilesystemTaskRepository;
    assert!(run(&root, &["init"]).status.success());
    let parent = task(1, "Parent task");
    let mut current = task(2, "Inspect declaration freshness");
    let dependency = task(3, "Prepare repository");
    let related = task(4, "Document repository");
    current.parent_id = Some(parent.id);
    for item in [&parent, &current, &dependency, &related] {
        create_task(&root, item.clone());
    }
    repo.create_relationship(
        &root,
        &relationship(current.id, dependency.id, RelationshipType::DependsOn, None),
    )
    .unwrap();
    repo.create_relationship(
        &root,
        &relationship(
            current.id,
            related.id,
            RelationshipType::RelatedTo,
            Some("same surface"),
        ),
    )
    .unwrap();
    let output = run(&root, &["show", &current.id.to_string()]);
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        str::from_utf8(&output.stdout).unwrap(),
        expected(&current, &parent, &dependency, &related)
    );
    fs::remove_dir_all(root).unwrap();
}

fn expected(
    task: &minerva_domain::Task,
    parent: &minerva_domain::Task,
    dependency: &minerva_domain::Task,
    related: &minerva_domain::Task,
) -> String {
    format!(
        "{id} Inspect declaration freshness\ntype: feature\nstatus: backlog\npriority: medium\nparent: {parent_id} Parent task\ndependencies: {dep_id} Prepare repository\ndeclaration freshness: potentially-stale\nfreshness reasons: missing-covered-commit\ncreated_at: 1970-01-01T00:00:00Z\nupdated_at: 1970-01-01T00:00:00Z\ncompleted_at: none\ndeclaration_updated_at: 1970-01-01T00:00:00Z\nversion: 1\ndeclaration version: 1\nfacts.modules: none\nfacts.files: none\nfacts.migrations_required: false\nfacts.feature_flags: none\nfacts.acceptance_checks: none\nfacts.resources.reads: none\nfacts.resources.writes: none\nrelationships:\n- related-to outgoing {rel_id} Document repository (same surface)\n",
        id = task.id,
        parent_id = parent.id,
        dep_id = dependency.id,
        rel_id = related.id,
    )
}

fn relationship(
    source: minerva_domain::TaskId,
    target: minerva_domain::TaskId,
    kind: RelationshipType,
    reason: Option<&str>,
) -> Relationship {
    Relationship::new(Relationship {
        schema_version: 1,
        id: RelationshipId::new(),
        source_task: source,
        target_task: target,
        relationship_type: kind,
        reason: reason.map(str::to_string),
        created_at: UNIX_EPOCH,
    })
    .unwrap()
}
