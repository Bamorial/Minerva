use minerva_domain::MinervaError;
use std::process::{Command, Stdio};

pub fn copy(value: &str) -> Result<(), MinervaError> {
    for program in commands() {
        if write_clipboard(program, value).is_ok() {
            return Ok(());
        }
    }
    Err(MinervaError::InvalidConfiguration {
        key: "clipboard".into(),
        reason: "no supported clipboard command was available".into(),
    })
}

fn commands() -> &'static [&'static [&'static str]] {
    &[
        &["pbcopy"],
        &["wl-copy"],
        &["xclip", "-selection", "clipboard"],
        &["xsel", "--clipboard", "--input"],
    ]
}

fn write_clipboard(command: &[&str], value: &str) -> Result<(), std::io::Error> {
    let mut child = Command::new(command[0])
        .args(&command[1..])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    if let Some(stdin) = child.stdin.as_mut() {
        use std::io::Write;
        stdin.write_all(value.as_bytes())?;
    }
    let status = child.wait()?;
    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other("clipboard command failed"))
    }
}
