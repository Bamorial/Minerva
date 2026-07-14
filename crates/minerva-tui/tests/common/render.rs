#![allow(dead_code)]

use minerva_tui::{AppState, draw};
use ratatui::{Terminal, backend::TestBackend};

pub fn render_screen(state: &AppState) -> String {
    let backend = TestBackend::new(90, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|frame| draw(frame, state)).unwrap();
    let buffer = terminal.backend().buffer();
    (0..buffer.area.height)
        .map(|y| {
            (0..buffer.area.width).map(|x| buffer[(x, y)].symbol()).collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn normalize(value: &str) -> String {
    value.split_whitespace().collect()
}
