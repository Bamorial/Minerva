use minerva_domain::MinervaError;
use ratatui::{DefaultTerminal, Frame};

pub struct TerminalSession {
    terminal: DefaultTerminal,
}

impl TerminalSession {
    pub fn enter() -> Result<Self, MinervaError> {
        install_panic_hook();
        Ok(Self { terminal: ratatui::init() })
    }

    pub fn draw<F>(&mut self, render: F) -> Result<(), MinervaError>
    where
        F: FnOnce(&mut Frame<'_>),
    {
        self.terminal
            .draw(render)
            .map(|_| ())
            .map_err(|error| terminal_error("draw", &error))
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        let _ = ratatui::restore();
    }
}

fn install_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic| {
        let _ = ratatui::restore();
        hook(panic);
    }));
}

fn terminal_error(key: &'static str, error: &std::io::Error) -> MinervaError {
    MinervaError::InvalidConfiguration { key: key.into(), reason: error.to_string() }
}
