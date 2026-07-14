#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectState {
    pub title: &'static str,
    pub options: Vec<String>,
    pub selected: usize,
}

impl SelectState {
    #[must_use]
    pub fn new(title: &'static str, options: Vec<String>, selected: usize) -> Self {
        Self { title, options, selected }
    }
}
