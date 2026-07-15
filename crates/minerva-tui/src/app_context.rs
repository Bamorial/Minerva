use minerva_application::{ProjectRepository, TaskRepository};
use minerva_domain::{
    AgentPromptMode, DeclarationDocument, MinervaError, RelationshipType, Task, TaskId,
};
use minerva_storage::{
    FilesystemProjectRepository, FilesystemTaskRepository, MinervaLayout,
};
use std::collections::{BTreeSet, HashMap};
use std::path::Path;

const PROJECT_INSTRUCTIONS_PLACEHOLDER: &str =
    "# Project Instructions\n\nAdd repository-wide Minerva instructions here.";
const STATIC_EXECUTION_CONTRACT: &str = "- Work only on the current task.\n\
- Do not resolve sibling tasks or unrelated work.\n\
- Use dependency task context only as reference.\n\
- Use sibling task context only as reference.\n\
- Use only the project instructions, parent task context, current task \
context, dependency task context, and sibling task context included below.";

pub fn load(
    start: &Path,
    task_ref: &str,
    mode: AgentPromptMode,
) -> Result<String, MinervaError> {
    match mode {
        AgentPromptMode::Static => static_prompt(start, task_ref),
        AgentPromptMode::Exploration => exploration_prompt(start, task_ref),
    }
}

fn static_prompt(start: &Path, task_ref: &str) -> Result<String, MinervaError> {
    let root = FilesystemProjectRepository.locate_project_root(start)?;
    let task = FilesystemTaskRepository.resolve_task(&root, task_ref)?;
    let tasks = FilesystemTaskRepository.list_tasks(&root)?;
    let mut sections = Vec::new();
    sections
        .push(section("Minerva Execution Contract", STATIC_EXECUTION_CONTRACT.into()));
    add_project_instructions(&mut sections, &root)?;
    add_parent_context(&mut sections, &root, &tasks, &task)?;
    add_current_task_context(&mut sections, &root, task.id)?;
    add_dependency_context(&mut sections, &root, &tasks, task.id)?;
    add_sibling_context(&mut sections, &root, &tasks, &task)?;
    sections.push(section("Output Requirements", output_requirements(&task)));
    Ok(format!("[static]\n\n{}", sections.join("\n\n")))
}

fn exploration_prompt(start: &Path, task_ref: &str) -> Result<String, MinervaError> {
    let root = FilesystemProjectRepository.locate_project_root(start)?;
    let task = FilesystemTaskRepository.resolve_task(&root, task_ref)?;
    let tasks = FilesystemTaskRepository.list_tasks(&root)?;
    let layout = MinervaLayout::new(&root);
    let parents = ancestor_paths(&layout, &tasks, &task)?;
    let dependencies = dependency_paths(&layout, &tasks, &root, task.id)?;
    let siblings = sibling_paths(&layout, &tasks, &task);
    Ok(format!(
        "[exploration]\n\nInvestigate the referenced Minerva files before changing code.\nVerify all existing parent tasks before changing code.\nTask path: `{}`\nProject instructions: `{}`\nParent paths:\n{}\nDependency paths:\n{}\nSibling paths:\n{}\nTask instructions: `{}`\nDeclaration to complete: `{}`",
        layout.task_dir(task.id).display(),
        layout.instructions_file().display(),
        path_block(&parents),
        path_block(&dependencies),
        path_block(&siblings),
        layout.task_instructions_file(task.id).display(),
        layout.declaration_file(task.id).display(),
    ))
}

fn add_project_instructions(
    sections: &mut Vec<String>,
    root: &Path,
) -> Result<(), MinervaError> {
    let instructions = FilesystemProjectRepository.read_project_instructions(root)?;
    if meaningful_project_instructions(&instructions) {
        sections.push(section("Project Instructions", instructions.trim().to_owned()));
    }
    Ok(())
}

fn add_parent_context(
    sections: &mut Vec<String>,
    root: &Path,
    tasks: &[Task],
    target: &Task,
) -> Result<(), MinervaError> {
    let parents = ancestor_lineage(tasks, target)?;
    if parents.is_empty() {
        return Ok(());
    }
    let mut blocks = Vec::new();
    let count = parents.len();
    for (index, task_id) in parents.into_iter().enumerate() {
        let generation = count - index;
        blocks.push(parent_generation_block(root, task_id, generation)?);
    }
    if !blocks.is_empty() {
        sections.push(section("Parent Task Context", blocks.join("\n\n")));
    }
    Ok(())
}

fn add_current_task_context(
    sections: &mut Vec<String>,
    root: &Path,
    task_id: TaskId,
) -> Result<(), MinervaError> {
    let blocks = task_context_blocks(root, task_id)?;
    if !blocks.is_empty() {
        sections.push(section("Current Task Context", blocks.join("\n\n")));
    }
    Ok(())
}

fn add_dependency_context(
    sections: &mut Vec<String>,
    root: &Path,
    tasks: &[Task],
    task_id: TaskId,
) -> Result<(), MinervaError> {
    let dependency_ids = dependency_ids(tasks, root, task_id)?;
    if dependency_ids.is_empty() {
        return Ok(());
    }
    let mut blocks = Vec::new();
    for dependency_id in dependency_ids {
        blocks.extend(task_context_blocks(root, dependency_id)?);
    }
    if !blocks.is_empty() {
        sections.push(section("Dependency Task Context", blocks.join("\n\n")));
    }
    Ok(())
}

fn add_sibling_context(
    sections: &mut Vec<String>,
    root: &Path,
    tasks: &[Task],
    target: &Task,
) -> Result<(), MinervaError> {
    let siblings = sibling_ids(tasks, target);
    if siblings.is_empty() {
        return Ok(());
    }
    let mut blocks = Vec::new();
    for task_id in siblings {
        blocks.extend(task_context_blocks(root, task_id)?);
    }
    if !blocks.is_empty() {
        sections.push(section("Sibling Task Context", blocks.join("\n\n")));
    }
    Ok(())
}

fn ancestor_lineage(
    tasks: &[Task],
    target: &Task,
) -> Result<Vec<TaskId>, MinervaError> {
    let task_map = tasks.iter().map(|task| (task.id, task)).collect::<HashMap<_, _>>();
    let mut lineage = Vec::new();
    let mut next = target.parent_id;
    while let Some(parent_id) = next {
        let Some(parent) = task_map.get(&parent_id) else {
            return Err(MinervaError::InvalidConfiguration {
                key: "context.parent".into(),
                reason: format!("missing parent task {parent_id}"),
            });
        };
        lineage.push(parent_id);
        next = parent.parent_id;
    }
    lineage.reverse();
    Ok(lineage)
}

fn dependency_ids(
    tasks: &[Task],
    root: &Path,
    task_id: TaskId,
) -> Result<Vec<TaskId>, MinervaError> {
    let outgoing = FilesystemTaskRepository.list_relationships_from(root, task_id)?;
    let incoming = FilesystemTaskRepository.list_relationships_to(root, task_id)?;
    let ids = outgoing
        .into_iter()
        .filter(|rel| {
            rel.relationship_type == RelationshipType::DependsOn
                && rel.source_task == task_id
        })
        .map(|rel| rel.target_task)
        .chain(incoming.into_iter().filter_map(|rel| {
            (rel.relationship_type == RelationshipType::Blocks
                && rel.target_task == task_id)
                .then_some(rel.source_task)
        }))
        .collect::<BTreeSet<_>>();
    Ok(tasks.iter().filter(|task| ids.contains(&task.id)).map(|task| task.id).collect())
}

fn sibling_ids(tasks: &[Task], target: &Task) -> Vec<TaskId> {
    tasks
        .iter()
        .filter(|item| item.parent_id == target.parent_id && item.id != target.id)
        .map(|item| item.id)
        .collect()
}

fn dependency_paths(
    layout: &MinervaLayout,
    tasks: &[Task],
    root: &Path,
    task_id: TaskId,
) -> Result<Vec<String>, MinervaError> {
    Ok(dependency_ids(tasks, root, task_id)?
        .into_iter()
        .map(|id| format!("- `{}`", layout.task_dir(id).display()))
        .collect())
}

fn task_context_blocks(
    root: &Path,
    task_id: TaskId,
) -> Result<Vec<String>, MinervaError> {
    let task = FilesystemTaskRepository.read_task(root, task_id)?;
    let mut blocks = Vec::new();
    blocks.push(format!("### {} {}", task.id, task.title));
    blocks.push(subsection(
        "Instructions",
        FilesystemTaskRepository
            .read_task_instructions(root, task_id)?
            .trim()
            .to_owned(),
    ));
    let declaration = FilesystemTaskRepository.read_task_declaration(root, task_id)?;
    if !DeclarationDocument::is_effectively_empty(&declaration) {
        blocks.push(subsection("Declaration", declaration.trim().to_owned()));
    }
    Ok(blocks)
}

fn parent_generation_block(
    root: &Path,
    task_id: TaskId,
    generation: usize,
) -> Result<String, MinervaError> {
    let body = task_context_blocks(root, task_id)?.join("\n\n");
    Ok(format!("### Parent Generation {generation}\n\n{}", body.trim()))
}

fn meaningful_project_instructions(text: &str) -> bool {
    let trimmed = text.trim();
    !trimmed.is_empty() && trimmed != PROJECT_INSTRUCTIONS_PLACEHOLDER
}

fn output_requirements(task: &Task) -> String {
    [
        "Complete only the current task shown above.".into(),
        "Do not resolve sibling tasks.".into(),
        "The agent must complete the declaration before finishing.".into(),
        format!("Declaration path: `.minerva/tasks/{}/declaration.md`", task.id),
    ]
    .join("\n")
}

fn section(title: &str, body: String) -> String {
    format!("## {title}\n\n{}", body.trim())
}

fn subsection(title: &str, body: String) -> String {
    format!("#### {title}\n\n{}", body.trim())
}

fn ancestor_paths(
    layout: &MinervaLayout,
    tasks: &[Task],
    target: &Task,
) -> Result<Vec<String>, MinervaError> {
    Ok(ancestor_lineage(tasks, target)?
        .into_iter()
        .map(|id| format!("- `{}`", layout.task_dir(id).display()))
        .collect())
}

fn sibling_paths(layout: &MinervaLayout, tasks: &[Task], task: &Task) -> Vec<String> {
    tasks
        .iter()
        .filter(|item| item.parent_id == task.parent_id && item.id != task.id)
        .map(|item| format!("- `{}`", layout.task_dir(item.id).display()))
        .collect()
}

fn path_block(paths: &[String]) -> String {
    if paths.is_empty() { "- none".into() } else { paths.join("\n") }
}

#[cfg(test)]
mod tests {
    use super::load;
    use minerva_application::{ProjectRepository, TaskCreateRecord, TaskRepository};
    use minerva_domain::{
        AgentPromptMode, ArchiveState, DeclarationActor, DeclarationDocument,
        DeclarationMetadata, Relationship, RelationshipId, RelationshipType, StatusKey,
        Task, TaskFacts, TaskIdAllocator, TaskPriority, TaskTypeKey, TaskVersion,
    };
    use minerva_storage::{FilesystemProjectRepository, FilesystemTaskRepository};
    use std::collections::BTreeSet;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static NEXT_DIR_ID: AtomicU64 = AtomicU64::new(0);

    #[test]
    fn static_prompt_includes_lineage_and_sibling_context() {
        let root = temp_dir("static-prompt-lineage");
        FilesystemProjectRepository.initialize_project(&root, false).unwrap();
        FilesystemProjectRepository
            .write_project_instructions(&root, "# Project\n\nRespect repository rules.")
            .unwrap();
        let grandparent = persist_task(
            &root,
            1,
            "Grandparent",
            None,
            "Root context.",
            "Root declaration.",
        );
        let parent = persist_task(
            &root,
            2,
            "Parent",
            Some(grandparent.id),
            "Parent context.",
            "Parent declaration.",
        );
        let target = persist_task(
            &root,
            3,
            "Target",
            Some(parent.id),
            "Target context.",
            "Target declaration.",
        );
        let sibling = persist_task(
            &root,
            4,
            "Sibling",
            Some(parent.id),
            "Sibling context.",
            "Sibling declaration.",
        );
        let dependency = persist_task(
            &root,
            5,
            "Dependency",
            None,
            "Dependency context.",
            "Dependency declaration.",
        );
        create_dependency(&root, target.id, dependency.id);

        let prompt =
            load(&root, &target.id.to_string(), AgentPromptMode::Static).unwrap();

        assert!(prompt.starts_with("[static]\n\n## Minerva Execution Contract"));
        assert!(prompt.contains("## Project Instructions"));
        assert!(prompt.contains("## Parent Task Context"));
        assert!(prompt.contains("### Parent Generation 2"));
        assert!(prompt.contains("### Parent Generation 1"));
        assert!(
            prompt.contains(&format!("### {} {}", grandparent.id, grandparent.title))
        );
        assert!(prompt.contains(&format!("### {} {}", parent.id, parent.title)));
        assert!(prompt.contains("## Current Task Context"));
        assert!(prompt.contains(&format!("### {} {}", target.id, target.title)));
        assert!(prompt.contains("## Dependency Task Context"));
        assert!(
            prompt.contains(&format!("### {} {}", dependency.id, dependency.title))
        );
        assert!(prompt.contains("Do not resolve sibling tasks."));
        assert!(
            prompt
                .contains("The agent must complete the declaration before finishing.")
        );
        assert!(prompt.contains(&format!(
            "Declaration path: `.minerva/tasks/{}/declaration.md`",
            target.id
        )));
        assert!(prompt.contains("## Sibling Task Context"));
        assert!(prompt.contains(&format!("### {} {}", sibling.id, sibling.title)));
        assert!(ordered(&prompt, "## Parent Task Context", "## Current Task Context"));
        assert!(ordered(
            &prompt,
            "## Current Task Context",
            "## Dependency Task Context"
        ));
        assert!(ordered(
            &prompt,
            "## Dependency Task Context",
            "## Sibling Task Context"
        ));
    }

    #[test]
    fn static_prompt_skips_placeholder_project_instructions_and_empty_declaration() {
        let root = temp_dir("static-prompt-cleanup");
        FilesystemProjectRepository.initialize_project(&root, false).unwrap();
        let target = persist_task(&root, 1, "Target", None, "Target context.", "");
        clear_declaration(&root, target.id);

        let prompt =
            load(&root, &target.id.to_string(), AgentPromptMode::Static).unwrap();

        assert!(!prompt.contains("## Project Instructions"));
        assert!(!prompt.contains("#### Declaration"));
        assert!(prompt.contains("#### Instructions"));
    }

    #[test]
    fn static_prompt_skips_empty_sibling_declarations() {
        let root = temp_dir("static-prompt-sibling-declaration");
        FilesystemProjectRepository.initialize_project(&root, false).unwrap();
        let parent = persist_task(
            &root,
            1,
            "Parent",
            None,
            "Parent context.",
            "Parent declaration.",
        );
        let target = persist_task(
            &root,
            2,
            "Target",
            Some(parent.id),
            "Target context.",
            "Target declaration.",
        );
        let sibling =
            persist_task(&root, 3, "Sibling", Some(parent.id), "Sibling context.", "");
        clear_declaration(&root, sibling.id);

        let prompt =
            load(&root, &target.id.to_string(), AgentPromptMode::Static).unwrap();

        assert!(prompt.contains("## Sibling Task Context"));
        assert!(prompt.contains(&format!("### {} {}", sibling.id, sibling.title)));
        assert!(!prompt.contains("Sibling declaration."));
    }

    #[test]
    fn static_prompt_includes_dependency_context_and_skips_empty_declaration() {
        let root = temp_dir("static-prompt-dependency-declaration");
        FilesystemProjectRepository.initialize_project(&root, false).unwrap();
        let target = persist_task(&root, 1, "Target", None, "Target context.", "");
        let dependency =
            persist_task(&root, 2, "Dependency", None, "Dependency context.", "");
        clear_declaration(&root, dependency.id);
        create_dependency(&root, target.id, dependency.id);

        let prompt =
            load(&root, &target.id.to_string(), AgentPromptMode::Static).unwrap();

        assert!(prompt.contains("## Dependency Task Context"));
        assert!(
            prompt.contains(&format!("### {} {}", dependency.id, dependency.title))
        );
        assert!(prompt.contains("Dependency context."));
        assert!(!prompt.contains("Dependency declaration."));
    }

    #[test]
    fn exploration_prompt_lists_dependency_paths() {
        let root = temp_dir("exploration-prompt-dependencies");
        FilesystemProjectRepository.initialize_project(&root, false).unwrap();
        let grandparent =
            persist_task(&root, 1, "Grandparent", None, "Grandparent context.", "");
        let parent = persist_task(
            &root,
            2,
            "Parent",
            Some(grandparent.id),
            "Parent context.",
            "",
        );
        let target =
            persist_task(&root, 3, "Target", Some(parent.id), "Target context.", "");
        let dependency =
            persist_task(&root, 4, "Dependency", None, "Dependency context.", "");
        create_dependency(&root, target.id, dependency.id);

        let prompt =
            load(&root, &target.id.to_string(), AgentPromptMode::Exploration).unwrap();

        assert!(
            prompt.contains("Verify all existing parent tasks before changing code.")
        );
        assert!(prompt.contains("Parent paths:"));
        assert!(prompt.contains(&format!(
                "- `{}`",
                minerva_storage::MinervaLayout::new(&root)
                    .task_dir(grandparent.id)
                    .display()
            )));
        assert!(prompt.contains(&format!(
            "- `{}`",
            minerva_storage::MinervaLayout::new(&root).task_dir(parent.id).display()
        )));
        assert!(prompt.contains("Dependency paths:"));
        assert!(prompt.contains(&format!(
            "- `{}`",
            minerva_storage::MinervaLayout::new(&root)
                .task_dir(dependency.id)
                .display()
        )));
    }

    fn ordered(text: &str, first: &str, second: &str) -> bool {
        let first = text.find(first).unwrap();
        let second = text.find(second).unwrap();
        first < second
    }

    fn persist_task(
        root: &Path,
        sequence: u32,
        title: &str,
        parent_id: Option<minerva_domain::TaskId>,
        instructions: &str,
        declaration_text: &str,
    ) -> Task {
        let task = task(sequence, title, parent_id);
        let declaration = if declaration_text.is_empty() {
            DeclarationDocument::template()
        } else {
            DeclarationDocument::template().replace(
                "## Current State\n",
                &format!("## Current State\n{declaration_text}\n"),
            )
        };
        FilesystemTaskRepository
            .create_task(
                root,
                &TaskCreateRecord {
                    task: task.clone(),
                    instructions: format!("# Feature\n\n{instructions}"),
                    declaration,
                },
            )
            .unwrap();
        task
    }

    fn clear_declaration(root: &Path, task_id: minerva_domain::TaskId) {
        let task = FilesystemTaskRepository.read_task(root, task_id).unwrap();
        FilesystemTaskRepository
            .update_task_declaration(
                root,
                task_id,
                task.version,
                DeclarationActor::Human,
                None,
                &DeclarationDocument::template(),
            )
            .unwrap();
    }

    fn create_dependency(
        root: &Path,
        source_task: minerva_domain::TaskId,
        target_task: minerva_domain::TaskId,
    ) {
        FilesystemTaskRepository
            .create_relationship(
                root,
                &Relationship::new(Relationship {
                    schema_version: 1,
                    id: RelationshipId::new(),
                    source_task,
                    target_task,
                    relationship_type: RelationshipType::DependsOn,
                    reason: None,
                    created_at: UNIX_EPOCH,
                })
                .unwrap(),
            )
            .unwrap();
    }

    fn task(
        sequence: u32,
        title: &str,
        parent_id: Option<minerva_domain::TaskId>,
    ) -> Task {
        Task::new(Task {
            schema_version: 1,
            id: TaskIdAllocator::new(sequence - 1).next_id(),
            title: title.into(),
            slug: None,
            task_type: TaskTypeKey::new("feature").unwrap(),
            status: StatusKey::new("backlog").unwrap(),
            parent_id,
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
            facts: TaskFacts::default(),
            archive_state: ArchiveState::Active,
        })
        .unwrap()
    }

    fn temp_dir(name: &str) -> PathBuf {
        let unique = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let sequence = NEXT_DIR_ID.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir()
            .join(format!("minerva-tui-{name}-{unique}-{sequence}"));
        fs::create_dir(&dir).unwrap();
        dir
    }
}
