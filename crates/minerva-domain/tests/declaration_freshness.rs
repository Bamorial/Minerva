use minerva_domain::{
    DeclarationFreshness, DeclarationFreshnessProbe, DeclarationFreshnessReason,
};
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
fn matching_commit_is_fresh() {
    let mut probe = probe();
    probe.covered_commit_hash = Some("abc123".into());
    probe.current_commit_hash = Some("abc123".into());
    let report = probe.evaluate();
    assert_eq!(report.status, DeclarationFreshness::Fresh);
    assert!(report.reasons.is_empty());
}

fn probe() -> DeclarationFreshnessProbe {
    DeclarationFreshnessProbe { covered_commit_hash: None, current_commit_hash: None }
}
