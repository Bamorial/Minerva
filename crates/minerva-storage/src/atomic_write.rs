use std::fs::{self, File, OpenOptions};
use std::io::{self, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::process;

pub fn atomic_replace(path: &Path, contents: &[u8]) -> io::Result<()> {
    atomic_replace_with(path, contents, || Ok(()), || Ok(()))
}

pub(crate) fn atomic_replace_with<F, G>(
    path: &Path,
    contents: &[u8],
    after_sync: F,
    before_rename: G,
) -> io::Result<()>
where
    F: FnOnce() -> io::Result<()>,
    G: FnOnce() -> io::Result<()>,
{
    let parent = path
        .parent()
        .ok_or_else(|| io::Error::new(ErrorKind::InvalidInput, "path needs parent"))?;
    let temp = temp_path(path);
    let result = (|| {
        let mut file = OpenOptions::new().write(true).create_new(true).open(&temp)?;
        file.write_all(contents)?;
        file.flush()?;
        file.sync_all()?;
        after_sync()?;
        drop(file);
        before_rename()?;
        fs::rename(&temp, path)?;
        sync_parent(parent)
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temp);
    }
    result
}

pub(crate) fn temp_path(path: &Path) -> PathBuf {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("minerva");
    parent.join(format!(".{name}.tmp.{}", process::id()))
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
