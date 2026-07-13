use crate::MinervaLayout;
use minerva_application::RepairResult;
use minerva_domain::MinervaError;

pub fn repair_project_state(
    root: &std::path::Path,
    dry_run: bool,
) -> Result<RepairResult, MinervaError> {
    let layout = MinervaLayout::new(root);
    let mut operations = crate::safe_repair_directories::repair(&layout, dry_run)?;
    operations.extend(crate::safe_repair_notes::repair(&layout, dry_run)?);
    operations.extend(crate::safe_repair_temp_files::repair(&layout, dry_run)?);
    let (index, issues) = crate::safe_repair_task_index::repair(&layout, dry_run)?;
    operations.extend(index);
    Ok(RepairResult { operations, issues })
}
