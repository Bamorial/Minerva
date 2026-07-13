use minerva_domain::{MinervaError, ProjectConfig};

#[test]
fn project_config_accepts_supported_editor_override() {
    let config = ProjectConfig::new(ProjectConfig {
        schema_version: 1,
        editor: Some("zed --wait".into()),
    })
    .unwrap();
    assert_eq!(config.editor.as_deref(), Some("zed --wait"));
}

#[test]
fn project_config_rejects_zero_schema_and_blank_editor() {
    let zero = ProjectConfig::new(ProjectConfig {
        schema_version: 0,
        editor: Some("zed".into()),
    });
    let blank = ProjectConfig::new(ProjectConfig {
        schema_version: 1,
        editor: Some("   ".into()),
    });
    assert!(
        matches!(zero, Err(MinervaError::InvalidConfiguration { key, .. }) if key == "schema_version")
    );
    assert!(
        matches!(blank, Err(MinervaError::InvalidConfiguration { key, .. }) if key == "editor")
    );
}
