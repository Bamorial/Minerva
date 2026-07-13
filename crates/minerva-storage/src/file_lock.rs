use minerva_domain::MinervaError;
use std::fs::{self, File, OpenOptions};
use std::io::{self, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct FileLock {
    path: PathBuf,
    file: Option<File>,
}

impl FileLock {
    pub fn acquire(path: PathBuf) -> Result<Self, MinervaError> {
        let parent = path.parent().unwrap_or_else(|| Path::new("."));
        fs::create_dir_all(parent).map_err(|_| conflict(&path))?;
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|_| conflict(&path))?;
        write_metadata(&mut file).map_err(|_| conflict(&path))?;
        sync_parent(parent).map_err(|_| conflict(&path))?;
        Ok(Self { path, file: Some(file) })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn release(mut self) -> io::Result<()> {
        self.release_inner()
    }

    fn release_inner(&mut self) -> io::Result<()> {
        if self.file.take().is_none() {
            return Ok(());
        }
        fs::remove_file(&self.path)?;
        sync_parent(self.path.parent().unwrap_or_else(|| Path::new(".")))
    }
}

impl Drop for FileLock {
    fn drop(&mut self) {
        let _ = self.release_inner();
    }
}

fn write_metadata(file: &mut File) -> io::Result<()> {
    let created = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| io::Error::new(ErrorKind::InvalidData, err))?
        .as_secs();
    writeln!(file, "pid={}", std::process::id())?;
    writeln!(file, "created_at_unix={created}")?;
    writeln!(file, "stale=remove manually after verifying owner is inactive")?;
    file.sync_all()
}

fn sync_parent(parent: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        File::open(parent)?.sync_all()
    }
    #[cfg(not(unix))]
    {
        let _ = parent;
        Ok(())
    }
}

fn conflict(path: &Path) -> MinervaError {
    MinervaError::LockConflict { path: path.display().to_string() }
}
