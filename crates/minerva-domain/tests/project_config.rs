use minerva_domain::{
    AgentPromptMode, MinervaError, ProjectConfig, TaskPriority, TaskTag,
};
use std::collections::BTreeSet;

#[test]
fn project_config_accepts_supported_editor_override() {
    let config = ProjectConfig::new(ProjectConfig {
        schema_version: 1,
        editor: Some("zed --wait".into()),
        default_priority: TaskPriority::High,
        default_tags: BTreeSet::from([TaskTag::new("release").unwrap()]),
        agent_prompt_mode: AgentPromptMode::Exploration,
    })
    .unwrap();
    assert_eq!(config.editor.as_deref(), Some("zed --wait"));
    assert_eq!(config.default_priority, TaskPriority::High);
    assert_eq!(config.agent_prompt_mode, AgentPromptMode::Exploration);
}

#[test]
fn project_config_rejects_zero_schema_and_blank_editor() {
    let zero = ProjectConfig::new(ProjectConfig {
        schema_version: 0,
        editor: Some("zed".into()),
        default_priority: TaskPriority::Medium,
        default_tags: BTreeSet::new(),
        agent_prompt_mode: AgentPromptMode::Static,
    });
    let blank = ProjectConfig::new(ProjectConfig {
        schema_version: 1,
        editor: Some("   ".into()),
        default_priority: TaskPriority::Medium,
        default_tags: BTreeSet::new(),
        agent_prompt_mode: AgentPromptMode::Static,
    });
    assert!(
        matches!(zero, Err(MinervaError::InvalidConfiguration { key, .. }) if key == "schema_version")
    );
    assert!(
        matches!(blank, Err(MinervaError::InvalidConfiguration { key, .. }) if key == "editor")
    );
}
