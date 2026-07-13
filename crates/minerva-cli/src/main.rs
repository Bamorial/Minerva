use minerva_application::{ProjectInstructionService, ProjectRepository, render_cli};
use minerva_storage::FilesystemProjectRepository;
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
    let repo = FilesystemProjectRepository;
    match parse_command(args)? {
        Command::Init { force } => {
            let project =
                repo.initialize_project(&root, force).map_err(Failure::Domain)?;
            Ok(format!("initialized Minerva in {}", project.name))
        }
        Command::Instruction => {
            let path = ProjectInstructionService::edit(&repo, &root)
                .map_err(Failure::Domain)?;
            Ok(format!("opened {}", path.display()))
        }
    }
}

fn parse_command(args: Vec<String>) -> Result<Command, Failure> {
    match args.as_slice() {
        [command] if command == "init" => Ok(Command::Init { force: false }),
        [command, flag] if command == "init" && flag == "--force" => {
            Ok(Command::Init { force: true })
        }
        [command] if command == "instruction" => Ok(Command::Instruction),
        _ => Err(Failure::Usage(
            "usage: minerva-cli init [--force]\n       minerva-cli instruction".into(),
        )),
    }
}

enum Command {
    Init { force: bool },
    Instruction,
}

enum Failure {
    Usage(String),
    Domain(minerva_domain::MinervaError),
}
