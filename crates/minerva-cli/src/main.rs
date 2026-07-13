mod cli;
mod context_command;
mod context_output;
mod exit_code;
mod hierarchy_command;
mod hierarchy_output;
mod list_args;
mod list_command;
mod list_output;
mod log_command;
mod log_output;
mod move_command;
mod new_command;
mod new_prompt;
mod new_resolve;
mod output;
mod relationship_command;
mod relationship_output;
mod response;
mod run;
mod show_output;
mod status_args;
mod status_command;
mod task_ref_arg;
mod tree_args;
mod tree_command;
mod tree_output;

use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    match cli::Cli::try_parse() {
        Ok(cli) => run::run(&cli),
        Err(error) => {
            let code = error.exit_code();
            error.print().expect("failed to write clap output");
            ExitCode::from(u8::try_from(code).unwrap_or(1))
        }
    }
}
