use minerva_domain::Task;

#[must_use]
pub fn render_task_facts(task: &Task) -> String {
    let facts = &task.facts;
    [
        format!("facts.modules: {}", render_list(&facts.modules)),
        format!("facts.files: {}", render_list(&facts.files)),
        format!("facts.migrations_required: {}", facts.migrations_required),
        format!("facts.feature_flags: {}", render_list(&facts.feature_flags)),
        format!("facts.acceptance_checks: {}", render_list(&facts.acceptance_checks)),
        format!("facts.resources.reads: {}", render_list(&facts.resources.reads)),
        format!("facts.resources.writes: {}", render_list(&facts.resources.writes)),
    ]
    .join("\n")
}

fn render_list(values: &[String]) -> String {
    if values.is_empty() { "none".into() } else { values.join(", ") }
}
