use minerva_application::{BootstrapService, render_task_facts};
use minerva_domain::Task;

#[must_use]
pub fn compile_workspace_context() -> String {
    let crates = BootstrapService::workspace_blueprint().crates().join(", ");
    let interfaces = BootstrapService::interface_descriptions()
        .map(|item| item.crate_name)
        .join(", ");
    format!("Workspace crates: {crates}\nInterfaces: {interfaces}")
}

#[must_use]
pub fn compile_task_context(task: &Task) -> String {
    format!("Task: {} {}\n{}", task.id, task.title, render_task_facts(task))
}

#[cfg(test)]
mod tests {
    use super::{compile_task_context, compile_workspace_context};
    use minerva_domain::{
        ArchiveState, DeclarationActor, DeclarationMetadata, StatusKey, Task,
        TaskFacts, TaskIdAllocator, TaskPriority, TaskResources, TaskTypeKey,
        TaskVersion,
    };
    use std::collections::BTreeSet;
    use std::time::UNIX_EPOCH;

    #[test]
    fn context_compilation_is_deterministic() {
        let context = compile_workspace_context();
        assert!(context.contains("minerva-domain"));
        assert!(context.contains("minerva-mcp"));
    }

    #[test]
    fn task_context_includes_structured_facts() {
        let task = Task::new(Task {
            schema_version: 1,
            id: TaskIdAllocator::new(0).next_id(),
            title: "Define task facts".into(),
            slug: None,
            task_type: TaskTypeKey::new("feature").unwrap(),
            status: StatusKey::new("backlog").unwrap(),
            parent_id: None,
            priority: TaskPriority::Medium,
            tags: BTreeSet::default(),
            created_at: UNIX_EPOCH,
            updated_at: UNIX_EPOCH,
            completed_at: None,
            version: TaskVersion::initial(),
            declaration: DeclarationMetadata {
                version: 1,
                updated_at: UNIX_EPOCH,
                updated_by: DeclarationActor::Human,
                commit_hash: None,
            },
            facts: TaskFacts {
                modules: vec!["minerva-domain".into()],
                files: vec!["crates/minerva-domain/src/task_facts.rs".into()],
                migrations_required: true,
                feature_flags: vec!["task-facts".into()],
                acceptance_checks: vec!["round-trip persistence".into()],
                resources: TaskResources::default(),
            },
            archive_state: ArchiveState::Active,
        })
        .unwrap();
        let context = compile_task_context(&task);
        assert!(context.contains("facts.modules: minerva-domain"));
        assert!(context.contains("facts.migrations_required: true"));
    }
}
