use minerva_domain::{MinervaError, TaskTypeDefinition, TaskTypeKey};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TaskTypeDocument {
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub declaration_requirements: Vec<String>,
}

pub fn parse_task_type(
    source: &str,
    contents: &str,
) -> Result<TaskTypeDefinition, MinervaError> {
    let (front_matter, template) = split_front_matter(contents)?;
    let doc = serde_yaml::from_str::<TaskTypeDocument>(front_matter)
        .map_err(|err| invalid(source, &err.to_string()))?;
    TaskTypeDefinition::new(TaskTypeDefinition {
        name: doc
            .name
            .parse::<TaskTypeKey>()
            .map_err(|err| invalid(source, &err.to_string()))?,
        display_name: doc.display_name,
        description: doc.description,
        declaration_requirements: doc.declaration_requirements,
        instruction_template: template.into(),
    })
    .map_err(|err| invalid(source, &err.to_string()))
}

fn split_front_matter(contents: &str) -> Result<(&str, &str), MinervaError> {
    let rest = contents
        .strip_prefix("---\n")
        .ok_or_else(|| invalid("task type", "missing front matter"))?;
    let (front_matter, body) = rest
        .split_once("\n---\n")
        .ok_or_else(|| invalid("task type", "missing front matter terminator"))?;
    Ok((front_matter, body))
}

fn invalid(source: &str, reason: &str) -> MinervaError {
    MinervaError::InvalidConfiguration {
        key: format!("task_types.{source}"),
        reason: reason.into(),
    }
}
