use minerva_domain::{
    ContextDetail, ContextPolicy, ContextRelationPolicy, MinervaError,
};

#[test]
fn strict_policy_is_valid() {
    let policy = ContextPolicy::strict();
    assert_eq!(policy.project_instructions, Some(ContextDetail::Full));
    assert_eq!(policy.target_declaration, Some(ContextDetail::Full));
    assert_eq!(policy.ancestors.as_ref().map(|item| item.depth), Some(1));
    assert!(policy.validate().is_ok());
}

#[test]
fn policy_rejects_zero_depth_and_empty_sources() {
    let zero_depth = ContextPolicy::new(ContextPolicy {
        dependencies: Some(ContextRelationPolicy {
            detail: ContextDetail::Summary,
            depth: 0,
        }),
        ..ContextPolicy::strict()
    });
    let no_sources = ContextPolicy::new(ContextPolicy {
        project_instructions: None,
        target_task_instructions: None,
        target_declaration: None,
        ancestors: None,
        dependencies: None,
        ..ContextPolicy::default()
    });
    assert!(matches!(
        zero_depth,
        Err(MinervaError::InvalidConfiguration { key, .. })
        if key == "context_policy.dependencies.depth"
    ));
    assert!(matches!(
        no_sources,
        Err(MinervaError::InvalidConfiguration { key, .. }) if key == "context_policy"
    ));
}

#[test]
fn policy_rejects_task_filters_without_related_scope() {
    let result = ContextPolicy::new(ContextPolicy {
        include_completed: true,
        ancestors: None,
        dependencies: None,
        related_tasks: None,
        children: None,
        siblings: None,
        ..ContextPolicy::strict()
    });
    assert!(matches!(
        result,
        Err(MinervaError::InvalidConfiguration { reason, .. })
        if reason.contains("related task scope")
    ));
}
