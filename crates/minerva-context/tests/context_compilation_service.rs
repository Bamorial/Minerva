mod support;

use minerva_application::TaskRepository;
use minerva_context::{ContextCompilationRequest, ContextCompilationService};
use minerva_domain::{ContextPolicy, DeclarationActor, DeclarationDocument};
use minerva_storage::{FilesystemProjectRepository, FilesystemTaskRepository};
use support::{persist_task, realistic_graph, repo};

#[test]
fn service_compiles_realistic_graph_deterministically() {
    let (root, target, child, policy) = realistic_graph();
    let request = ContextCompilationRequest {
        task_ref: target.id.to_string(),
        policy: Some(policy),
        budget: None,
    };
    let first = ContextCompilationService::compile(
        &FilesystemProjectRepository,
        &FilesystemTaskRepository,
        &root,
        &request,
    )
    .unwrap();
    let second = ContextCompilationService::compile(
        &FilesystemProjectRepository,
        &FilesystemTaskRepository,
        &root,
        &request,
    )
    .unwrap();
    assert_eq!(first, second);
    assert!(first.markdown.contains("## Project Instructions"));
    assert!(first.markdown.contains("## Ancestor Declarations"));
    assert!(first.markdown.contains("## Dependency Declarations"));
    assert!(first.markdown.contains("## Related Task Summaries"));
    assert!(first.markdown.contains(&child.id.to_string()));
    assert!(first.markdown.contains("Declaration path: `.minerva/tasks/"));
    assert!(!first.markdown.contains("## Context Manifest Summary"));
    assert_eq!(first.manifest.total_estimated_tokens, first.estimated_tokens);
}

#[test]
fn service_skips_placeholder_project_instructions_and_empty_declarations() {
    let root = repo("compile-cleanup");
    let target = persist_task(
        &root,
        1,
        "Target",
        None,
        "# Feature\n\nImplement cleanup.",
        "Temporary declaration text.",
        &[],
    );
    let current = FilesystemTaskRepository.read_task(&root, target.id).unwrap();
    FilesystemTaskRepository
        .update_task_declaration(
            &root,
            target.id,
            current.version,
            DeclarationActor::Human,
            None,
            &DeclarationDocument::template(),
        )
        .unwrap();
    let result = ContextCompilationService::compile(
        &FilesystemProjectRepository,
        &FilesystemTaskRepository,
        &root,
        &ContextCompilationRequest {
            task_ref: target.id.to_string(),
            policy: Some(ContextPolicy::strict()),
            budget: None,
        },
    )
    .unwrap();
    assert!(!result.markdown.contains("## Project Instructions"));
    assert!(!result.markdown.contains("## Target Declaration"));
    assert!(result.markdown.contains("Declaration path: `.minerva/tasks/"));
}
