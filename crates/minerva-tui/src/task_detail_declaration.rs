use minerva_domain::DeclarationDocument;

pub fn declaration_summary(value: Option<&str>) -> Vec<String> {
    let Some(value) = value else {
        return vec!["Declaration unavailable.".into()];
    };
    if let Err(error) = DeclarationDocument::parse(value) {
        return vec![format!("Invalid declaration: {error}")];
    }
    summary_lines(value).unwrap_or_else(|| {
        vec!["Declaration is present but summary sections are empty.".into()]
    })
}

fn summary_lines(value: &str) -> Option<Vec<String>> {
    ["Current State", "Completed Work", "Objective", "Remaining Work"]
        .into_iter()
        .find_map(|section| section_body(value, section).map(compact_lines))
        .filter(|lines| !lines.is_empty())
}

fn compact_lines(value: &str) -> Vec<String> {
    value
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(cleanup_bullet)
        .take(3)
        .collect()
}

fn cleanup_bullet(line: &str) -> String {
    line.strip_prefix("- ").unwrap_or(line).trim().into()
}

fn section_body<'a>(value: &'a str, section: &str) -> Option<&'a str> {
    let heading = format!("## {section}");
    let start = value.find(&heading)?;
    let body = &value[start + heading.len()..];
    let end = body.find("\n## ").unwrap_or(body.len());
    let body = body[..end].trim();
    (!body.is_empty()).then_some(body)
}
