use crate::{MinervaLayout, ProjectLock};
use minerva_domain::MinervaError;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static NEXT_DIR_ID: AtomicU64 = AtomicU64::new(0);

#[test]
fn project_lock_creates_and_releases_runtime_file() {
    let dir = temp_dir();
    let layout = MinervaLayout::new(&dir);
    let path = layout.project_lock_file();
    let lock = ProjectLock::acquire(&layout).unwrap();
    assert_eq!(lock.path(), path.as_path());
    assert!(path.exists());
    lock.release().unwrap();
    assert!(!path.exists());
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn second_writer_gets_structured_lock_conflict() {
    let dir = temp_dir();
    let layout = MinervaLayout::new(&dir);
    let _lock = ProjectLock::acquire(&layout).unwrap();
    let error = ProjectLock::acquire(&layout).unwrap_err();
    assert_eq!(
        error,
        MinervaError::LockConflict {
            path: layout.project_lock_file().display().to_string(),
        }
    );
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn stale_lock_file_blocks_new_writers_until_removed() {
    let dir = temp_dir();
    let layout = MinervaLayout::new(&dir);
    fs::create_dir_all(layout.locks_dir()).unwrap();
    fs::write(layout.project_lock_file(), b"pid=999999\n").unwrap();
    assert!(matches!(
        ProjectLock::acquire(&layout),
        Err(MinervaError::LockConflict { .. })
    ));
    fs::remove_file(layout.project_lock_file()).unwrap();
    ProjectLock::acquire(&layout).unwrap().release().unwrap();
    fs::remove_dir_all(dir).unwrap();
}

fn temp_dir() -> PathBuf {
    let unique = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let sequence = NEXT_DIR_ID.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir()
        .join(format!("minerva-project-lock-{}-{unique}-{sequence}", process::id()));
    fs::create_dir(&dir).unwrap();
    dir
}
