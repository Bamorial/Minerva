use crate::{
    app_event::AppEvent, app_services, app_state::AppState, render,
    terminal_session::TerminalSession,
};
use crossterm::event;
use minerva_application::render_tui;
use minerva_domain::MinervaError;

pub fn run() -> Result<(), MinervaError> {
    let start =
        std::env::current_dir().map_err(|error| terminal_error("cwd", &error))?;
    let mut state = AppState::new(app_services::project_root(&start)?);
    load_initial(&mut state)?;
    let mut session = TerminalSession::enter()?;
    loop {
        session.draw(|frame| render::draw(frame, &state))?;
        match AppEvent::from_terminal(read_event()?, state.search_mode) {
            AppEvent::Exit => return Ok(()),
            AppEvent::Next => update_detail(&mut state, AppState::select_next),
            AppEvent::Previous => update_detail(&mut state, AppState::select_previous),
            AppEvent::Expand => update_detail(&mut state, AppState::expand_selected),
            AppEvent::Collapse => {
                update_detail(&mut state, AppState::collapse_selected)
            }
            AppEvent::Reload => reload(&mut state),
            AppEvent::ToggleArchived => {
                update_detail(&mut state, AppState::toggle_archived)
            }
            AppEvent::DetailDown => state.scroll_detail_down(),
            AppEvent::DetailUp => state.scroll_detail_up(),
            AppEvent::BeginSearch => state.begin_search(),
            AppEvent::SearchChar(value) => {
                search(&mut state, |state| state.append_search(value))
            }
            AppEvent::SearchBackspace => search(&mut state, AppState::pop_search),
            AppEvent::SearchClear => search(&mut state, AppState::clear_search),
            AppEvent::SearchFinish => state.finish_search(),
            AppEvent::Ignore => {}
        }
    }
}

fn load_initial(state: &mut AppState) -> Result<(), MinervaError> {
    let result = app_services::load_tree(&state.root)?;
    state.set_tree(result.roots);
    reload_detail(state);
    Ok(())
}

fn reload(state: &mut AppState) {
    match app_services::load_tree(&state.root) {
        Ok(result) => {
            state.error = None;
            state.set_tree(result.roots);
            state.reset_detail_scroll();
            reload_detail(state);
        }
        Err(error) => state.error = Some(render_tui(&error)),
    }
}

fn read_event() -> Result<crossterm::event::Event, MinervaError> {
    event::read().map_err(|error| terminal_error("event", &error))
}

fn search(state: &mut AppState, update: impl FnOnce(&mut AppState)) {
    update(state);
    reload_detail(state);
}

fn update_detail(state: &mut AppState, update: impl FnOnce(&mut AppState)) {
    update(state);
    reload_detail(state);
}

fn reload_detail(state: &mut AppState) {
    state.detail = state
        .selected_task_ref()
        .and_then(|task_ref| app_services::load_task(&state.root, &task_ref).ok());
    if state.detail.is_some() || state.rows.is_empty() {
        state.error = None;
    }
}

fn terminal_error(key: &'static str, error: &std::io::Error) -> MinervaError {
    MinervaError::InvalidConfiguration { key: key.into(), reason: error.to_string() }
}
