use crate::{
    cli::{Cli, Command},
    output,
};
use minerva_application::{
    ProjectInstructionService, ProjectRepository, RebuildAction, RebuildResult,
    RebuildService, TaskDeclarationService, TaskInstructionService, TaskStatusService,
    render_cli,
};
use minerva_storage::{FilesystemProjectRepository, FilesystemTaskRepository};
use std::{env, path::PathBuf, process::ExitCode};

pub fn run(cli: Cli) -> ExitCode {
    match execute(&cli) {
        Ok(output_text) => output::success(&cli, &output_text),
        Err(Failure::Domain(error)) => {
            output::domain(&cli, render_cli(&error), error.code())
        }
        Err(Failure::Rebuild(result, dry_run)) => {
            output::rebuild(&cli, &render_rebuild(&result, dry_run), &result)
        }
        Err(Failure::Internal(message)) => output::internal(&message),
    }
}

fn execute(cli: &Cli) -> Result<String, Failure> {
    let root = root(cli)?;
    let project_repo = FilesystemProjectRepository;
    let task_repo = FilesystemTaskRepository;
    match &cli.command {
        Command::Init { force } => {
            let project = project_repo
                .initialize_project(&root, *force)
                .map_err(Failure::Domain)?;
            Ok(format!("initialized Minerva in {}", project.name))
        }
        Command::Instruction { task_ref: None } => {
            ProjectInstructionService::edit(&project_repo, &root)
                .map(|path| format!("opened {}", path.display()))
                .map_err(Failure::Domain)
        }
        Command::Instruction { task_ref: Some(task_ref) } => {
            TaskInstructionService::edit(&project_repo, &task_repo, &root, task_ref)
                .map(|path| format!("opened {}", path.display()))
                .map_err(Failure::Domain)
        }
        Command::Declaration { task_ref } => {
            TaskDeclarationService::edit(&project_repo, &task_repo, &root, task_ref)
                .map(|path| format!("opened {}", path.display()))
                .map_err(Failure::Domain)
        }
        Command::Status { task_ref } => {
            TaskStatusService::show(&project_repo, &task_repo, &root, task_ref)
                .map_err(Failure::Domain)
        }
        Command::Rebuild { dry_run } => {
            let result =
                RebuildService::run(&project_repo, &task_repo, &root, *dry_run)
                    .map_err(Failure::Domain)?;
            if result.has_errors() {
                Err(Failure::Rebuild(result, *dry_run))
            } else {
                Ok(render_rebuild(&result, *dry_run))
            }
        }
    }
}

fn root(cli: &Cli) -> Result<PathBuf, Failure> {
    cli.root.clone().map_or_else(
        || env::current_dir().map_err(|err| Failure::Internal(err.to_string())),
        Ok,
    )
}

enum Failure {
    Domain(minerva_domain::MinervaError),
    Rebuild(RebuildResult, bool),
    Internal(String),
}

fn render_rebuild(result: &RebuildResult, dry_run: bool) -> String {
    let action = match (dry_run, result.index_action) {
        (true, RebuildAction::Create | RebuildAction::Update) => "would write",
        (false, RebuildAction::Create | RebuildAction::Update) => "wrote",
        (true, RebuildAction::NoChange) => "would keep",
        (false, RebuildAction::NoChange) => "kept",
    };
    let mut lines = vec![format!("rebuild: {action} {}", result.index_path)];
    lines.extend(result.task_errors.iter().map(|error| {
        format!("invalid task {} at {}: {}", error.task_ref, error.path, error.reason)
    }));
    lines.join("\n")
}
