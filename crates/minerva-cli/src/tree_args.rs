use crate::list_args::ArchiveStateArg;
use clap::Args;

#[derive(Debug, Clone, Args)]
pub struct TreeArgs {
    #[arg(long, value_name = "STATUS")]
    pub status: Option<String>,
    #[arg(long, value_enum, default_value_t = ArchiveStateArg::Active)]
    pub archive_state: ArchiveStateArg,
}
