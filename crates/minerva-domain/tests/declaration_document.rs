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
fn content_hash_is_stable_for_identical_contents() {
    let declaration = DeclarationDocument::template();
    assert_eq!(
        DeclarationDocument::content_hash(&declaration),
        DeclarationDocument::content_hash(&declaration)
    );
}
