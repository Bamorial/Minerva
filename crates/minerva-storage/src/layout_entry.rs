#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutClass {
    Canonical,
    Derived,
    Operational,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutEntry {
    pub relative_path: &'static str,
    pub class: LayoutClass,
    pub description: &'static str,
}

impl LayoutEntry {
    #[must_use]
    pub const fn new(
        relative_path: &'static str,
        class: LayoutClass,
        description: &'static str,
    ) -> Self {
        Self { relative_path, class, description }
    }
}
