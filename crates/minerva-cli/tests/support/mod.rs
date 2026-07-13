use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

static NEXT_DIR_ID: AtomicU64 = AtomicU64::new(0);

pub fn run(root: &PathBuf, args: &[&str]) -> Output {
    Command::new(binary()).args(args).current_dir(root).output().unwrap()
}

pub fn temp_dir(name: &str) -> PathBuf {
    let unique = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let sequence = NEXT_DIR_ID.fetch_add(1, Ordering::Relaxed);
    let dir =
        std::env::temp_dir().join(format!("minerva-cli-{name}-{unique}-{sequence}"));
    fs::create_dir(&dir).unwrap();
    dir
}

#[allow(dead_code)]
pub fn write_editor(root: &PathBuf, name: &str, body: &str) -> PathBuf {
    let path = root.join(name);
    fs::write(&path, format!("#!/bin/sh\nset -eu\n{body}")).unwrap();
    let mut perms = fs::metadata(&path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&path, perms).unwrap();
    path
}

fn binary() -> PathBuf {
    std::env::var_os("CARGO_BIN_EXE_minerva-cli").unwrap().into()
}
