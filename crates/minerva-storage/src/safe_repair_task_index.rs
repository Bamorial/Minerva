use crate::MinervaLayout;
use minerva_application::{
    RebuildAction, RebuildTaskError, RepairAction, RepairIssue, RepairKind,
    RepairOperation, RepairSafety,
};
use minerva_domain::MinervaError;

pub fn repair(
    layout: &MinervaLayout,
    dry_run: bool,
) -> Result<(Vec<RepairOperation>, Vec<RepairIssue>), MinervaError> {
    let result = crate::task_index_rebuild::rebuild_task_index(layout, dry_run)?;
    let operations = match result.index_action {
        RebuildAction::NoChange => Vec::new(),
        RebuildAction::Create | RebuildAction::Update => vec![RepairOperation {
            kind: RepairKind::DerivedIndex,
            safety: RepairSafety::Safe,
            action: action(result.index_action),
            path: result.index_path,
            backup_path: None,
            message: "rebuilt derived task index".into(),
        }],
    };
    Ok((operations, result.task_errors.into_iter().map(issue).collect()))
}

const fn action(value: RebuildAction) -> RepairAction {
    match value {
        RebuildAction::Create => RepairAction::Create,
        RebuildAction::Update | RebuildAction::NoChange => RepairAction::Update,
    }
}

fn issue(error: RebuildTaskError) -> RepairIssue {
    RepairIssue {
        code: "invalid_task".into(),
        path: error.path,
        task_ref: Some(error.task_ref),
        message: error.reason,
    }
}
