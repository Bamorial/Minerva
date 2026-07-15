use crate::{
    app_command::AppCommand,
    app_prompt::PromptKind,
    app_state::{AppState, CreateField, FocusPane, LinkField, PendingSequence},
};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use minerva_domain::RelationshipType;

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
    if let Some(sequence) = state.pending_sequence.take() {
        return pending_key(state, sequence, key);
    }
    if state.settings.is_some() {
        return settings_key(state, key.code);
    }
    if state.delete.is_some() {
        return delete_key(state, key.code);
    }
    if state.create.is_some() {
        return create_key(state, key.code);
    }
    if state.link.is_some() {
        return link_key(state, key.code);
    }
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

fn pending_key(
    state: &mut AppState,
    sequence: PendingSequence,
    key: KeyEvent,
) -> Dispatch {
    match (sequence, key.code) {
        (PendingSequence::CreateOrNextTask, KeyCode::Char('t')) => {
            state.jump_next_peer();
            Dispatch::None
        }
        (PendingSequence::PreviousTask, KeyCode::Char('t')) => {
            state.jump_previous_peer();
            Dispatch::None
        }
        (PendingSequence::CreateOrNextTask, _) => {
            state.begin_create(None);
            handle_key(state, key)
        }
        (PendingSequence::PreviousTask, _) => Dispatch::None,
    }
}

fn delete_key(state: &mut AppState, code: KeyCode) -> Dispatch {
    match code {
        KeyCode::Char('y') | KeyCode::Enter => {
            state.delete.take().map_or(Dispatch::None, |modal| {
                Dispatch::Run(AppCommand::DeleteTask { task_ref: modal.task_ref })
            })
        }
        KeyCode::Char('n') | KeyCode::Esc => {
            state.cancel_action();
            Dispatch::None
        }
        _ => Dispatch::None,
    }
}

fn settings_key(state: &mut AppState, code: KeyCode) -> Dispatch {
    let Some(settings) = state.settings.as_mut() else {
        return Dispatch::None;
    };
    match code {
        KeyCode::Esc => {
            state.cancel_action();
            Dispatch::None
        }
        KeyCode::Tab
        | KeyCode::Left
        | KeyCode::Right
        | KeyCode::Up
        | KeyCode::Down
        | KeyCode::Char('h')
        | KeyCode::Char('j')
        | KeyCode::Char('k')
        | KeyCode::Char('l') => {
            settings.toggle();
            Dispatch::None
        }
        KeyCode::Char('1') => {
            settings.selected_mode = minerva_domain::AgentPromptMode::Static;
            Dispatch::None
        }
        KeyCode::Char('2') => {
            settings.selected_mode = minerva_domain::AgentPromptMode::Exploration;
            Dispatch::None
        }
        KeyCode::Enter => {
            Dispatch::Run(AppCommand::SetPromptMode { mode: settings.selected_mode })
        }
        _ => Dispatch::None,
    }
}

fn create_key(state: &mut AppState, code: KeyCode) -> Dispatch {
    let Some(_) = state.create.as_ref() else {
        return Dispatch::None;
    };
    match code {
        KeyCode::Esc => {
            state.cancel_action();
            Dispatch::None
        }
        KeyCode::Tab | KeyCode::Down | KeyCode::Up => {
            let create = state.create.as_mut().expect("create modal");
            create.field = match create.field {
                CreateField::Title => CreateField::TaskType,
                CreateField::TaskType => CreateField::Title,
            };
            Dispatch::None
        }
        KeyCode::Left => {
            let create = state.create.as_mut().expect("create modal");
            if create.field == CreateField::TaskType && !create.task_types.is_empty() {
                create.selected_type = create
                    .selected_type
                    .checked_sub(1)
                    .unwrap_or(create.task_types.len().saturating_sub(1));
            }
            Dispatch::None
        }
        KeyCode::Right => {
            let create = state.create.as_mut().expect("create modal");
            if create.field == CreateField::TaskType && !create.task_types.is_empty() {
                create.selected_type =
                    (create.selected_type + 1) % create.task_types.len();
            }
            Dispatch::None
        }
        KeyCode::Backspace => {
            let create = state.create.as_mut().expect("create modal");
            if create.field == CreateField::Title {
                create.title.pop();
            }
            Dispatch::None
        }
        KeyCode::Char(value) => {
            let create = state.create.as_mut().expect("create modal");
            if create.field == CreateField::Title {
                create.title.push(value);
            }
            Dispatch::None
        }
        KeyCode::Enter => submit_create(state),
        _ => Dispatch::None,
    }
}

fn submit_create(state: &mut AppState) -> Dispatch {
    let Some(create) = state.create.as_ref() else {
        return Dispatch::None;
    };
    if create.field == CreateField::Title {
        if let Some(create) = state.create.as_mut() {
            create.field = CreateField::TaskType;
        }
        return Dispatch::None;
    }
    let title = create.title.trim().to_string();
    if title.is_empty() || create.task_types.is_empty() {
        return Dispatch::None;
    }
    let task_type = create.task_types[create.selected_type].clone();
    let parent_id = create.parent_id;
    state.create = None;
    Dispatch::Run(AppCommand::CreateTask { title, task_type, parent_id })
}

fn link_key(state: &mut AppState, code: KeyCode) -> Dispatch {
    let Some(_) = state.link.as_ref() else {
        return Dispatch::None;
    };
    match code {
        KeyCode::Esc => {
            state.cancel_action();
            Dispatch::None
        }
        KeyCode::Tab => {
            let link = state.link.as_mut().expect("link modal");
            link.field = match link.field {
                LinkField::Query => LinkField::Relationship,
                LinkField::Relationship => LinkField::Results,
                LinkField::Results => LinkField::Query,
            };
            Dispatch::None
        }
        KeyCode::Down => {
            let link = state.link.as_mut().expect("link modal");
            if link.field == LinkField::Results {
                link.selected =
                    (link.selected + 1).min(link.candidates.len().saturating_sub(1));
            } else {
                link.field = LinkField::Results;
            }
            Dispatch::None
        }
        KeyCode::Up => {
            let link = state.link.as_mut().expect("link modal");
            if link.field == LinkField::Results {
                link.selected = link.selected.saturating_sub(1);
            } else {
                link.field = LinkField::Query;
            }
            Dispatch::None
        }
        KeyCode::Left => {
            let link = state.link.as_mut().expect("link modal");
            if link.field == LinkField::Relationship {
                link.relationship_type = RelationshipType::DependsOn;
            }
            Dispatch::None
        }
        KeyCode::Right => {
            let link = state.link.as_mut().expect("link modal");
            if link.field == LinkField::Relationship {
                link.relationship_type = RelationshipType::References;
            }
            Dispatch::None
        }
        KeyCode::Backspace => {
            let query_field =
                state.link.as_ref().is_some_and(|link| link.field == LinkField::Query);
            if query_field {
                state.link.as_mut().expect("link modal").query.pop();
                state.refresh_link_candidates();
            }
            Dispatch::None
        }
        KeyCode::Char(value) => {
            let query_field =
                state.link.as_ref().is_some_and(|link| link.field == LinkField::Query);
            if query_field {
                state.link.as_mut().expect("link modal").query.push(value);
                state.refresh_link_candidates();
            }
            Dispatch::None
        }
        KeyCode::Enter => submit_link(state),
        _ => Dispatch::None,
    }
}

fn submit_link(state: &mut AppState) -> Dispatch {
    let Some(link) = state.link.as_ref() else {
        return Dispatch::None;
    };
    let Some(candidate) = link.candidates.get(link.selected) else {
        return Dispatch::None;
    };
    let relationship_type = link.relationship_type;
    let task_ref = candidate.task_ref.clone();
    state.link = None;
    Dispatch::Run(AppCommand::AddRelationship { task_ref, relationship_type })
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
    let Some(_) = state.select.as_ref() else {
        return Dispatch::None;
    };
    match code {
        KeyCode::Down | KeyCode::Char('j') => {
            let select = state.select.as_mut().expect("select state");
            select.selected = (select.selected + 1).min(select.options.len() - 1);
            Dispatch::None
        }
        KeyCode::Up | KeyCode::Char('k') => {
            let select = state.select.as_mut().expect("select state");
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
        PromptKind::MoveTask => confirm_move(state, value),
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
        KeyCode::Enter
            if state.focus == FocusPane::Tree && state.selected_children_hidden() =>
        {
            state.expand_selected();
            Dispatch::None
        }
        KeyCode::Enter | KeyCode::Char('0') => {
            state.focus_current();
            Dispatch::None
        }
        KeyCode::Char('1') if state.count_buffer.is_empty() => {
            state.focus_tree();
            Dispatch::None
        }
        KeyCode::PageDown => {
            state.scroll_detail_down();
            Dispatch::None
        }
        KeyCode::PageUp => {
            state.scroll_detail_up();
            Dispatch::None
        }
        KeyCode::Char(value)
            if state.focus == FocusPane::Tree && digit(state, value) =>
        {
            Dispatch::None
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let count = state.take_count();
            state.select_next(count);
            Dispatch::None
        }
        KeyCode::Up | KeyCode::Char('k') => {
            let count = state.take_count();
            state.select_previous(count);
            Dispatch::None
        }
        KeyCode::Right | KeyCode::Char('l') if state.focus == FocusPane::Tree => {
            state.expand_selected();
            Dispatch::None
        }
        KeyCode::Left | KeyCode::Char('h') if state.focus == FocusPane::Tree => {
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
        KeyCode::Char('/') => {
            state.begin_search();
            Dispatch::None
        }
        KeyCode::Char('n') => {
            state.pending_sequence = Some(PendingSequence::CreateOrNextTask);
            Dispatch::None
        }
        KeyCode::Char('N') => {
            state.pending_sequence = Some(PendingSequence::PreviousTask);
            Dispatch::None
        }
        KeyCode::Char('a') => {
            state.begin_create(state.selected_task_id());
            Dispatch::None
        }
        KeyCode::Char('s') => {
            state.begin_settings();
            Dispatch::None
        }
        KeyCode::Char('S') => open_status_select(state),
        KeyCode::Char('m') => {
            state.begin_prompt(PromptKind::MoveTask);
            Dispatch::None
        }
        KeyCode::Char('e') => Dispatch::Run(AppCommand::EditInstructions),
        KeyCode::Char('I') => Dispatch::Run(AppCommand::EditProjectInstructions),
        KeyCode::Char('c') => {
            Dispatch::Run(AppCommand::ShowContext { mode: state.prompt_mode })
        }
        KeyCode::Char('y')
            if state.current_view == crate::app_state::CurrentView::Context =>
        {
            Dispatch::Run(AppCommand::CopyContext)
        }
        KeyCode::Char('@') => {
            state.begin_link();
            Dispatch::None
        }
        KeyCode::Char('d') => {
            state.begin_delete();
            Dispatch::None
        }
        KeyCode::Char('x') => {
            state.begin_prompt(PromptKind::RemoveDependency);
            Dispatch::None
        }
        KeyCode::Char('t') if state.focus == FocusPane::Tree => {
            state.jump_next_peer();
            Dispatch::None
        }
        KeyCode::Char('T') if state.focus == FocusPane::Tree => {
            state.jump_previous_peer();
            Dispatch::None
        }
        _ => {
            state.clear_count();
            Dispatch::None
        }
    }
}

fn digit(state: &mut AppState, value: char) -> bool {
    if !value.is_ascii_digit() {
        return false;
    }
    if state.count_buffer.is_empty() && matches!(value, '0' | '1') {
        return false;
    }
    state.push_count(value);
    true
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
