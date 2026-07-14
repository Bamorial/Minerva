use crate::{app_command::AppCommand, app_prompt::PromptKind, app_state::AppState};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dispatch {
    None,
    Exit,
    Run(AppCommand),
}

pub fn dispatch(state: &mut AppState, event: Event) -> Dispatch {
    match event {
        Event::Key(key) if key.kind == KeyEventKind::Press => handle_key(state, key),
        _ => Dispatch::None,
    }
}

fn handle_key(state: &mut AppState, key: KeyEvent) -> Dispatch {
    if state.confirm.is_some() {
        return confirm_key(state, key.code);
    }
    if state.select.is_some() {
        return select_key(state, key.code);
    }
    if state.prompt.is_some() {
        return prompt_key(state, key.code);
    }
    if state.search_mode {
        return search_key(state, key.code);
    }
    normal_key(state, key)
}

fn confirm_key(state: &mut AppState, code: KeyCode) -> Dispatch {
    match code {
        KeyCode::Char('y') | KeyCode::Enter => state
            .confirm
            .take()
            .map_or(Dispatch::None, |confirm| Dispatch::Run(confirm.command)),
        KeyCode::Char('n') | KeyCode::Esc => {
            state.cancel_action();
            Dispatch::None
        }
        _ => Dispatch::None,
    }
}

fn select_key(state: &mut AppState, code: KeyCode) -> Dispatch {
    let Some(select) = state.select.as_mut() else {
        return Dispatch::None;
    };
    match code {
        KeyCode::Down | KeyCode::Char('j') => {
            select.selected = (select.selected + 1).min(select.options.len() - 1);
            Dispatch::None
        }
        KeyCode::Up | KeyCode::Char('k') => {
            select.selected = select.selected.saturating_sub(1);
            Dispatch::None
        }
        KeyCode::Char(value) if ('1'..='9').contains(&value) => {
            submit_index(state, value)
        }
        KeyCode::Enter => submit_selected(state),
        KeyCode::Esc => {
            state.cancel_action();
            Dispatch::None
        }
        _ => Dispatch::None,
    }
}

fn submit_index(state: &mut AppState, value: char) -> Dispatch {
    let index = value.to_digit(10).unwrap_or_default() as usize - 1;
    if let Some(select) = state.select.as_mut()
        && index < select.options.len()
    {
        select.selected = index;
        return submit_selected(state);
    }
    Dispatch::None
}

fn submit_selected(state: &mut AppState) -> Dispatch {
    state.select.take().map_or(Dispatch::None, |select| {
        Dispatch::Run(AppCommand::ChangeStatus {
            status: select.options[select.selected].clone(),
        })
    })
}

fn prompt_key(state: &mut AppState, code: KeyCode) -> Dispatch {
    match code {
        KeyCode::Esc => {
            state.cancel_action();
            Dispatch::None
        }
        KeyCode::Enter => submit_prompt(state),
        KeyCode::Backspace => {
            state.prompt.as_mut().map(|prompt| prompt.value.pop());
            Dispatch::None
        }
        KeyCode::Char(value) => {
            state.prompt.as_mut().map(|prompt| prompt.value.push(value));
            Dispatch::None
        }
        _ => Dispatch::None,
    }
}

fn submit_prompt(state: &mut AppState) -> Dispatch {
    let Some(prompt) = state.prompt.take() else {
        return Dispatch::None;
    };
    let value = prompt.value.trim().to_string();
    match prompt.kind {
        PromptKind::CreateTask if !value.is_empty() => {
            Dispatch::Run(AppCommand::CreateTask { title: value })
        }
        PromptKind::MoveTask => confirm_move(state, value),
        PromptKind::AddDependency if !value.is_empty() => {
            Dispatch::Run(AppCommand::AddDependency { depends_on_ref: value })
        }
        PromptKind::RemoveDependency if !value.is_empty() => {
            confirm_remove(state, value)
        }
        _ => Dispatch::None,
    }
}

fn confirm_move(state: &mut AppState, value: String) -> Dispatch {
    let target = normalize_ref(&value);
    let summary = target.clone().unwrap_or_else(|| "root".into());
    state.confirm(
        format!("Move task to {summary}? [y/N]"),
        AppCommand::MoveTask { parent_ref: target },
    );
    Dispatch::None
}

fn confirm_remove(state: &mut AppState, depends_on_ref: String) -> Dispatch {
    state.confirm(
        format!("Remove dependency on {depends_on_ref}? [y/N]"),
        AppCommand::RemoveDependency { depends_on_ref },
    );
    Dispatch::None
}

fn normalize_ref(value: &str) -> Option<String> {
    let trimmed = value.trim();
    (!trimmed.is_empty() && trimmed != "root").then(|| trimmed.to_string())
}

fn search_key(state: &mut AppState, code: KeyCode) -> Dispatch {
    match code {
        KeyCode::Esc | KeyCode::Enter => state.finish_search(),
        KeyCode::Backspace => state.pop_search(),
        KeyCode::Char(value) => state.append_search(value),
        _ => {}
    }
    Dispatch::None
}

fn normal_key(state: &mut AppState, key: KeyEvent) -> Dispatch {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => Dispatch::Exit,
        KeyCode::PageDown => {
            state.scroll_detail_down();
            Dispatch::None
        }
        KeyCode::PageUp => {
            state.scroll_detail_up();
            Dispatch::None
        }
        KeyCode::Down | KeyCode::Char('j') => {
            state.select_next();
            Dispatch::None
        }
        KeyCode::Up | KeyCode::Char('k') => {
            state.select_previous();
            Dispatch::None
        }
        KeyCode::Right | KeyCode::Char('l') => {
            state.expand_selected();
            Dispatch::None
        }
        KeyCode::Left | KeyCode::Char('h') => {
            state.collapse_selected();
            Dispatch::None
        }
        KeyCode::Char('d') if key.modifiers == KeyModifiers::CONTROL => {
            state.scroll_detail_down();
            Dispatch::None
        }
        KeyCode::Char('u') if key.modifiers == KeyModifiers::CONTROL => {
            state.scroll_detail_up();
            Dispatch::None
        }
        KeyCode::Char('r') => Dispatch::Run(AppCommand::Reload),
        KeyCode::Char('a') => {
            state.toggle_archived();
            Dispatch::None
        }
        KeyCode::Char('/') => {
            state.begin_search();
            Dispatch::None
        }
        KeyCode::Char('c') => {
            state.clear_search();
            Dispatch::None
        }
        KeyCode::Char('n') => {
            state.begin_prompt(PromptKind::CreateTask);
            Dispatch::None
        }
        KeyCode::Char('s') => open_status_select(state),
        KeyCode::Char('m') => {
            state.begin_prompt(PromptKind::MoveTask);
            Dispatch::None
        }
        KeyCode::Char('i') => Dispatch::Run(AppCommand::EditInstructions),
        KeyCode::Char('e') => Dispatch::Run(AppCommand::EditDeclaration),
        KeyCode::Char('d') => {
            state.begin_prompt(PromptKind::AddDependency);
            Dispatch::None
        }
        KeyCode::Char('x') => {
            state.begin_prompt(PromptKind::RemoveDependency);
            Dispatch::None
        }
        _ => Dispatch::None,
    }
}

fn open_status_select(state: &mut AppState) -> Dispatch {
    let Some(current) =
        state.rows.get(state.selected).map(|row| row.task.status.to_string())
    else {
        return Dispatch::None;
    };
    if !state.statuses.is_empty() {
        state.begin_status_select(&current);
    }
    Dispatch::None
}

#[cfg(test)]
mod tests {
    use super::{Dispatch, dispatch};
    use crate::AppState;
    use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
    use std::path::PathBuf;

    #[test]
    fn start_create_action_from_normal_mode() {
        let mut state = AppState::new(PathBuf::from("."));
        let event = key(KeyCode::Char('n'));
        assert_eq!(dispatch(&mut state, event), Dispatch::None);
        assert!(state.prompt.is_some());
    }

    #[test]
    fn submit_create_prompt_dispatches_command() {
        let mut state = AppState::new(PathBuf::from("."));
        state.begin_prompt(crate::app_prompt::PromptKind::CreateTask);
        state.prompt.as_mut().unwrap().value = "Ship TUI actions".into();
        assert_eq!(
            dispatch(&mut state, key(KeyCode::Enter)),
            Dispatch::Run(crate::app_command::AppCommand::CreateTask {
                title: "Ship TUI actions".into(),
            }),
        );
    }

    #[test]
    fn select_status_dispatches_chosen_value() {
        let mut state = AppState::new(PathBuf::from("."));
        state.statuses = vec!["backlog".into(), "in-progress".into()];
        state.begin_status_select("backlog");
        assert_eq!(
            dispatch(&mut state, key(KeyCode::Char('2'))),
            Dispatch::Run(crate::app_command::AppCommand::ChangeStatus {
                status: "in-progress".into(),
            },)
        );
    }

    #[test]
    fn move_prompt_requires_confirmation() {
        let mut state = AppState::new(PathBuf::from("."));
        state.begin_prompt(crate::app_prompt::PromptKind::MoveTask);
        state.prompt.as_mut().unwrap().value = "TSK-0002".into();
        assert_eq!(dispatch(&mut state, key(KeyCode::Enter)), Dispatch::None);
        assert!(state.confirm.is_some());
    }

    #[test]
    fn confirm_dispatches_pending_command() {
        let mut state = AppState::new(PathBuf::from("."));
        state
            .confirm("Confirm".into(), crate::app_command::AppCommand::EditDeclaration);
        assert_eq!(
            dispatch(&mut state, key(KeyCode::Char('y'))),
            Dispatch::Run(crate::app_command::AppCommand::EditDeclaration),
        );
    }

    fn key(code: KeyCode) -> Event {
        Event::Key(KeyEvent::new(code, KeyModifiers::NONE))
    }
}
