use crate::{FileLock, MinervaLayout};
use minerva_domain::{MinervaError, TaskId};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct TaskLock(FileLock);

impl TaskLock {
    pub fn acquire(
        layout: &MinervaLayout,
        task_id: TaskId,
    ) -> Result<Self, MinervaError> {
        Ok(Self(FileLock::acquire(layout.task_lock_file(task_id))?))
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        self.0.path()
    }

    pub fn release(self) -> std::io::Result<()> {
        self.0.release()
    }
}

#[derive(Debug)]
pub struct TaskLocks {
    locks: Vec<FileLock>,
}

impl TaskLocks {
    pub fn acquire(
        layout: &MinervaLayout,
        task_ids: impl IntoIterator<Item = TaskId>,
    ) -> Result<Self, MinervaError> {
        let mut paths: Vec<_> = task_ids
            .into_iter()
            .map(|task_id| layout.task_lock_file(task_id))
            .collect();
        paths.sort();
        paths.dedup();
        let mut locks = Vec::with_capacity(paths.len());
        for path in paths {
            locks.push(FileLock::acquire(path)?);
        }
        Ok(Self { locks })
    }

    #[must_use]
    pub fn paths(&self) -> Vec<PathBuf> {
        self.locks.iter().map(|lock| lock.path().to_path_buf()).collect()
    }

    pub fn release(self) -> std::io::Result<()> {
        for lock in self.locks {
            lock.release()?;
        }
        Ok(())
    }
}
