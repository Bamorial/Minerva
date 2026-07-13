mod atomic_write;
mod config_document;
mod file_lock;
mod layout_catalog;
mod layout_entry;
mod minerva_layout;
mod project_document;
mod project_document_parts;
mod project_files;
mod project_lock;
mod task_document;
mod task_document_parts;
mod task_files;
mod task_lock;
mod task_markdown;
mod yaml_codec;

pub use atomic_write::atomic_replace;
pub(crate) use file_lock::FileLock;
pub use layout_catalog::{project_layout, task_layout};
pub use layout_entry::{LayoutClass, LayoutEntry};
pub use minerva_layout::MinervaLayout;
pub use project_files::{
    read_project, read_project_config, write_project, write_project_config,
};
pub use project_lock::ProjectLock;
pub use task_files::{
    read_task, read_task_declaration, read_task_instructions, read_task_notes,
    write_task, write_task_declaration, write_task_instructions, write_task_notes,
};
pub use task_lock::{TaskLock, TaskLocks};

#[cfg(test)]
mod atomic_write_tests;
#[cfg(test)]
mod project_lock_tests;
#[cfg(test)]
mod task_lock_tests;
