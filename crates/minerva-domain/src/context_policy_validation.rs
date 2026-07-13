use crate::{ContextPolicy, ContextRelationPolicy, MinervaError};

pub fn validate(policy: &ContextPolicy) -> Result<(), MinervaError> {
    validate_relation(&policy.ancestors, "ancestors")?;
    validate_relation(&policy.dependencies, "dependencies")?;
    validate_relation(&policy.related_tasks, "related_tasks")?;
    validate_relation(&policy.children, "children")?;
    validate_relation(&policy.siblings, "siblings")?;
    validate_sources(policy)?;
    validate_filters(policy)
}

fn validate_relation(
    policy: &Option<ContextRelationPolicy>,
    key: &str,
) -> Result<(), MinervaError> {
    policy.as_ref().map_or(Ok(()), |value| value.validate(key))
}

fn validate_sources(policy: &ContextPolicy) -> Result<(), MinervaError> {
    if has_any(&[
        policy.project_instructions.is_some(),
        policy.target_task_instructions.is_some(),
        policy.target_declaration.is_some(),
        policy.ancestors.is_some(),
        policy.dependencies.is_some(),
        policy.related_tasks.is_some(),
        policy.children.is_some(),
        policy.siblings.is_some(),
    ]) {
        return Ok(());
    }
    Err(MinervaError::InvalidConfiguration {
        key: "context_policy".into(),
        reason: "must include at least one context source".into(),
    })
}

fn validate_filters(policy: &ContextPolicy) -> Result<(), MinervaError> {
    if has_any_related_scope(policy)
        || (!policy.include_archived && !policy.include_completed)
    {
        return Ok(());
    }
    Err(MinervaError::InvalidConfiguration {
        key: filter_key(policy).into(),
        reason: "task-state filters require at least one related task scope".into(),
    })
}

fn has_any(values: &[bool]) -> bool {
    values.iter().copied().any(std::convert::identity)
}

fn has_any_related_scope(policy: &ContextPolicy) -> bool {
    has_any(&[
        policy.ancestors.is_some(),
        policy.dependencies.is_some(),
        policy.related_tasks.is_some(),
        policy.children.is_some(),
        policy.siblings.is_some(),
    ])
}

fn filter_key(policy: &ContextPolicy) -> &'static str {
    if policy.include_archived {
        "context_policy.include_archived"
    } else {
        "context_policy.include_completed"
    }
}
