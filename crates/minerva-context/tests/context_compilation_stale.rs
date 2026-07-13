mod support;

use minerva_context::{
    ContextCompilationError, ContextCompilationRequest, ContextCompilationService,
};
use minerva_domain::{ContextPolicy, RelationshipType};
use minerva_storage::{FilesystemProjectRepository, FilesystemTaskRepository};
use support::{
    persist_task, refresh_declaration, relate, repo, stale_task,
    write_project_instructions,
};

#[test]
fn service_rejects_stale_declarations_for_selected_dependencies() {
    let root = repo("stale");
    write_project_instructions(&root, "# Project\n\nKeep context deterministic.");
    let target =
        persist_task(&root, 1, "Target", None, "# Target", "# Declaration", &["pass"]);
    let dependency = persist_task(
        &root,
        2,
        "Dependency",
        None,
        "# Dependency",
        "# Declaration",
        &["pass"],
    );
    relate(&root, target.id, dependency.id, RelationshipType::DependsOn);
    refresh_declaration(&root, target.id, "Target");
    stale_task(&root, &dependency);
    let request = ContextCompilationRequest {
        task_ref: target.id.to_string(),
        policy: Some(ContextPolicy::strict()),
        budget: None,
    };
    let error = ContextCompilationService::compile(
        &FilesystemProjectRepository,
        &FilesystemTaskRepository,
        &root,
        &request,
    )
    .unwrap_err();
    assert!(
        matches!(error, ContextCompilationError::StaleReference { task_ref, .. } if task_ref == dependency.id.to_string())
    );
}
