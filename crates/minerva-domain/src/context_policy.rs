use crate::{ContextDetail, ContextRelationPolicy, MinervaError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextPolicy {
    pub project_instructions: Option<ContextDetail>,
    pub target_task_instructions: Option<ContextDetail>,
    pub target_declaration: Option<ContextDetail>,
    pub ancestors: Option<ContextRelationPolicy>,
    pub dependencies: Option<ContextRelationPolicy>,
    pub related_tasks: Option<ContextRelationPolicy>,
    pub children: Option<ContextRelationPolicy>,
    pub siblings: Option<ContextRelationPolicy>,
    pub include_archived: bool,
    pub include_completed: bool,
}

impl ContextPolicy {
    pub fn new(policy: Self) -> Result<Self, MinervaError> {
        policy.validate()?;
        Ok(policy)
    }

    #[must_use]
    pub fn strict() -> Self {
        Self {
            project_instructions: Some(ContextDetail::Full),
            target_task_instructions: Some(ContextDetail::Full),
            target_declaration: Some(ContextDetail::Full),
            ancestors: Some(ContextRelationPolicy {
                detail: ContextDetail::Summary,
                depth: 1,
            }),
            dependencies: Some(ContextRelationPolicy {
                detail: ContextDetail::Summary,
                depth: 1,
            }),
            related_tasks: None,
            children: None,
            siblings: None,
            include_archived: false,
            include_completed: false,
        }
    }

    pub fn validate(&self) -> Result<(), MinervaError> {
        crate::context_policy_validation::validate(self)
    }
}

impl Default for ContextPolicy {
    fn default() -> Self {
        Self::strict()
    }
}
