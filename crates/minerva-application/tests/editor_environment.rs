use minerva_application::{EditorEnvironment, EditorSource};
use minerva_domain::MinervaError;

#[test]
fn resolution_prefers_minerva_editor_over_other_sources() {
    let env = EditorEnvironment {
        minerva_editor: Some("hx --wait".into()),
        visual: Some("zed --wait".into()),
        editor: Some("vim".into()),
    };
    let editor = env.resolve_with_fallback(Some("code --wait"), "nano").unwrap();
    assert_eq!(editor.source, EditorSource::MinervaEditor);
    assert_eq!(editor.program, "hx");
    assert_eq!(editor.args, vec!["--wait"]);
}

#[test]
fn resolution_supports_quoted_programs_and_skips_blank_values() {
    let env = EditorEnvironment {
        minerva_editor: Some("   ".into()),
        visual: None,
        editor: None,
    };
    let editor = env
        .resolve_with_fallback(Some("\"Visual Studio Code\" --wait"), "nano")
        .unwrap();
    assert_eq!(editor.source, EditorSource::Configured);
    assert_eq!(editor.program, "Visual Studio Code");
    assert_eq!(editor.args, vec!["--wait"]);
}

#[test]
fn resolution_reports_invalid_editor_commands() {
    let env = EditorEnvironment::default();
    let error = env.resolve_with_fallback(Some("\"broken"), "nano").unwrap_err();
    assert!(matches!(
        error,
        MinervaError::EditorLaunchFailure { editor, .. } if editor == "\"broken"
    ));
}

#[cfg(windows)]
#[test]
fn resolution_uses_windows_fallback() {
    let editor = EditorEnvironment::default().resolve(None).unwrap();
    assert_eq!(editor.source, EditorSource::Fallback);
    assert_eq!(editor.program, "notepad");
}

#[cfg(not(windows))]
#[test]
fn resolution_uses_unix_fallback() {
    let editor = EditorEnvironment::default().resolve(None).unwrap();
    assert_eq!(editor.source, EditorSource::Fallback);
    assert_eq!(editor.program, "nvim");
}
