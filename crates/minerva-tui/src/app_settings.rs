use minerva_domain::AgentPromptMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SettingsModal {
    pub selected_mode: AgentPromptMode,
}

impl SettingsModal {
    #[must_use]
    pub const fn new(selected_mode: AgentPromptMode) -> Self {
        Self { selected_mode }
    }

    pub fn toggle(&mut self) {
        self.selected_mode = match self.selected_mode {
            AgentPromptMode::Static => AgentPromptMode::Exploration,
            AgentPromptMode::Exploration => AgentPromptMode::Static,
        };
    }
}
