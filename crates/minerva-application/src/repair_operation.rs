use crate::{RepairAction, RepairKind, RepairSafety};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RepairOperation {
    pub kind: RepairKind,
    pub safety: RepairSafety,
    pub action: RepairAction,
    pub path: String,
    pub backup_path: Option<String>,
    pub message: String,
}
