use crate::{MinervaLayout, TaskLock, TaskLocks};
use minerva_domain::{MinervaError, TaskId};
use std::fs;
use std::num::NonZeroU32;
use std::path::PathBuf;
use std::process;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

static NEXT_DIR_ID: AtomicU64 = AtomicU64::new(0);

#[test]
fn same_task_conflict_returns_structured_error() {
    let dir = temp_dir();
    let layout = MinervaLayout::new(&dir);
    let task_id = task_id(7);
    let _lock = TaskLock::acquire(&layout, task_id).unwrap();
    let error = TaskLock::acquire(&layout, task_id).unwrap_err();
    assert_eq!(
        error,
        MinervaError::LockConflict {
            path: layout.task_lock_file(task_id).display().to_string()
        }
    );
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn different_tasks_can_be_locked_concurrently() {
    let dir = temp_dir();
    let layout = MinervaLayout::new(&dir);
    let first = TaskLock::acquire(&layout, task_id(1)).unwrap();
    let root = dir.clone();
    let worker = thread::spawn(move || {
        let layout = MinervaLayout::new(root);
        TaskLock::acquire(&layout, task_id(2)).unwrap().release().unwrap();
    });
    worker.join().unwrap();
    first.release().unwrap();
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn multi_task_lock_orders_acquisition_and_cleans_up_partial_state() {
    let dir = temp_dir();
    let layout = MinervaLayout::new(&dir);
    let first = task_id(1);
    let second = task_id(2);
    let _held = TaskLock::acquire(&layout, first).unwrap();
    let error = TaskLocks::acquire(&layout, [second, first]).unwrap_err();
    assert_eq!(
        error,
        MinervaError::LockConflict {
            path: layout.task_lock_file(first).display().to_string()
        }
    );
    assert!(!layout.task_lock_file(second).exists());
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn multi_task_lock_deduplicates_repeated_task_ids() {
    let dir = temp_dir();
    let layout = MinervaLayout::new(&dir);
    let task_id = task_id(9);
    let locks = TaskLocks::acquire(&layout, [task_id, task_id]).unwrap();
    assert_eq!(locks.paths(), vec![layout.task_lock_file(task_id)]);
    locks.release().unwrap();
    fs::remove_dir_all(dir).unwrap();
}

fn task_id(sequence: u32) -> TaskId {
    TaskId::from_sequence(NonZeroU32::new(sequence).unwrap())
}

fn temp_dir() -> PathBuf {
    let unique = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let sequence = NEXT_DIR_ID.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir()
        .join(format!("minerva-task-lock-{}-{unique}-{sequence}", process::id()));
    fs::create_dir(&dir).unwrap();
    dir
}
