use crate::{
    ContextPolicy, MinervaError, ProjectId, StatusDefinition, StatusKey,
    StatusTransition, TaskTypeKey,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Project {
    pub schema_version: u32,
    pub id: ProjectId,
    pub name: String,
    pub created_at: SystemTime,
    pub default_task_type: TaskTypeKey,
    pub default_status: StatusKey,
    pub statuses: Vec<StatusDefinition>,
    pub transitions: Vec<StatusTransition>,
    pub context_policy: ContextPolicy,
}

impl Project {
    pub fn new(project: Self) -> Result<Self, MinervaError> {
        project.validate()?;
        Ok(project)
    }

    pub fn validate(&self) -> Result<(), MinervaError> {
        if self.schema_version == 0 {
            return invalid("schema_version", "must be greater than zero");
        }
        if self.name.trim().is_empty() {
            return invalid("name", "must not be empty");
        }
        self.context_policy.validate()?;
        let statuses = status_set(&self.statuses)?;
        if !statuses.contains(&self.default_status) {
            return invalid("default_status", "must exist in configured statuses");
        }
        for transition in &self.transitions {
            validate_transition_endpoint(
                "transitions.from",
                &transition.from,
                &statuses,
            )?;
            validate_transition_endpoint("transitions.to", &transition.to, &statuses)?;
        }
        Ok(())
    }

    #[must_use]
    pub fn can_transition(&self, from: &StatusKey, to: &StatusKey) -> bool {
        self.transitions
            .iter()
            .any(|transition| &transition.from == from && &transition.to == to)
    }
}

fn status_set(
    statuses: &[StatusDefinition],
) -> Result<HashSet<StatusKey>, MinervaError> {
    let mut seen = HashSet::new();
    for status in statuses {
        if !seen.insert(status.key.clone()) {
            return invalid("statuses", "contains duplicate status keys");
        }
    }
    Ok(seen)
}

fn validate_transition_endpoint(
    key: &str,
    status: &StatusKey,
    allowed: &HashSet<StatusKey>,
) -> Result<(), MinervaError> {
    if allowed.contains(status) {
        return Ok(());
    }
    invalid(key, &format!("references unknown status `{status}`"))
}

fn invalid<T>(key: &str, reason: &str) -> Result<T, MinervaError> {
    Err(MinervaError::InvalidConfiguration { key: key.into(), reason: reason.into() })
}
