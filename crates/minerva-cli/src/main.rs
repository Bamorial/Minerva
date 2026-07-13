use minerva_application::{ProjectRepository, render_cli};
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
    let force = parse_init(args)?;
    let root = env::current_dir().map_err(|err| Failure::Usage(err.to_string()))?;
    let project = FilesystemProjectRepository
        .initialize_project(&root, force)
        .map_err(Failure::Domain)?;
    Ok(format!("initialized Minerva in {}", project.name))
}

fn parse_init(args: Vec<String>) -> Result<bool, Failure> {
    match args.as_slice() {
        [command] if command == "init" => Ok(false),
        [command, flag] if command == "init" && flag == "--force" => Ok(true),
        _ => Err(Failure::Usage("usage: minerva-cli init [--force]".into())),
    }
}

enum Failure {
    Usage(String),
    Domain(minerva_domain::MinervaError),
}
