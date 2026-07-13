mod support;

use minerva_context::{ContextCompilationRequest, ContextCompilationService};
use minerva_storage::{FilesystemProjectRepository, FilesystemTaskRepository};
use support::realistic_graph;

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
    assert_eq!(first.manifest.total_estimated_tokens, first.estimated_tokens);
}
