use minerva_domain::{DeclarationDocument, MinervaError};

#[test]
fn template_contains_all_required_sections() {
    let declaration = DeclarationDocument::template();
    DeclarationDocument::parse(&declaration).unwrap();
    assert!(declaration.contains("## Objective"));
    assert!(declaration.contains("## Open Questions"));
}

#[test]
fn parser_rejects_missing_required_heading() {
    let declaration =
        DeclarationDocument::template().replace("## Remaining Work\n", "");
    let error = DeclarationDocument::parse(&declaration).unwrap_err();
    assert!(matches!(
        error,
        MinervaError::InvalidConfiguration { key, reason }
            if key == "declaration.heading"
                && reason.contains("## Remaining Work")
    ));
}

#[test]
fn completion_validation_rejects_missing_required_content() {
    let error =
        DeclarationDocument::validate_completion(&DeclarationDocument::template())
            .unwrap_err();
    assert!(matches!(
        error,
        MinervaError::InvalidConfiguration { key, reason }
            if key == "declaration.completion"
                && reason.contains("Current State or Final State")
                && reason.contains("Completed Work")
                && reason.contains("Verification")
    ));
}

#[test]
fn completion_validation_accepts_filled_required_sections() {
    let declaration = DeclarationDocument::template()
        .replace("## Current State\n", "## Current State\nShipped.\n")
        .replace("## Completed Work\n", "## Completed Work\nImplemented validation.\n")
        .replace("## Verification\n", "## Verification\ncargo test\n");
    DeclarationDocument::validate_completion(&declaration).unwrap();
}

#[test]
fn content_hash_is_stable_for_identical_contents() {
    let declaration = DeclarationDocument::template();
    assert_eq!(
        DeclarationDocument::content_hash(&declaration),
        DeclarationDocument::content_hash(&declaration)
    );
}

#[test]
fn empty_template_is_detected_as_effectively_empty() {
    assert!(
        DeclarationDocument::is_effectively_empty(&DeclarationDocument::template())
    );
    let filled = DeclarationDocument::template()
        .replace("## Current State\n", "## Current State\nImplemented.\n");
    assert!(!DeclarationDocument::is_effectively_empty(&filled));
}
