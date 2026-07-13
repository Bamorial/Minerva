use minerva_application::{EditorEnvironment, EditorLauncher};
use minerva_domain::MinervaError;
use std::path::Path;

#[test]
fn launch_failure_is_structured_when_process_cannot_start() {
    let env = EditorEnvironment {
        minerva_editor: Some("minerva-editor-command-that-should-not-exist".into()),
        visual: None,
        editor: None,
    };
    let error = EditorLauncher::edit_path_with_environment(
        Path::new("Cargo.toml"),
        None,
        &env,
    )
    .unwrap_err();
    assert!(matches!(
        error,
        MinervaError::EditorLaunchFailure { editor, reason }
            if editor == "minerva-editor-command-that-should-not-exist"
                && !reason.is_empty()
    ));
}
