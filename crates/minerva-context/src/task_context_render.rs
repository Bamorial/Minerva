use minerva_application::render_task_facts;
use minerva_domain::Task;

#[must_use]
pub fn render_target_metadata(task: &Task) -> String {
    [
        format!("task: {} {}", task.id, task.title),
        format!("type: {}", task.task_type),
        format!("status: {}", task.status),
        format!("priority: {}", priority(task.priority)),
        render_task_facts(task),
    ]
    .join("\n")
}

#[must_use]
pub fn render_task_summary(task: &Task) -> String {
    format!(
        "{} {}\ntype: {}\nstatus: {}",
        task.id, task.title, task.task_type, task.status
    )
}

fn priority(value: minerva_domain::TaskPriority) -> &'static str {
    match value {
        minerva_domain::TaskPriority::Low => "low",
        minerva_domain::TaskPriority::Medium => "medium",
        minerva_domain::TaskPriority::High => "high",
        minerva_domain::TaskPriority::Urgent => "urgent",
    }
}
