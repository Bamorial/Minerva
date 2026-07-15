use crate::{
    app_command::AppCommand,
    app_dispatch::{Dispatch, dispatch},
    app_services,
    app_state::AppState,
    clipboard, render,
    terminal_session::TerminalSession,
};
use crossterm::event;
use minerva_application::render_tui;
use minerva_domain::{MinervaError, TaskId};
use std::time::Duration;

pub fn run() -> Result<(), MinervaError> {
    let start =
        std::env::current_dir().map_err(|error| terminal_error("cwd", &error))?;
    let mut state = AppState::new(app_services::project_root(&start)?);
    load_initial(&mut state)?;
    let mut session = TerminalSession::enter()?;
    loop {
        session.draw(|frame| render::draw(frame, &state))?;
        if handle_pending_sequence(&mut state)? {
            continue;
        }
        let selected = state.selected_task_id();
        match dispatch(&mut state, read_event()?) {
            Dispatch::Exit => return Ok(()),
            Dispatch::Run(command) => execute(&mut state, &mut session, command),
            Dispatch::None if selection_changed(selected, &state) => {
                reload_detail(&mut state)
            }
            Dispatch::None => {}
        }
    }
}

fn handle_pending_sequence(state: &mut AppState) -> Result<bool, MinervaError> {
    if state.pending_sequence.is_none() {
        return Ok(false);
    }
    let ready = event::poll(Duration::from_millis(250))
        .map_err(|error| terminal_error("event", &error))?;
    if ready {
        return Ok(false);
    }
    if matches!(
        state.pending_sequence.take(),
        Some(crate::app_state::PendingSequence::CreateOrNextTask)
    ) {
        state.begin_create(None);
    }
    Ok(true)
}

fn load_initial(state: &mut AppState) -> Result<(), MinervaError> {
    let project = app_services::load_project(&state.root)?;
    let statuses =
        project.statuses.into_iter().map(|status| status.key.to_string()).collect();
    state.set_statuses(statuses);
    state.set_task_types(app_services::load_task_types(&state.root)?);
    let result = app_services::load_tree(&state.root)?;
    state.set_tree_with_selected(result.roots, state.selected_task_id());
    reload_detail(state);
    Ok(())
}

fn execute(state: &mut AppState, session: &mut TerminalSession, command: AppCommand) {
    let result = match command {
        AppCommand::Reload => reload(state, state.selected_task_id()),
        AppCommand::CreateTask { title, task_type, parent_id } => {
            app_services::create_task(&state.root, title, task_type, parent_id)
                .and_then(|result| {
                    state.notice = Some(format!(
                        "Created {} {}",
                        result.task.id, result.task.title
                    ));
                    reload(state, Some(result.task.id))
                })
        }
        AppCommand::ChangeStatus { status } => {
            selected_ref(state).and_then(|task_ref| {
                app_services::set_status(&state.root, &task_ref, &status).and_then(
                    |result| {
                        state.notice = Some(format!(
                            "{} -> {}",
                            result.task.id, result.task.status
                        ));
                        reload(state, Some(result.task.id))
                    },
                )
            })
        }
        AppCommand::MoveTask { parent_ref } => {
            selected_ref(state).and_then(|task_ref| {
                app_services::move_task(&state.root, &task_ref, parent_ref.as_deref())
                    .and_then(|result| {
                        state.notice = Some(format!("Moved {}", result.task.id));
                        reload(state, Some(result.task.id))
                    })
            })
        }
        AppCommand::EditInstructions => {
            edit_selected(state, session, app_services::edit_task_instructions)
        }
        AppCommand::EditProjectInstructions => edit_project(state, session),
        AppCommand::ShowContext => selected_ref(state).and_then(|task_ref| {
            app_services::load_context(&state.root, &task_ref).map(|context| {
                state.notice = Some(format!("Loaded context for {task_ref}"));
                state.show_context(context);
            })
        }),
        AppCommand::CopyContext => state.context.as_deref().map_or_else(
            || {
                Err(MinervaError::InvalidConfiguration {
                    key: "context".into(),
                    reason: "no compiled context is loaded".into(),
                })
            },
            |context| {
                clipboard::copy(context).map(|_| {
                    state.notice = Some("Copied compiled context to clipboard".into());
                })
            },
        ),
        AppCommand::AddRelationship { task_ref, relationship_type } => {
            selected_ref(state).and_then(|source_ref| {
                app_services::add_relationship(
                    &state.root,
                    &source_ref,
                    &task_ref,
                    relationship_type,
                )
                .and_then(|_| {
                    state.notice = Some(format!(
                        "Added {} relationship to {task_ref}",
                        relationship_name(relationship_type)
                    ));
                    reload(state, state.selected_task_id())
                })
            })
        }
        AppCommand::RemoveDependency { depends_on_ref } => selected_ref(state)
            .and_then(|task_ref| {
                app_services::remove_dependency(&state.root, &task_ref, &depends_on_ref)
                    .and_then(|_| {
                        state.notice =
                            Some(format!("Removed dependency on {depends_on_ref}"));
                        reload(state, state.selected_task_id())
                    })
            }),
        AppCommand::DeleteTask { task_ref } => {
            app_services::delete_task(&state.root, &task_ref).and_then(|result| {
                state.notice = Some(format!(
                    "Deleted {} task(s) rooted at {}",
                    result.deleted_task_ids.len(),
                    result.task.id
                ));
                reload(state, result.task.parent_id)
            })
        }
    };
    if let Err(error) = result {
        state.error = Some(render_tui(&error));
    }
}

fn reload(state: &mut AppState, selected: Option<TaskId>) -> Result<(), MinervaError> {
    match app_services::load_tree(&state.root) {
        Ok(result) => {
            state.set_tree_with_selected(result.roots, selected);
            state.reset_detail_scroll();
            reload_detail(state);
            Ok(())
        }
        Err(error) => Err(error),
    }
}

fn read_event() -> Result<crossterm::event::Event, MinervaError> {
    event::read().map_err(|error| terminal_error("event", &error))
}

fn reload_detail(state: &mut AppState) {
    let Some(task_ref) = state.selected_task_ref() else {
        state.detail = None;
        return;
    };
    match app_services::load_task(&state.root, &task_ref) {
        Ok(detail) => state.detail = Some(detail),
        Err(error) => {
            state.detail = None;
            state.error = Some(render_tui(&error));
        }
    }
}

fn edit_selected(
    state: &mut AppState,
    session: &mut TerminalSession,
    action: fn(&std::path::Path, &str) -> Result<std::path::PathBuf, MinervaError>,
) -> Result<(), MinervaError> {
    let task_ref = selected_ref(state)?;
    session.suspend(|| action(&state.root, &task_ref))?;
    state.notice = Some(format!("Edited instructions for {task_ref}"));
    reload(state, state.selected_task_id())
}

fn edit_project(
    state: &mut AppState,
    session: &mut TerminalSession,
) -> Result<(), MinervaError> {
    session.suspend(|| app_services::edit_project_instructions(&state.root))?;
    state.notice = Some("Edited project instructions".into());
    reload(state, state.selected_task_id())
}

fn selected_ref(state: &AppState) -> Result<String, MinervaError> {
    state.selected_task_ref().ok_or_else(no_task_selected)
}

fn no_task_selected() -> MinervaError {
    MinervaError::TaskNotFound { task_ref: "selection".into() }
}

fn selection_changed(previous: Option<TaskId>, state: &AppState) -> bool {
    previous != state.selected_task_id()
}

fn relationship_name(value: minerva_domain::RelationshipType) -> &'static str {
    match value {
        minerva_domain::RelationshipType::DependsOn => "depends-on",
        minerva_domain::RelationshipType::References => "references",
        _ => "relationship",
    }
}

fn terminal_error(key: &'static str, error: &std::io::Error) -> MinervaError {
    MinervaError::InvalidConfiguration { key: key.into(), reason: error.to_string() }
}
