use crate::{cli::LogArgs, log_output, response::CommandOutput};
use minerva_application::{ProjectRepository, TaskLogService, TaskRepository};
use minerva_domain::MinervaError;
use std::path::Path;

pub fn execute(
    project_repo: &impl ProjectRepository,
    task_repo: &impl TaskRepository,
    root: &Path,
    args: &LogArgs,
) -> Result<CommandOutput, MinervaError> {
    TaskLogService::show(project_repo, task_repo, root, &args.task_ref, &args.kinds())
        .map(|result| log_output::render(&result))
}
