#![allow(dead_code)]

use std::{
    io::Write,
    path::PathBuf,
    process::{Command, Output, Stdio},
};

pub fn run_with_input(root: &PathBuf, args: &[&str], input: &str) -> Output {
    let mut child = Command::new(binary())
        .args(args)
        .current_dir(root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    child.wait_with_output().unwrap()
}

fn binary() -> PathBuf {
    std::env::var_os("CARGO_BIN_EXE_minerva-cli").unwrap().into()
}
