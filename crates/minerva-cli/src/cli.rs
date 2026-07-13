use crate::list_args::ListArgs;
use crate::tree_args::TreeArgs;
use clap::{ArgAction, Args, Parser, Subcommand};
use std::path::PathBuf;

const HELP: &str = "Exit codes:\n  0 success\n  1 internal failure\n  2 command usage error\n  10 project not initialized\n  11 project already initialized\n  12 task not found\n  13 ambiguous task reference\n  14 invalid status transition\n  15 hierarchy cycle\n  16 dependency cycle\n  17 schema error\n  18 version conflict\n  19 lock conflict\n  20 invalid configuration\n  21 editor launch failure\n  22 rebuild validation failure";

#[derive(Debug, Parser)]
#[command(name = "minerva", version, about = "Minerva command line interface")]
#[command(after_help = HELP)]
pub struct Cli {
    #[arg(long, global = true, value_name = "PATH")]
    pub root: Option<PathBuf>,
    #[arg(long, global = true)]
    pub json: bool,
    #[arg(long, global = true, conflicts_with_all = ["json", "verbose"])]
    pub quiet: bool,
    #[arg(short, long, global = true, action = ArgAction::Count)]
    pub verbose: u8,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Init {
        #[arg(long)]
        force: bool,
    },
    New(NewArgs),
    List(ListArgs),
    Tree(TreeArgs),
    Show(ShowArgs),
    Instruction {
        task_ref: Option<String>,
    },
    Declaration {
        task_ref: String,
    },
    Status(ShowArgs),
    Rebuild {
        #[arg(long)]
        dry_run: bool,
    },
}

#[derive(Debug, Clone, Args)]
pub struct NewArgs {
    pub title: Option<String>,
    #[arg(long = "type", value_name = "TASK_TYPE")]
    pub task_type: Option<String>,
    #[arg(long, value_name = "TASK_REF")]
    pub parent: Option<String>,
    #[arg(long, value_name = "PRIORITY")]
    pub priority: Option<String>,
    #[arg(long, value_name = "TAG", value_delimiter = ',')]
    pub tags: Vec<String>,
    #[arg(long)]
    pub no_edit: bool,
}

#[derive(Debug, Clone, Args)]
pub struct ShowArgs {
    pub task_ref: String,
    #[arg(long)]
    pub instructions: bool,
    #[arg(long)]
    pub declaration: bool,
}

impl Command {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Init { .. } => "init",
            Self::New(_) => "new",
            Self::List(_) => "list",
            Self::Tree(_) => "tree",
            Self::Show(_) => "show",
            Self::Instruction { .. } => "instruction",
            Self::Declaration { .. } => "declaration",
            Self::Status(_) => "status",
            Self::Rebuild { .. } => "rebuild",
        }
    }
}
