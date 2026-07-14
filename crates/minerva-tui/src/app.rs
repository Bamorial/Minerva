use crate::{
    app_command::AppCommand,
    app_dispatch::{Dispatch, dispatch},
    app_services,
    app_state::AppState,
    render,
    terminal_session::TerminalSession,
};
use crossterm::event;
use minerva_application::render_tui;
use minerva_domain::{MinervaError, TaskId};

pub fn run() -> Result<(), MinervaError> {
    let start =
        std::env::current_dir().map_err(|error| terminal_error("cwd", &error))?;
    let mut state = AppState::new(app_services::project_root(&start)?);
    load_initial(&mut state)?;
    let mut session = TerminalSession::enter()?;
    loop {
        session.draw(|frame| render::draw(frame, &state))?;
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

fn load_initial(state: &mut AppState) -> Result<(), MinervaError> {
    let project = app_services::load_project(&state.root)?;
    let statuses =
        project.statuses.into_iter().map(|status| status.key.to_string()).collect();
    state.set_statuses(statuses);
    let result = app_services::load_tree(&state.root)?;
    state.set_tree_with_selected(result.roots, state.selected_task_id());
    reload_detail(state);
    Ok(())
}

fn execute(state: &mut AppState, session: &mut TerminalSession, command: AppCommand) {
    let result = match command {
        AppCommand::Reload => reload(state, state.selected_task_id()),
        AppCommand::CreateTask { title } => {
            let parent_id = state.selected_task_id();
            app_services::create_task(&state.root, title, parent_id).and_then(
                |result| {
                    state.notice = Some(format!(
                        "Created {} {}",
                        result.task.id, result.task.title
                    ));
                    reload(state, Some(result.task.id))
                },
            )
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
            edit(state, session, app_services::edit_instructions)
        }
        AppCommand::EditDeclaration => {
            edit(state, session, app_services::edit_declaration)
        }
        AppCommand::AddDependency { depends_on_ref } => {
            selected_ref(state).and_then(|task_ref| {
                app_services::add_dependency(&state.root, &task_ref, &depends_on_ref)
                    .and_then(|_| {
                        state.notice =
                            Some(format!("Added dependency on {depends_on_ref}"));
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

fn edit(
    state: &mut AppState,
    session: &mut TerminalSession,
    action: fn(&std::path::Path, &str) -> Result<std::path::PathBuf, MinervaError>,
) -> Result<(), MinervaError> {
    let task_ref = selected_ref(state)?;
    session.suspend(|| action(&state.root, &task_ref))?;
    state.notice = Some(format!("Edited {task_ref}"));
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

fn terminal_error(key: &'static str, error: &std::io::Error) -> MinervaError {
    MinervaError::InvalidConfiguration { key: key.into(), reason: error.to_string() }
}
