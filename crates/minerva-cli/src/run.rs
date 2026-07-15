use crate::{
    cli::{Cli, Command, ShowArgs},
    context_command, delete_command, hierarchy_command, list_command, log_command,
    migrate_output, move_command, new_command, output, relationship_command,
    repair_output,
    response::CommandOutput,
    show_output, status_command, tree_command, validate_command,
};
use minerva_application::{
    ProjectInstructionService, ProjectMigrationService, ProjectRepository,
    ProjectValidationService, RebuildAction, RebuildResult, RebuildService,
    RepairService, TaskDeclarationService, TaskInstructionService, TaskShowOptions,
    TaskShowService, render_cli,
};
use minerva_storage::{FilesystemProjectRepository, FilesystemTaskRepository};
use std::{env, process::ExitCode};

pub fn run(cli: &Cli) -> ExitCode {
    match execute(cli) {
        Ok(output_text) => output::success(cli, &output_text),
        Err(Failure::Domain(error)) => {
            output::domain(cli, render_cli(&error), error.code())
        }
        Err(Failure::Rebuild(result, dry_run)) => {
            output::rebuild(cli, &render_rebuild(&result, dry_run), &result)
        }
        Err(Failure::Validation(output_text, exit_code, code)) => {
            output::validation(cli, &output_text, exit_code, code)
        }
        Err(Failure::Internal(message)) => output::internal(&message),
    }
}

fn execute(cli: &Cli) -> Result<CommandOutput, Failure> {
    let root = cli.root.clone().map_or_else(
        || env::current_dir().map_err(|err| Failure::Internal(err.to_string())),
        Ok,
    )?;
    dispatch_command(cli, &root)
}

fn dispatch_command(
    cli: &Cli,
    root: &std::path::Path,
) -> Result<CommandOutput, Failure> {
    let project_repo = FilesystemProjectRepository;
    let task_repo = FilesystemTaskRepository;
    match &cli.command {
        Command::Init { force } => init(project_repo, root, *force),
        Command::New(args) => {
            new_command::execute(&project_repo, &task_repo, root, args)
                .map_err(Failure::Domain)
        }
        Command::List(args) => {
            list_command::execute(&project_repo, &task_repo, root, args)
                .map_err(Failure::Domain)
        }
        Command::Tree(args) => {
            tree_command::execute(&project_repo, &task_repo, root, args)
                .map_err(Failure::Domain)
        }
        Command::Show(args) => {
            show(&project_repo, task_repo, root, args).map_err(Failure::Domain)
        }
        Command::Context(args) => {
            context_command::execute(&project_repo, &task_repo, root, args)
                .map_err(Failure::Domain)
        }
        Command::Log(args) => {
            log_command::execute(&project_repo, &task_repo, root, args)
                .map_err(Failure::Domain)
        }
        Command::Delete(args) => {
            delete_command::execute(&project_repo, &task_repo, root, &args.task_ref)
                .map_err(Failure::Domain)
        }
        Command::Status(args) => {
            status_command::set(&project_repo, &task_repo, root, args)
                .map_err(Failure::Domain)
        }
        Command::Complete(args) => {
            status_command::complete(&project_repo, &task_repo, root, &args.task_ref)
                .map_err(Failure::Domain)
        }
        Command::Reopen(args) => {
            status_command::reopen(&project_repo, &task_repo, root, &args.task_ref)
                .map_err(Failure::Domain)
        }
        Command::Move(args) => {
            move_command::execute(&project_repo, &task_repo, root, args)
                .map_err(Failure::Domain)
        }
        Command::Depend(args) => {
            relationship_command::depend(&project_repo, &task_repo, root, args)
                .map_err(Failure::Domain)
        }
        Command::Undepend(args) => {
            relationship_command::undepend(&project_repo, &task_repo, root, args)
                .map_err(Failure::Domain)
        }
        Command::Relate(args) => {
            relationship_command::relate(&project_repo, &task_repo, root, args)
                .map_err(Failure::Domain)
        }
        Command::Unrelate(args) => {
            relationship_command::unrelate(&project_repo, &task_repo, root, args)
                .map_err(Failure::Domain)
        }
        Command::Children(args) => {
            hierarchy_command::children(&project_repo, &task_repo, root, &args.task_ref)
                .map_err(Failure::Domain)
        }
        Command::Ancestors(args) => hierarchy_command::ancestors(
            &project_repo,
            &task_repo,
            root,
            &args.task_ref,
        )
        .map_err(Failure::Domain),
        Command::Repair { dry_run } => {
            repair(&project_repo, &task_repo, root, *dry_run)
        }
        Command::Migrate { dry_run } => {
            migrate(&project_repo, &task_repo, root, *dry_run)
        }
        Command::Instruction { task_ref: None } => {
            open_project_instruction(project_repo, root).map_err(Failure::Domain)
        }
        Command::Instruction { task_ref: Some(task_ref) } => {
            TaskInstructionService::edit(&project_repo, &task_repo, root, task_ref)
                .map(|path| opened(path.as_path()))
                .map_err(Failure::Domain)
        }
        Command::Declaration { task_ref } => {
            TaskDeclarationService::edit(&project_repo, &task_repo, root, task_ref)
                .map(|path| opened(path.as_path()))
                .map_err(Failure::Domain)
        }
        Command::Rebuild { dry_run } => {
            rebuild(project_repo, task_repo, root, *dry_run)
        }
        Command::Validate(args) => validate(&project_repo, &task_repo, root, args),
    }
}

fn init(
    project_repo: FilesystemProjectRepository,
    root: &std::path::Path,
    force: bool,
) -> Result<CommandOutput, Failure> {
    let project =
        project_repo.initialize_project(root, force).map_err(Failure::Domain)?;
    Ok(CommandOutput::text(format!("initialized Minerva in {}", project.name)))
}

fn open_project_instruction(
    project_repo: FilesystemProjectRepository,
    root: &std::path::Path,
) -> Result<CommandOutput, minerva_domain::MinervaError> {
    ProjectInstructionService::edit(&project_repo, root)
        .map(|path| opened(path.as_path()))
}

fn rebuild(
    project_repo: FilesystemProjectRepository,
    task_repo: FilesystemTaskRepository,
    root: &std::path::Path,
    dry_run: bool,
) -> Result<CommandOutput, Failure> {
    let result = RebuildService::run(&project_repo, &task_repo, root, dry_run)
        .map_err(Failure::Domain)?;
    if result.has_errors() {
        Err(Failure::Rebuild(result, dry_run))
    } else {
        Ok(CommandOutput::text(render_rebuild(&result, dry_run)))
    }
}

fn repair(
    project_repo: &impl ProjectRepository,
    task_repo: &impl minerva_application::TaskRepository,
    root: &std::path::Path,
    dry_run: bool,
) -> Result<CommandOutput, Failure> {
    let result = RepairService::run(project_repo, task_repo, root, dry_run)
        .map_err(Failure::Domain)?;
    let output = repair_output::render(&result, dry_run);
    match result.validation.as_ref() {
        Some(validation) if validation.has_errors() => Err(Failure::Validation(
            output,
            crate::exit_code::VALIDATION_ERROR,
            "validation_error",
        )),
        Some(validation) if validation.has_warnings() => Err(Failure::Validation(
            output,
            crate::exit_code::VALIDATION_WARNING,
            "validation_warning",
        )),
        _ => Ok(output),
    }
}

fn migrate(
    project_repo: &impl ProjectRepository,
    task_repo: &impl minerva_application::TaskRepository,
    root: &std::path::Path,
    dry_run: bool,
) -> Result<CommandOutput, Failure> {
    let result = ProjectMigrationService::run(project_repo, root, dry_run)
        .map_err(Failure::Domain)?;
    let rebuild = if dry_run || result.is_current() {
        None
    } else {
        let rebuilt = RebuildService::run(
            &FilesystemProjectRepository,
            &FilesystemTaskRepository,
            root,
            false,
        )
        .map_err(Failure::Domain)?;
        if rebuilt.has_errors() {
            return Err(Failure::Rebuild(rebuilt, false));
        }
        Some(rebuilt)
    };
    let validation = if dry_run || result.is_current() {
        None
    } else {
        Some(
            ProjectValidationService::run(project_repo, task_repo, root)
                .map_err(Failure::Domain)?,
        )
    };
    let output =
        migrate_output::render(&result, rebuild.as_ref(), validation.as_ref(), dry_run);
    match validation {
        Some(result) if result.has_errors() => Err(Failure::Validation(
            output,
            crate::exit_code::VALIDATION_ERROR,
            "validation_error",
        )),
        Some(result) if result.has_warnings() => Err(Failure::Validation(
            output,
            crate::exit_code::VALIDATION_WARNING,
            "validation_warning",
        )),
        _ => Ok(output),
    }
}

fn opened(path: &std::path::Path) -> CommandOutput {
    CommandOutput::text(format!("opened {}", path.display()))
}

fn show(
    project_repo: &impl ProjectRepository,
    task_repo: FilesystemTaskRepository,
    root: &std::path::Path,
    args: &ShowArgs,
) -> Result<CommandOutput, minerva_domain::MinervaError> {
    TaskShowService::show(
        project_repo,
        &task_repo,
        root,
        &args.task_ref,
        &TaskShowOptions {
            include_instructions: args.instructions,
            include_declaration: args.declaration,
        },
    )
    .map(|result| show_output::render(&result))
}

enum Failure {
    Domain(minerva_domain::MinervaError),
    Rebuild(RebuildResult, bool),
    Validation(CommandOutput, u8, &'static str),
    Internal(String),
}

fn validate(
    project_repo: &impl ProjectRepository,
    task_repo: &impl minerva_application::TaskRepository,
    root: &std::path::Path,
    args: &crate::cli::ValidateArgs,
) -> Result<CommandOutput, Failure> {
    let validation = validate_command::execute(project_repo, task_repo, root, args)
        .map_err(Failure::Domain)?;
    if validation.result.has_errors() {
        return Err(Failure::Validation(
            validation.output,
            crate::exit_code::VALIDATION_ERROR,
            "validation_error",
        ));
    }
    if validation.result.has_warnings() {
        return Err(Failure::Validation(
            validation.output,
            crate::exit_code::VALIDATION_WARNING,
            "validation_warning",
        ));
    }
    Ok(validation.output)
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
