use minerva_application::TaskShowLink;

pub fn section(title: &str, body: Vec<String>) -> Vec<ratatui::text::Line<'static>> {
    std::iter::once(ratatui::text::Line::from(title.to_string()))
        .chain(
            body.into_iter().map(|line| ratatui::text::Line::from(format!("  {line}"))),
        )
        .chain(std::iter::once(ratatui::text::Line::from(String::new())))
        .collect()
}

pub fn list_or_message(values: Vec<String>, empty: &str) -> Vec<String> {
    if values.is_empty() { vec![empty.into()] } else { values }
}

pub fn link(value: Option<&TaskShowLink>) -> String {
    value.map_or_else(|| "No parent.".into(), label)
}

pub fn label(value: &TaskShowLink) -> String {
    format!("{} {}", value.id, value.title)
}

pub fn list(label: &str, values: &[String]) -> String {
    format!(
        "{label}: {}",
        if values.is_empty() { "none".into() } else { values.join(", ") }
    )
}

pub const fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

pub const fn priority(value: minerva_domain::TaskPriority) -> &'static str {
    match value {
        minerva_domain::TaskPriority::Low => "low",
        minerva_domain::TaskPriority::Medium => "medium",
        minerva_domain::TaskPriority::High => "high",
        minerva_domain::TaskPriority::Urgent => "urgent",
    }
}
