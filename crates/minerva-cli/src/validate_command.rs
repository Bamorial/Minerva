use crate::{cli::ValidateArgs, response::CommandOutput, validate_output};
use minerva_application::{
    ProjectRepository, ProjectValidationResult, ProjectValidationService,
    TaskRepository,
};
use minerva_domain::MinervaError;
use std::path::Path;

pub struct ValidationExecution {
    pub output: CommandOutput,
    pub result: ProjectValidationResult,
}

pub fn execute(
    project_repo: &impl ProjectRepository,
    task_repo: &impl TaskRepository,
    root: &Path,
    args: &ValidateArgs,
) -> Result<ValidationExecution, MinervaError> {
    let result = ProjectValidationService::run(project_repo, task_repo, root)?;
    let result = match args.task_ref.as_deref() {
        Some(task_ref) => {
            task_repo.resolve_task(root, task_ref)?;
            result.for_task(task_ref)
        }
        None => result,
    };
    let output = validate_output::render(&result, args.task_ref.as_deref());
    Ok(ValidationExecution { output, result })
}
