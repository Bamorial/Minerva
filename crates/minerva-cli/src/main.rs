mod cli;
mod exit_code;
mod output;
mod run;

use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    match cli::Cli::try_parse() {
        Ok(cli) => run::run(cli),
        Err(error) => {
            let code = error.exit_code();
            error.print().expect("failed to write clap output");
            ExitCode::from(code as u8)
        }
    }
}
