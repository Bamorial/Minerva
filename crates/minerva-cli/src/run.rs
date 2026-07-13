use crate::{
    cli::{Cli, Command, ShowArgs},
    list_command, new_command, output,
    response::CommandOutput,
    show_output, status_command, tree_command,
};
use minerva_application::{
    ProjectInstructionService, ProjectRepository, RebuildAction, RebuildResult,
    RebuildService, TaskDeclarationService, TaskInstructionService, TaskShowOptions,
    TaskShowService, render_cli,
};
use minerva_storage::{FilesystemProjectRepository, FilesystemTaskRepository};
use std::{env, process::ExitCode};

pub fn run(cli: Cli) -> ExitCode {
    match execute(&cli) {
        Ok(output_text) => output::success(&cli, output_text),
        Err(Failure::Domain(error)) => {
            output::domain(&cli, render_cli(&error), error.code())
        }
        Err(Failure::Rebuild(result, dry_run)) => {
            output::rebuild(&cli, &render_rebuild(&result, dry_run), &result)
        }
        Err(Failure::Internal(message)) => output::internal(&message),
    }
}

fn execute(cli: &Cli) -> Result<CommandOutput, Failure> {
    let root = cli.root.clone().map_or_else(
        || env::current_dir().map_err(|err| Failure::Internal(err.to_string())),
        Ok,
    )?;
    let project_repo = FilesystemProjectRepository;
    let task_repo = FilesystemTaskRepository;
    match &cli.command {
        Command::Init { force } => {
            let project = project_repo
                .initialize_project(&root, *force)
                .map_err(Failure::Domain)?;
            Ok(CommandOutput::text(format!("initialized Minerva in {}", project.name)))
        }
        Command::New(args) => {
            new_command::execute(&project_repo, &task_repo, &root, args)
                .map_err(Failure::Domain)
        }
        Command::List(args) => {
            list_command::execute(&project_repo, &task_repo, &root, args)
                .map_err(Failure::Domain)
        }
        Command::Tree(args) => {
            tree_command::execute(&project_repo, &task_repo, &root, args)
                .map_err(Failure::Domain)
        }
        Command::Show(args) => {
            show(&project_repo, &task_repo, &root, args).map_err(Failure::Domain)
        }
        Command::Status(args) => {
            status_command::set(&project_repo, &task_repo, &root, args)
                .map_err(Failure::Domain)
        }
        Command::Complete(args) => {
            status_command::complete(&project_repo, &task_repo, &root, &args.task_ref)
                .map_err(Failure::Domain)
        }
        Command::Reopen(args) => {
            status_command::reopen(&project_repo, &task_repo, &root, &args.task_ref)
                .map_err(Failure::Domain)
        }
        Command::Instruction { task_ref: None } => {
            ProjectInstructionService::edit(&project_repo, &root)
                .map(|path| CommandOutput::text(format!("opened {}", path.display())))
                .map_err(Failure::Domain)
        }
        Command::Instruction { task_ref: Some(task_ref) } => {
            TaskInstructionService::edit(&project_repo, &task_repo, &root, task_ref)
                .map(|path| CommandOutput::text(format!("opened {}", path.display())))
                .map_err(Failure::Domain)
        }
        Command::Declaration { task_ref } => {
            TaskDeclarationService::edit(&project_repo, &task_repo, &root, task_ref)
                .map(|path| CommandOutput::text(format!("opened {}", path.display())))
                .map_err(Failure::Domain)
        }
        Command::Rebuild { dry_run } => {
            let result =
                RebuildService::run(&project_repo, &task_repo, &root, *dry_run)
                    .map_err(Failure::Domain)?;
            if result.has_errors() {
                Err(Failure::Rebuild(result, *dry_run))
            } else {
                Ok(CommandOutput::text(render_rebuild(&result, *dry_run)))
            }
        }
    }
}

fn show(
    project_repo: &impl ProjectRepository,
    task_repo: &FilesystemTaskRepository,
    root: &std::path::Path,
    args: &ShowArgs,
) -> Result<CommandOutput, minerva_domain::MinervaError> {
    TaskShowService::show(
        project_repo,
        task_repo,
        root,
        &args.task_ref,
        &TaskShowOptions {
            include_instructions: args.instructions,
            include_declaration: args.declaration,
        },
    )
    .map(show_output::render)
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
