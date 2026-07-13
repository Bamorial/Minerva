use minerva_domain::{
    ErrorCode, ErrorValue, InterfaceKind, MinervaError, WORKSPACE_CRATES,
    WorkspaceBlueprint,
};

#[test]
fn blueprint_lists_all_workspace_crates() {
    let blueprint = WorkspaceBlueprint::new();
    assert_eq!(blueprint.crates(), &WORKSPACE_CRATES);
}

#[test]
fn interface_kinds_remain_transport_specific() {
    assert_ne!(InterfaceKind::Cli, InterfaceKind::Mcp);
}

#[test]
fn errors_expose_stable_codes_and_details() {
    let error = MinervaError::AmbiguousTaskReference {
        task_ref: "TSK-1".into(),
        matches: vec!["TSK-10".into(), "TSK-11".into()],
    };
    let details = error.details();
    assert_eq!(error.code(), ErrorCode::AmbiguousTaskReference);
    assert_eq!(error.code().as_str(), "ambiguous_task_reference");
    assert_eq!(details[0].key, "task_ref");
    assert_eq!(
        details[1].value,
        ErrorValue::List(vec!["TSK-10".into(), "TSK-11".into()])
    );
}
