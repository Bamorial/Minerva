mod archive_state;
mod context_detail;
mod context_policy;
mod context_policy_validation;
mod context_relation_policy;
mod declaration_actor;
mod declaration_document;
mod declaration_freshness;
mod declaration_metadata;
mod error;
mod error_code;
mod error_detail;
mod error_details;
mod event_id;
mod hierarchy;
mod identifier_error;
mod project;
mod project_config;
mod project_id;
mod project_status;
mod project_task_type;
mod relationship;
mod relationship_graph;
mod relationship_id;
mod relationship_type;
mod task;
mod task_event_kind;
mod task_facts;
mod task_id;
mod task_identity;
mod task_priority;
mod task_resources;
mod task_slug;
mod task_tag;
mod task_transition;
mod task_version;

pub use archive_state::ArchiveState;
pub use context_detail::ContextDetail;
pub use context_policy::ContextPolicy;
pub use context_relation_policy::ContextRelationPolicy;
pub use declaration_actor::DeclarationActor;
pub use declaration_document::DeclarationDocument;
pub use declaration_freshness::{
    DeclarationFreshness, DeclarationFreshnessProbe, DeclarationFreshnessReason,
    DeclarationFreshnessReport,
};
pub use declaration_metadata::DeclarationMetadata;
pub use error::MinervaError;
pub use error_code::ErrorCode;
pub use error_detail::{ErrorDetail, ErrorValue};
pub use event_id::EventId;
pub use hierarchy::validate_task_hierarchy;
pub use identifier_error::IdentifierError;
pub use project::Project;
pub use project_config::ProjectConfig;
pub use project_id::ProjectId;
pub use project_status::{StatusDefinition, StatusKey, StatusTransition};
pub use project_task_type::{TaskTypeDefinition, TaskTypeKey};
pub use relationship::Relationship;
pub use relationship_graph::validate_relationships;
pub use relationship_id::RelationshipId;
pub use relationship_type::RelationshipType;
pub use task::Task;
pub use task_event_kind::TaskEventKind;
pub use task_facts::TaskFacts;
pub use task_id::{TaskId, TaskIdAllocator};
pub use task_identity::TaskIdentity;
pub use task_priority::TaskPriority;
pub use task_resources::TaskResources;
pub use task_slug::TaskSlug;
pub use task_tag::TaskTag;
pub use task_transition::{TaskTransitionOutcome, TaskTransitionService};
pub use task_version::TaskVersion;

pub const WORKSPACE_CRATES: [&str; 7] = [
    "minerva-domain",
    "minerva-application",
    "minerva-storage",
    "minerva-context",
    "minerva-cli",
    "minerva-tui",
    "minerva-mcp",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceKind {
    Cli,
    Tui,
    Mcp,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceBlueprint {
    crates: &'static [&'static str],
}

impl WorkspaceBlueprint {
    #[must_use]
    pub const fn new() -> Self {
        Self { crates: &WORKSPACE_CRATES }
    }

    #[must_use]
    pub const fn crates(self) -> &'static [&'static str] {
        self.crates
    }
}

impl Default for WorkspaceBlueprint {
    fn default() -> Self {
        Self::new()
    }
}
