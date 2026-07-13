use crate::atomic_write::{atomic_replace, atomic_replace_with, temp_path};
use std::fs;
use std::io::{self, ErrorKind};
use std::path::PathBuf;
use std::process;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static NEXT_DIR_ID: AtomicU64 = AtomicU64::new(0);

#[test]
fn atomic_replace_writes_new_file() {
    let dir = temp_dir();
    let path = dir.join("project.yaml");
    atomic_replace(&path, b"v1").unwrap();
    assert_eq!(fs::read(&path).unwrap(), b"v1");
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn atomic_replace_replaces_existing_file() {
    let dir = temp_dir();
    let path = dir.join("task.yaml");
    fs::write(&path, b"old").unwrap();
    atomic_replace(&path, b"new").unwrap();
    assert_eq!(fs::read(&path).unwrap(), b"new");
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn interrupted_write_keeps_existing_file_and_cleans_temp() {
    let dir = temp_dir();
    let path = dir.join("declaration.md");
    fs::write(&path, b"stable").unwrap();
    let result = atomic_replace_with(
        &path,
        b"draft",
        || Err(io::Error::new(ErrorKind::Interrupted, "stop")),
        || Ok(()),
    );
    assert_eq!(result.unwrap_err().kind(), ErrorKind::Interrupted);
    assert_eq!(fs::read(&path).unwrap(), b"stable");
    assert!(!temp_path(&path).exists());
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn temp_file_stays_in_target_directory() {
    let dir = temp_dir();
    let path = dir.join("result.md");
    assert_eq!(temp_path(&path).parent(), path.parent());
    fs::remove_dir_all(dir).unwrap();
}

fn temp_dir() -> PathBuf {
    let unique = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let sequence = NEXT_DIR_ID.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir()
        .join(format!("minerva-storage-{}-{unique}-{sequence}", process::id()));
    fs::create_dir(&dir).unwrap();
    dir
}
