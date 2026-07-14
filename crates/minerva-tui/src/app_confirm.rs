use crate::app_command::AppCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfirmState {
    pub message: String,
    pub command: AppCommand,
}
