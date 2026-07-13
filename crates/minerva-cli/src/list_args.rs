use clap::{Args, ValueEnum};

#[derive(Debug, Clone, Args)]
pub struct ListArgs {
    #[arg(long, value_name = "STATUS")]
    pub status: Option<String>,
    #[arg(long = "type", value_name = "TASK_TYPE")]
    pub task_type: Option<String>,
    #[arg(long, value_name = "TASK_REF")]
    pub parent: Option<String>,
    #[arg(long, value_name = "TAG")]
    pub tag: Option<String>,
    #[arg(long, value_enum, default_value_t = ArchiveStateArg::Active)]
    pub archive_state: ArchiveStateArg,
    #[arg(long, value_name = "TEXT")]
    pub search: Option<String>,
    #[arg(long, value_enum, default_value_t = SortArg::Id)]
    pub sort: SortArg,
    #[arg(long, default_value_t = 0)]
    pub offset: usize,
    #[arg(long, default_value_t = 50, conflicts_with = "all")]
    pub limit: usize,
    #[arg(long)]
    pub all: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ArchiveStateArg {
    Active,
    Archived,
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum SortArg {
    Created,
    Updated,
    Priority,
    Title,
    Id,
}
