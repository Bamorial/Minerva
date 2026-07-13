use minerva_domain::{MinervaError, Task, TaskId, TaskTypeDefinition, TaskTypeKey};
use std::io::{self, Write};

pub fn prompt(label: &str) -> Result<String, MinervaError> {
    let value = prompt_optional(label)?;
    if value.is_empty() {
        return Err(MinervaError::InvalidConfiguration {
            key: "interactive".into(),
            reason: format!("missing input for `{label}`"),
        });
    }
    Ok(value)
}

pub fn prompt_optional(label: &str) -> Result<String, MinervaError> {
    print!("{label}");
    io::stdout().flush().map_err(|err| io_error(&err))?;
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|err| io_error(&err))?;
    Ok(input.trim().to_string())
}

pub fn choose_type(
    task_types: &[TaskTypeDefinition],
    default: &TaskTypeKey,
) -> Result<Option<TaskTypeKey>, MinervaError> {
    println!("Task type:");
    for (index, task_type) in task_types.iter().enumerate() {
        let default_marker = if task_type.name == *default { " (default)" } else { "" };
        println!("  {}. {}{}", index + 1, task_type.name, default_marker);
    }
    let choice = prompt_optional("Select task type by number (Enter for default): ")?;
    if choice.is_empty() {
        return Ok(None);
    }
    let index = choice.parse::<usize>().map_err(|_| invalid("task_type"))?;
    task_types
        .get(index.saturating_sub(1))
        .map(|task_type| Some(task_type.name.clone()))
        .ok_or_else(|| invalid("task_type"))
}

pub fn choose_parent(query: &str, matches: &[Task]) -> Result<TaskId, MinervaError> {
    if matches.len() == 1 {
        return Ok(matches[0].id);
    }
    println!("Parent matches for `{query}`:");
    for (index, task) in matches.iter().enumerate() {
        println!("  {}. {} {}", index + 1, task.id, task.title);
    }
    let choice = prompt("Select parent by number: ")?;
    let index = choice.parse::<usize>().map_err(|_| invalid("parent"))?;
    matches
        .get(index.saturating_sub(1))
        .map(|task| task.id)
        .ok_or_else(|| invalid("parent"))
}

fn invalid(key: &str) -> MinervaError {
    MinervaError::InvalidConfiguration {
        key: key.into(),
        reason: "invalid interactive selection".into(),
    }
}

fn io_error(err: &io::Error) -> MinervaError {
    MinervaError::InvalidConfiguration {
        key: "interactive".into(),
        reason: err.to_string(),
    }
}
