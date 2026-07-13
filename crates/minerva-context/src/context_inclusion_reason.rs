use minerva_domain::RelationshipType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ContextRelationshipDirection {
    Incoming,
    Outgoing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextInclusionReason {
    Target,
    Ancestor {
        depth: u8,
    },
    Dependency {
        depth: u8,
        relationship_type: RelationshipType,
    },
    RelatedTask {
        depth: u8,
        relationship_type: RelationshipType,
        direction: ContextRelationshipDirection,
    },
    Child {
        depth: u8,
    },
    Sibling {
        depth: u8,
    },
}
