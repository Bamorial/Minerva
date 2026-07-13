use minerva_application::{
    ProjectInstructionService, ProjectRepository, TaskDeclarationService,
    TaskInstructionService, TaskStatusService, render_cli,
};
use minerva_storage::{FilesystemProjectRepository, FilesystemTaskRepository};
use std::{env, process::ExitCode};

fn main() -> ExitCode {
    match run(env::args().skip(1).collect()) {
        Ok(message) => {
            println!("{message}");
            ExitCode::SUCCESS
        }
        Err(Failure::Usage(message)) => {
            eprintln!("{message}");
            ExitCode::from(2)
        }
        Err(Failure::Domain(error)) => {
            let report = render_cli(&error);
            eprintln!("{} [{}]", report.message, report.code);
            for detail in report.details {
                eprintln!("{detail}");
            }
            ExitCode::from(1)
        }
    }
}

fn run(args: Vec<String>) -> Result<String, Failure> {
    let root = env::current_dir().map_err(|err| Failure::Usage(err.to_string()))?;
    let project_repo = FilesystemProjectRepository;
    let task_repo = FilesystemTaskRepository;
    match parse_command(args)? {
        Command::Init { force } => {
            let project = project_repo
                .initialize_project(&root, force)
                .map_err(Failure::Domain)?;
            Ok(format!("initialized Minerva in {}", project.name))
        }
        Command::Instruction { task_ref: None } => {
            let path = ProjectInstructionService::edit(&project_repo, &root)
                .map_err(Failure::Domain)?;
            Ok(format!("opened {}", path.display()))
        }
        Command::Instruction { task_ref: Some(task_ref) } => {
            let path = TaskInstructionService::edit(
                &project_repo,
                &task_repo,
                &root,
                &task_ref,
            )
            .map_err(Failure::Domain)?;
            Ok(format!("opened {}", path.display()))
        }
        Command::Declaration { task_ref } => {
            let path = TaskDeclarationService::edit(
                &project_repo,
                &task_repo,
                &root,
                &task_ref,
            )
            .map_err(Failure::Domain)?;
            Ok(format!("opened {}", path.display()))
        }
        Command::Status { task_ref } => {
            TaskStatusService::show(&project_repo, &task_repo, &root, &task_ref)
                .map_err(Failure::Domain)
        }
    }
}

fn parse_command(args: Vec<String>) -> Result<Command, Failure> {
    match args.as_slice() {
        [command] if command == "init" => Ok(Command::Init { force: false }),
        [command, flag] if command == "init" && flag == "--force" => {
            Ok(Command::Init { force: true })
        }
        [command] if command == "instruction" => {
            Ok(Command::Instruction { task_ref: None })
        }
        [command, task_ref] if command == "instruction" => {
            Ok(Command::Instruction { task_ref: Some(task_ref.clone()) })
        }
        [command, task_ref] if command == "declaration" => {
            Ok(Command::Declaration { task_ref: task_ref.clone() })
        }
        [command, task_ref] if command == "status" => {
            Ok(Command::Status { task_ref: task_ref.clone() })
        }
        _ => Err(Failure::Usage(
            "usage: minerva-cli init [--force]\n       minerva-cli instruction [<task>]\n       minerva-cli declaration <task>\n       minerva-cli status <task>".into(),
        )),
    }
}

enum Command {
    Init { force: bool },
    Instruction { task_ref: Option<String> },
    Declaration { task_ref: String },
    Status { task_ref: String },
}

enum Failure {
    Usage(String),
    Domain(minerva_domain::MinervaError),
}
