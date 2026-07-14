use minerva_domain::MinervaError;
use ratatui::{DefaultTerminal, Frame};

pub struct TerminalSession {
    terminal: Option<DefaultTerminal>,
}

impl TerminalSession {
    pub fn enter() -> Result<Self, MinervaError> {
        install_panic_hook();
        Ok(Self { terminal: Some(ratatui::init()) })
    }

    pub fn draw<F>(&mut self, render: F) -> Result<(), MinervaError>
    where
        F: FnOnce(&mut Frame<'_>),
    {
        self.terminal
            .as_mut()
            .expect("terminal session should be active")
            .draw(render)
            .map(|_| ())
            .map_err(|error| terminal_error("draw", &error))
    }

    pub fn suspend<T>(
        &mut self,
        action: impl FnOnce() -> Result<T, MinervaError>,
    ) -> Result<T, MinervaError> {
        self.terminal.take();
        let _ = ratatui::restore();
        let result = action();
        self.terminal = Some(ratatui::init());
        result
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        if self.terminal.take().is_some() {
            let _ = ratatui::restore();
        }
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
