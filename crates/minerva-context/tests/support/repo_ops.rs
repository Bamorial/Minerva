#![allow(dead_code)]

use minerva_application::{ProjectRepository, TaskRepository};
use minerva_domain::{
    DeclarationActor, DeclarationDocument, Relationship, RelationshipId,
    RelationshipType, Task, TaskId,
};
use minerva_storage::{FilesystemProjectRepository, FilesystemTaskRepository};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn repo(name: &str) -> PathBuf {
    let root =
        std::env::temp_dir().join(format!("minerva-context-{name}-{}", unique()));
    fs::create_dir_all(&root).unwrap();
    FilesystemProjectRepository.initialize_project(&root, false).unwrap();
    root
}

pub fn write_project_instructions(root: &Path, body: &str) {
    FilesystemProjectRepository.write_project_instructions(root, body).unwrap();
}

pub fn stale_task(root: &Path, task: &Task) {
    let task = FilesystemTaskRepository.read_task(root, task.id).unwrap();
    FilesystemTaskRepository
        .update_task_instructions(
            root,
            task.id,
            task.version,
            "# Updated\n\nThis task is now stale.",
        )
        .unwrap();
}

pub fn refresh_declaration(root: &Path, task_id: TaskId, body: &str) {
    let task = FilesystemTaskRepository.read_task(root, task_id).unwrap();
    FilesystemTaskRepository
        .update_task_declaration(
            root,
            task_id,
            task.version,
            DeclarationActor::Human,
            None,
            &valid_declaration(body),
        )
        .unwrap();
}

pub fn relate(root: &Path, source: TaskId, target: TaskId, kind: RelationshipType) {
    FilesystemTaskRepository
        .create_relationship(
            root,
            &Relationship::new(Relationship {
                schema_version: 1,
                id: RelationshipId::new(),
                source_task: source,
                target_task: target,
                relationship_type: kind,
                reason: None,
                created_at: UNIX_EPOCH,
            })
            .unwrap(),
        )
        .unwrap();
}

fn unique() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
}

fn valid_declaration(body: &str) -> String {
    DeclarationDocument::template()
        .replace("## Objective\n", &format!("## Objective\n{body}\n"))
}
