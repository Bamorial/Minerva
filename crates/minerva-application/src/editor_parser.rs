use minerva_domain::MinervaError;

pub fn parse_editor(source: &str) -> Result<(String, Vec<String>), MinervaError> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut quote = None;
    let mut escape = false;
    for ch in source.chars() {
        if escape {
            current.push(ch);
            escape = false;
        } else if ch == '\\' {
            escape = true;
        } else if let Some(active) = quote {
            if ch == active {
                quote = None;
            } else {
                current.push(ch);
            }
        } else if ch == '"' || ch == '\'' {
            quote = Some(ch);
        } else if ch.is_whitespace() {
            push_part(&mut parts, &mut current);
        } else {
            current.push(ch);
        }
    }
    if escape || quote.is_some() {
        return invalid(source, "contains an unterminated escape or quote");
    }
    push_part(&mut parts, &mut current);
    let Some((program, args)) = parts.split_first() else {
        return invalid(source, "must not be empty");
    };
    Ok((program.clone(), args.to_vec()))
}

fn push_part(parts: &mut Vec<String>, current: &mut String) {
    if !current.is_empty() {
        parts.push(std::mem::take(current));
    }
}

fn invalid<T>(editor: &str, reason: &str) -> Result<T, MinervaError> {
    Err(MinervaError::EditorLaunchFailure { editor: editor.into(), reason: reason.into() })
}
