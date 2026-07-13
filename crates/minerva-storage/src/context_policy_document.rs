use minerva_domain::{
    ContextDetail, ContextPolicy, ContextRelationPolicy, MinervaError,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContextPolicyDocument {
    pub project_instructions: Option<ContextDetail>,
    pub target_task_instructions: Option<ContextDetail>,
    pub target_declaration: Option<ContextDetail>,
    pub ancestors: Option<ContextRelationPolicyDocument>,
    pub dependencies: Option<ContextRelationPolicyDocument>,
    pub related_tasks: Option<ContextRelationPolicyDocument>,
    pub children: Option<ContextRelationPolicyDocument>,
    pub siblings: Option<ContextRelationPolicyDocument>,
    pub include_archived: bool,
    pub include_completed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContextRelationPolicyDocument {
    pub detail: ContextDetail,
    pub depth: u8,
}

impl TryFrom<ContextPolicyDocument> for ContextPolicy {
    type Error = MinervaError;

    fn try_from(doc: ContextPolicyDocument) -> Result<Self, Self::Error> {
        ContextPolicy::new(ContextPolicy {
            project_instructions: doc.project_instructions,
            target_task_instructions: doc.target_task_instructions,
            target_declaration: doc.target_declaration,
            ancestors: doc.ancestors.map(Into::into),
            dependencies: doc.dependencies.map(Into::into),
            related_tasks: doc.related_tasks.map(Into::into),
            children: doc.children.map(Into::into),
            siblings: doc.siblings.map(Into::into),
            include_archived: doc.include_archived,
            include_completed: doc.include_completed,
        })
    }
}

impl From<&ContextPolicy> for ContextPolicyDocument {
    fn from(policy: &ContextPolicy) -> Self {
        Self {
            project_instructions: policy.project_instructions,
            target_task_instructions: policy.target_task_instructions,
            target_declaration: policy.target_declaration,
            ancestors: policy.ancestors.clone().map(Into::into),
            dependencies: policy.dependencies.clone().map(Into::into),
            related_tasks: policy.related_tasks.clone().map(Into::into),
            children: policy.children.clone().map(Into::into),
            siblings: policy.siblings.clone().map(Into::into),
            include_archived: policy.include_archived,
            include_completed: policy.include_completed,
        }
    }
}

impl From<ContextRelationPolicyDocument> for ContextRelationPolicy {
    fn from(doc: ContextRelationPolicyDocument) -> Self {
        Self { detail: doc.detail, depth: doc.depth }
    }
}

impl From<ContextRelationPolicy> for ContextRelationPolicyDocument {
    fn from(policy: ContextRelationPolicy) -> Self {
        Self { detail: policy.detail, depth: policy.depth }
    }
}
