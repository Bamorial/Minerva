use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppEvent {
    Exit,
    Next,
    Previous,
    Expand,
    Collapse,
    Reload,
    ToggleArchived,
    DetailDown,
    DetailUp,
    BeginSearch,
    SearchChar(char),
    SearchBackspace,
    SearchClear,
    SearchFinish,
    Ignore,
}

impl AppEvent {
    #[must_use]
    pub fn from_terminal(event: Event, search_mode: bool) -> Self {
        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                search_event(key, search_mode)
            }
            _ => Self::Ignore,
        }
    }
}

fn search_event(key: KeyEvent, search_mode: bool) -> AppEvent {
    let code = key.code;
    if search_mode {
        return match code {
            KeyCode::Esc | KeyCode::Enter => AppEvent::SearchFinish,
            KeyCode::Backspace => AppEvent::SearchBackspace,
            KeyCode::Char('c') if cfg!(target_os = "macos") => AppEvent::Ignore,
            KeyCode::Char(value) => AppEvent::SearchChar(value),
            _ => AppEvent::Ignore,
        };
    }
    match code {
        KeyCode::Char('q') | KeyCode::Esc => AppEvent::Exit,
        KeyCode::PageDown => AppEvent::DetailDown,
        KeyCode::PageUp => AppEvent::DetailUp,
        KeyCode::Down | KeyCode::Char('j') => AppEvent::Next,
        KeyCode::Up | KeyCode::Char('k') => AppEvent::Previous,
        KeyCode::Right | KeyCode::Char('l') => AppEvent::Expand,
        KeyCode::Left | KeyCode::Char('h') => AppEvent::Collapse,
        KeyCode::Char('d') if key.modifiers == KeyModifiers::CONTROL => {
            AppEvent::DetailDown
        }
        KeyCode::Char('u') if key.modifiers == KeyModifiers::CONTROL => {
            AppEvent::DetailUp
        }
        KeyCode::Char('r') => AppEvent::Reload,
        KeyCode::Char('a') => AppEvent::ToggleArchived,
        KeyCode::Char('/') => AppEvent::BeginSearch,
        KeyCode::Char('c') => AppEvent::SearchClear,
        _ => AppEvent::Ignore,
    }
}
