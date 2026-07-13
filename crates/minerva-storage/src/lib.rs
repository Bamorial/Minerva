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
mod task_lock;
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
pub use task_lock::{TaskLock, TaskLocks};

#[cfg(test)]
mod atomic_write_tests;
#[cfg(test)]
mod project_lock_tests;
#[cfg(test)]
mod task_lock_tests;
