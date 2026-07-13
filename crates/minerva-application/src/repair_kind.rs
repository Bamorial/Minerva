#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RepairKind {
    DerivedIndex,
    LayoutDirectory,
    TaskNotes,
    TemporaryFile,
}
