use crate::{cli::ContextArgs, context_output, response::CommandOutput};
use minerva_application::{ProjectRepository, TaskRepository};
use minerva_context::{
    ContextCompilationError, ContextCompilationRequest, ContextCompilationService,
};
use minerva_domain::{DeclarationFreshnessReason, MinervaError};
use minerva_storage::atomic_replace;
use std::{fs, path::Path};

pub fn execute(
    project_repo: &impl ProjectRepository,
    task_repo: &impl TaskRepository,
    root: &Path,
    args: &ContextArgs,
) -> Result<CommandOutput, MinervaError> {
    let result = ContextCompilationService::compile(
        project_repo,
        task_repo,
        root,
        &ContextCompilationRequest {
            task_ref: args.task_ref.clone(),
            policy: None,
            budget: args.budget,
        },
    )
    .map_err(map_compile_error)?;
    let rendered = context_output::render(args, &result)?;
    if let Some(path) = &args.output {
        write_output(path, rendered.text.as_bytes())?;
        return Ok(CommandOutput::with_json(
            format!("wrote context to {}", path.display()),
            rendered.json,
        ));
    }
    Ok(CommandOutput::with_json(rendered.text, rendered.json))
}

fn map_compile_error(error: ContextCompilationError) -> MinervaError {
    match error {
        ContextCompilationError::Minerva(error) => error,
        ContextCompilationError::Budget(error) => MinervaError::InvalidConfiguration {
            key: "context.budget".into(),
            reason: error.to_string(),
        },
        ContextCompilationError::StaleReference { task_ref, reasons } => {
            MinervaError::InvalidConfiguration {
                key: format!("context.task.{task_ref}.declaration"),
                reason: reasons
                    .iter()
                    .copied()
                    .map(reason)
                    .collect::<Vec<_>>()
                    .join(", "),
            }
        }
    }
}

fn reason(value: DeclarationFreshnessReason) -> &'static str {
    match value {
        DeclarationFreshnessReason::MissingCoveredCommit => "missing-covered-commit",
        DeclarationFreshnessReason::CoveredCommitUnavailable => {
            "covered-commit-unavailable"
        }
        DeclarationFreshnessReason::CoveredCommitDiffers => "covered-commit-differs",
        DeclarationFreshnessReason::InstructionsUpdatedAfterDeclaration => {
            "instructions-updated-after-declaration"
        }
        DeclarationFreshnessReason::RelationshipsUpdatedAfterDeclaration => {
            "relationships-updated-after-declaration"
        }
        DeclarationFreshnessReason::TaskMetadataUpdatedAfterDeclaration => {
            "task-metadata-updated-after-declaration"
        }
    }
}

fn write_output(path: &Path, contents: &[u8]) -> Result<(), MinervaError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| schema(path, err))?;
    }
    atomic_replace(path, contents).map_err(|err| schema(path, err))
}

fn schema(path: &Path, err: impl std::fmt::Display) -> MinervaError {
    MinervaError::SchemaError {
        path: path.display().to_string(),
        reason: err.to_string(),
    }
}
