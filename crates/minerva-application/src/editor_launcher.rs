use crate::EditorEnvironment;
use minerva_domain::{MinervaError, ProjectConfig};
use std::path::Path;

pub struct EditorLauncher;

impl EditorLauncher {
    pub fn edit_path(
        path: &Path,
        config: Option<&ProjectConfig>,
    ) -> Result<(), MinervaError> {
        Self::edit_path_with_environment(path, config, &EditorEnvironment::capture())
    }

    pub fn edit_path_with_environment(
        path: &Path,
        config: Option<&ProjectConfig>,
        env: &EditorEnvironment,
    ) -> Result<(), MinervaError> {
        let editor = env.resolve(config.and_then(|value| value.editor.as_deref()))?;
        let status = editor.for_path(path).status().map_err(|err| {
            MinervaError::EditorLaunchFailure {
                editor: editor.spec.clone(),
                reason: err.to_string(),
            }
        })?;
        if status.success() {
            return Ok(());
        }
        Err(MinervaError::EditorLaunchFailure {
            editor: editor.spec,
            reason: format!("editor exited with status {status}"),
        })
    }
}
