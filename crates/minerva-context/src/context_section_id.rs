#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ContextSectionId {
    MinervaExecutionContract,
    ProjectInstructions,
    TargetMetadataAndFacts,
    AncestorInstructions,
    AncestorDeclarations,
    TargetInstructions,
    TargetDeclaration,
    DependencyDeclarations,
    RelatedTaskSummaries,
    OutputRequirements,
    ContextManifestSummary,
}

impl ContextSectionId {
    #[must_use]
    pub const fn heading(self) -> &'static str {
        match self {
            Self::MinervaExecutionContract => "Minerva Execution Contract",
            Self::ProjectInstructions => "Project Instructions",
            Self::TargetMetadataAndFacts => "Target Metadata and Facts",
            Self::AncestorInstructions => "Ancestor Instructions",
            Self::AncestorDeclarations => "Ancestor Declarations",
            Self::TargetInstructions => "Target Instructions",
            Self::TargetDeclaration => "Target Declaration",
            Self::DependencyDeclarations => "Dependency Declarations",
            Self::RelatedTaskSummaries => "Related Task Summaries",
            Self::OutputRequirements => "Output Requirements",
            Self::ContextManifestSummary => "Context Manifest Summary",
        }
    }

    #[must_use]
    pub const fn source_key(self) -> &'static str {
        match self {
            Self::MinervaExecutionContract => "minerva_execution_contract",
            Self::ProjectInstructions => "project_instructions",
            Self::TargetMetadataAndFacts => "target_metadata_and_facts",
            Self::AncestorInstructions => "ancestor_instructions",
            Self::AncestorDeclarations => "ancestor_declarations",
            Self::TargetInstructions => "target_instructions",
            Self::TargetDeclaration => "target_declaration",
            Self::DependencyDeclarations => "dependency_declarations",
            Self::RelatedTaskSummaries => "related_task_summaries",
            Self::OutputRequirements => "output_requirements",
            Self::ContextManifestSummary => "context_manifest_summary",
        }
    }

    #[must_use]
    pub const fn inclusion_reason(self) -> &'static str {
        match self {
            Self::MinervaExecutionContract
            | Self::ProjectInstructions
            | Self::OutputRequirements => "always_required",
            Self::TargetMetadataAndFacts
            | Self::TargetInstructions
            | Self::TargetDeclaration => "target_task",
            Self::AncestorInstructions | Self::AncestorDeclarations => {
                "ancestor_context"
            }
            Self::DependencyDeclarations => "dependency_context",
            Self::RelatedTaskSummaries => "related_task_context",
            Self::ContextManifestSummary => "manifest_output",
        }
    }

    #[must_use]
    pub const fn budget_priority(self) -> u8 {
        match self {
            Self::ContextManifestSummary => 0,
            Self::RelatedTaskSummaries => 1,
            Self::AncestorDeclarations => 2,
            Self::AncestorInstructions => 3,
            Self::DependencyDeclarations => 4,
            Self::ProjectInstructions => 5,
            Self::OutputRequirements => 6,
            Self::TargetInstructions => 7,
            Self::TargetDeclaration => 8,
            Self::TargetMetadataAndFacts => 9,
            Self::MinervaExecutionContract => 10,
        }
    }

    #[must_use]
    pub const fn is_critical(self) -> bool {
        matches!(
            self,
            Self::MinervaExecutionContract
                | Self::ProjectInstructions
                | Self::TargetMetadataAndFacts
                | Self::TargetInstructions
                | Self::TargetDeclaration
                | Self::OutputRequirements
        )
    }
}
