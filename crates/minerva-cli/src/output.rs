use crate::{cli::Cli, exit_code, response::CommandOutput};
use minerva_application::{CliErrorReport, RebuildResult};
use serde_json::json;
use std::process::ExitCode;

pub fn success(cli: &Cli, output: &CommandOutput) -> ExitCode {
    if cli.quiet {
        return ExitCode::SUCCESS;
    }
    if cli.json {
        print_json(&success_json(cli, output));
        return ExitCode::SUCCESS;
    }
    if cli.verbose > 0 {
        println!(
            "command: {}\nroot: {}",
            cli.command.name(),
            cli.root
                .as_ref()
                .map_or_else(|| ".".into(), |path| path.display().to_string())
        );
    }
    println!("{}", output.text);
    ExitCode::SUCCESS
}

pub fn domain(
    cli: &Cli,
    report: CliErrorReport,
    code: minerva_domain::ErrorCode,
) -> ExitCode {
    let code = exit_code::for_domain(code);
    if cli.json {
        print_ejson(&json!({
            "ok": false, "code": report.code, "exit_code": code,
            "message": report.message, "details": report.details,
        }));
        return exit_code::code(code);
    }
    eprintln!("{} [{}]", report.message, report.code);
    for detail in report.details {
        eprintln!("{detail}");
    }
    if cli.verbose > 0 {
        eprintln!("exit code: {code}");
    }
    exit_code::code(code)
}

pub fn rebuild(cli: &Cli, output: &str, result: &RebuildResult) -> ExitCode {
    let task_errors: Vec<_> = result
        .task_errors
        .iter()
        .map(|error| {
            json!({
                "task_ref": error.task_ref,
                "path": error.path,
                "reason": error.reason,
            })
        })
        .collect();
    if cli.json {
        print_ejson(&json!({
            "ok": false, "code": "rebuild_validation_failure",
            "exit_code": exit_code::REBUILD_FAILURE, "output": output,
            "task_errors": task_errors,
        }));
    } else {
        eprintln!("{output}");
    }
    exit_code::code(exit_code::REBUILD_FAILURE)
}

pub fn validation(
    cli: &Cli,
    output: &CommandOutput,
    exit_code_value: u8,
    code: &str,
) -> ExitCode {
    if cli.json {
        let result =
            output.json.clone().unwrap_or_else(|| json!({ "output": output.text }));
        print_ejson(&json!({
            "ok": false, "code": code, "exit_code": exit_code_value,
            "result": result,
        }));
    } else {
        eprintln!("{}", output.text);
    }
    exit_code::code(exit_code_value)
}

pub fn internal(message: &str) -> ExitCode {
    eprintln!("{message}");
    exit_code::code(exit_code::INTERNAL_FAILURE)
}

#[rustfmt::skip]
fn print_json(value: &serde_json::Value) { println!("{value}"); }
#[rustfmt::skip]
fn print_ejson(value: &serde_json::Value) { eprintln!("{value}"); }
fn success_json(cli: &Cli, output: &CommandOutput) -> serde_json::Value {
    output.json.clone().map_or_else(
        || {
            json!({
                "ok": true, "command": cli.command.name(), "root": cli.root,
                "output": output.text,
            })
        },
        |value| {
            json!({
                "ok": true, "command": cli.command.name(), "root": cli.root,
                "result": value,
            })
        },
    )
}
