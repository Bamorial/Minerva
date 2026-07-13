use crate::{FileLock, MinervaLayout};
use minerva_domain::MinervaError;
use std::path::Path;

#[derive(Debug)]
pub struct ProjectLock(FileLock);

impl ProjectLock {
    pub fn acquire(layout: &MinervaLayout) -> Result<Self, MinervaError> {
        Ok(Self(FileLock::acquire(layout.project_lock_file())?))
    }

    pub fn path(&self) -> &Path {
        self.0.path()
    }

    pub fn release(self) -> std::io::Result<()> {
        self.0.release()
    }
}
