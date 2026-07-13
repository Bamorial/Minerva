use minerva_domain::{
    DeclarationFreshness, DeclarationFreshnessProbe, DeclarationFreshnessReason,
};
use std::time::{Duration, UNIX_EPOCH};

#[test]
fn missing_covered_commit_is_potentially_stale() {
    let report = probe().evaluate();
    assert_eq!(report.status, DeclarationFreshness::PotentiallyStale);
    assert_eq!(report.reasons, vec![DeclarationFreshnessReason::MissingCoveredCommit]);
}

#[test]
fn unavailable_current_commit_is_unknown() {
    let mut probe = probe();
    probe.covered_commit_hash = Some("abc123".into());
    let report = probe.evaluate();
    assert_eq!(report.status, DeclarationFreshness::Unknown);
    assert_eq!(
        report.reasons,
        vec![DeclarationFreshnessReason::CoveredCommitUnavailable]
    );
}

#[test]
fn mismatched_commit_is_stale() {
    let mut probe = probe();
    probe.covered_commit_hash = Some("abc123".into());
    probe.current_commit_hash = Some("def456".into());
    let report = probe.evaluate();
    assert_eq!(report.status, DeclarationFreshness::Stale);
    assert_eq!(report.reasons, vec![DeclarationFreshnessReason::CoveredCommitDiffers]);
}

#[test]
fn instructions_and_relationships_changes_are_stale() {
    let mut probe = probe();
    probe.covered_commit_hash = Some("abc123".into());
    probe.current_commit_hash = Some("abc123".into());
    probe.task_updated_at = UNIX_EPOCH + Duration::from_secs(12);
    probe.instructions_updated_at = Some(UNIX_EPOCH + Duration::from_secs(11));
    probe.relationships_updated_at = Some(UNIX_EPOCH + Duration::from_secs(12));
    let report = probe.evaluate();
    assert_eq!(report.status, DeclarationFreshness::Stale);
    assert_eq!(
        report.reasons,
        vec![
            DeclarationFreshnessReason::InstructionsUpdatedAfterDeclaration,
            DeclarationFreshnessReason::RelationshipsUpdatedAfterDeclaration,
        ]
    );
}

#[test]
fn task_metadata_change_without_other_file_changes_is_stale() {
    let mut probe = probe();
    probe.covered_commit_hash = Some("abc123".into());
    probe.current_commit_hash = Some("abc123".into());
    probe.task_updated_at = UNIX_EPOCH + Duration::from_secs(11);
    let report = probe.evaluate();
    assert_eq!(report.status, DeclarationFreshness::Stale);
    assert_eq!(
        report.reasons,
        vec![DeclarationFreshnessReason::TaskMetadataUpdatedAfterDeclaration]
    );
}

fn probe() -> DeclarationFreshnessProbe {
    DeclarationFreshnessProbe {
        declaration_updated_at: UNIX_EPOCH + Duration::from_secs(10),
        task_updated_at: UNIX_EPOCH + Duration::from_secs(10),
        instructions_updated_at: Some(UNIX_EPOCH + Duration::from_secs(10)),
        relationships_updated_at: None,
        covered_commit_hash: None,
        current_commit_hash: None,
    }
}
