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
}
