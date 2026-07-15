mod support;

use minerva_domain::MinervaError;
use minerva_storage::{MinervaLayout, read_project_config, write_project_config};
use std::fs;
use support::{fixture, temp_repo};

#[test]
fn config_fixture_round_trips_through_yaml_storage() {
    let root = temp_repo("config-round-trip");
    let layout = MinervaLayout::new(&root);
    fs::copy(fixture("config.yaml"), layout.config_file()).unwrap();
    let config = read_project_config(&layout).unwrap();
    assert_eq!(config.editor.as_deref(), Some("zed --wait"));
    assert_eq!(config.default_priority, minerva_domain::TaskPriority::High);
    assert_eq!(config.agent_prompt_mode, minerva_domain::AgentPromptMode::Static);
    assert_eq!(
        config.default_tags,
        std::collections::BTreeSet::from([
            minerva_domain::TaskTag::new("release").unwrap()
        ])
    );
    write_project_config(&layout, &config).unwrap();
    assert_eq!(read_project_config(&layout).unwrap(), config);
}

#[test]
fn config_reader_rejects_unknown_fields_and_schema_mismatches() {
    let unknown_root = temp_repo("config-unknown");
    let unknown = MinervaLayout::new(&unknown_root);
    fs::write(
        unknown.config_file(),
        "schema_version: 1\neditor: zed --wait\ndefault_priority: Medium\ndefault_tags: []\nshell: zsh\n",
    )
    .unwrap();
    let unknown_error = read_project_config(&unknown).unwrap_err();
    assert!(
        matches!(unknown_error, MinervaError::SchemaError { reason, .. } if reason.contains("unknown field `shell`"))
    );
    let version_root = temp_repo("config-version");
    let versioned = MinervaLayout::new(&version_root);
    fs::write(
        versioned.config_file(),
        "schema_version: 2\neditor: zed --wait\ndefault_priority: Medium\ndefault_tags: []\n",
    )
    .unwrap();
    let version_error = read_project_config(&versioned).unwrap_err();
    assert!(
        matches!(version_error, MinervaError::SchemaError { reason, .. } if reason.contains("unsupported schema version `2`"))
    );
}
