use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorCommand {
    pub source: EditorSource,
    pub spec: String,
    pub program: String,
    pub args: Vec<String>,
}

impl EditorCommand {
    #[must_use]
    pub fn for_path(&self, path: &Path) -> Command {
        let mut command = Command::new(&self.program);
        command.args(&self.args);
        command.arg(path);
        command
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorSource {
    MinervaEditor,
    Visual,
    Editor,
    Configured,
    Fallback,
}
