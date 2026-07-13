mod atomic_write;
mod config_document;
mod file_lock;
mod filesystem_project_repository;
mod filesystem_task_repository;
mod layout_catalog;
mod layout_entry;
mod minerva_layout;
mod project_document;
mod project_document_parts;
mod project_files;
mod project_init;
mod project_instructions;
mod project_lock;
mod project_scaffold;
mod relationship_catalog;
mod relationship_document;
mod relationship_files;
mod relationship_mutations;
mod task_catalog;
mod task_document;
mod task_document_parts;
mod task_event_actor;
mod task_event_data;
mod task_event_reader;
mod task_event_record;
mod task_events;
mod task_files;
mod task_freshness;
mod task_hierarchy;
mod task_lock;
mod task_markdown;
mod task_repository_mutations;
mod task_repository_queries;
mod task_repository_support;
mod task_type_document;
mod task_type_files;
mod yaml_codec;

pub use atomic_write::atomic_replace;
pub(crate) use file_lock::FileLock;
pub use filesystem_project_repository::FilesystemProjectRepository;
pub use filesystem_task_repository::FilesystemTaskRepository;
pub use layout_catalog::{project_layout, task_layout};
pub use layout_entry::{LayoutClass, LayoutEntry};
pub use minerva_layout::MinervaLayout;
pub use project_files::{
    read_project, read_project_config, write_project, write_project_config,
};
pub use project_init::initialize_project;
pub use project_instructions::{read_project_instructions, write_project_instructions};
pub use project_lock::ProjectLock;
pub use project_scaffold::{
    SCHEMA_VERSION, TASK_TYPES, agents_md, default_config, default_project,
    instructions_md,
};
pub use relationship_catalog::{
    list_relationships, list_relationships_from, list_relationships_to,
};
pub use relationship_files::{read_relationships, write_relationships};
pub use relationship_mutations::{create_relationship, remove_relationship};
pub use task_event_reader::read_task_events;
pub use task_events::{
    append_archived_event, append_created_event, append_declaration_updated_event,
    append_instructions_updated_event, append_parent_changed_event,
    append_relationship_added_event, append_relationship_removed_event,
    append_status_changed_event,
};
pub use task_files::{
    read_task, read_task_declaration, read_task_instructions, read_task_notes,
    write_task, write_task_declaration, write_task_instructions, write_task_notes,
};
pub use task_lock::{TaskLock, TaskLocks};
pub use task_type_files::read_task_types;

#[cfg(test)]
mod atomic_write_tests;
#[cfg(test)]
mod project_lock_tests;
#[cfg(test)]
mod task_lock_tests;
