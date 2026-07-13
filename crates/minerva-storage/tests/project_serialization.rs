mod support;

use minerva_domain::MinervaError;
use minerva_storage::{MinervaLayout, read_project, write_project};
use std::fs;
use support::{fixture, temp_repo};

#[test]
fn project_fixture_round_trips_through_yaml_storage() {
    let root = temp_repo("project-round-trip");
    let layout = MinervaLayout::new(&root);
    fs::copy(fixture("project.yaml"), layout.project_file()).unwrap();
    let project = read_project(&layout).unwrap();
    assert_eq!(project.name, "Minerva");
    assert_eq!(project.statuses.len(), 4);
    write_project(&layout, &project).unwrap();
    assert_eq!(read_project(&layout).unwrap(), project);
}

#[test]
fn project_reader_rejects_unknown_fields() {
    let root = temp_repo("project-unknown");
    let layout = MinervaLayout::new(&root);
    fs::write(layout.project_file(), "schema_version: 1\nid: PRJ-01ARZ3NDEKTSV4RRFFQ69G5FAV\nname: Minerva\ncreated_at: 2026-07-13T09:00:00Z\ndefault_task_type: feature\ndefault_status: backlog\nstatuses: []\ntransitions: []\ncontext_policy: { max_items: 12, max_dependency_hops: 2, stale_after_hours: 24 }\nextra: nope\n").unwrap();
    let error = read_project(&layout).unwrap_err();
    assert!(
        matches!(error, MinervaError::SchemaError { reason, .. } if reason.contains("unknown field `extra`"))
    );
}

#[test]
fn project_reader_reports_invalid_yaml_and_schema_versions() {
    let broken_root = temp_repo("project-broken");
    let broken = MinervaLayout::new(&broken_root);
    fs::write(broken.project_file(), "schema_version: [").unwrap();
    let parse_error = read_project(&broken).unwrap_err();
    assert!(
        matches!(parse_error, MinervaError::SchemaError { path, reason } if path.ends_with("project.yaml") && !reason.is_empty())
    );
    let version_root = temp_repo("project-version");
    let versioned = MinervaLayout::new(&version_root);
    fs::write(
        versioned.project_file(),
        fs::read_to_string(fixture("project.yaml"))
            .unwrap()
            .replace("schema_version: 1", "schema_version: 2"),
    )
    .unwrap();
    let version_error = read_project(&versioned).unwrap_err();
    assert!(
        matches!(version_error, MinervaError::SchemaError { reason, .. } if reason.contains("unsupported schema version `2`"))
    );
}
