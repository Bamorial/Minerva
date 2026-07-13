use clap::Args;

#[derive(Debug, Clone, Args)]
pub struct StatusArgs {
    pub task_ref: String,
    pub status: String,
}
