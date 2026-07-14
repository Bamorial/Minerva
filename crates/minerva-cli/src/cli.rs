use crate::tree_args::TreeArgs;
use crate::{list_args::ListArgs, status_args::StatusArgs, task_ref_arg::TaskRefArg};
use clap::{ArgAction, Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

const HELP: &str = "Exit codes:\n  0 success\n  1 internal failure\n  2 command usage error\n  10 project not initialized\n  11 project already initialized\n  12 task not found\n  13 ambiguous task reference\n  14 invalid status transition\n  15 hierarchy cycle\n  16 dependency cycle\n  17 schema error\n  18 version conflict\n  19 lock conflict\n  20 invalid configuration\n  21 editor launch failure\n  22 rebuild validation failure\n  23 validation warning\n  24 validation error";

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
    Context(ContextArgs),
    Log(LogArgs),
    Instruction {
        task_ref: Option<String>,
    },
    Declaration {
        task_ref: String,
    },
    Status(StatusArgs),
    Complete(TaskRefArg),
    Reopen(TaskRefArg),
    Move(MoveArgs),
    Depend(DependArgs),
    Undepend(DependArgs),
    Relate(RelateArgs),
    Unrelate(UnrelateArgs),
    Children(TaskRefArg),
    Ancestors(TaskRefArg),
    Repair {
        #[arg(long)]
        dry_run: bool,
    },
    Migrate {
        #[arg(long)]
        dry_run: bool,
    },
    Rebuild {
        #[arg(long)]
        dry_run: bool,
    },
    Validate(ValidateArgs),
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

#[derive(Debug, Clone, Args)]
pub struct ContextArgs {
    pub task_ref: String,
    #[arg(long, value_name = "PATH")]
    pub output: Option<PathBuf>,
    #[arg(long, value_name = "FORMAT", default_value = "markdown")]
    pub format: ContextFormatArg,
    #[arg(long, value_name = "TOKENS")]
    pub budget: Option<usize>,
    #[arg(long)]
    pub explain: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ContextFormatArg {
    Markdown,
    Json,
}

#[derive(Debug, Clone, Args)]
pub struct LogArgs {
    pub task_ref: String,
    #[arg(long = "kind", value_name = "EVENT_KIND", value_delimiter = ',')]
    pub kinds: Vec<LogKindArg>,
}

impl LogArgs {
    #[must_use]
    pub fn kinds(&self) -> Vec<minerva_domain::TaskEventKind> {
        self.kinds.iter().copied().map(Into::into).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum LogKindArg {
    #[value(name = "task-created", alias = "created")]
    Created,
    #[value(name = "task-declaration-updated", alias = "declaration-updated")]
    DeclarationUpdated,
    #[value(name = "task-instructions-updated", alias = "instructions-updated")]
    InstructionsUpdated,
    #[value(name = "task-parent-changed", alias = "parent-changed")]
    ParentChanged,
    #[value(name = "task-status-changed", alias = "status-changed")]
    StatusChanged,
    #[value(name = "task-relationship-added", alias = "relationship-added")]
    RelationshipAdded,
    #[value(name = "task-relationship-removed", alias = "relationship-removed")]
    RelationshipRemoved,
    #[value(name = "task-archived", alias = "archived")]
    Archived,
}

impl From<LogKindArg> for minerva_domain::TaskEventKind {
    fn from(value: LogKindArg) -> Self {
        match value {
            LogKindArg::Created => Self::TaskCreated,
            LogKindArg::DeclarationUpdated => Self::TaskDeclarationUpdated,
            LogKindArg::InstructionsUpdated => Self::TaskInstructionsUpdated,
            LogKindArg::ParentChanged => Self::TaskParentChanged,
            LogKindArg::StatusChanged => Self::TaskStatusChanged,
            LogKindArg::RelationshipAdded => Self::TaskRelationshipAdded,
            LogKindArg::RelationshipRemoved => Self::TaskRelationshipRemoved,
            LogKindArg::Archived => Self::TaskArchived,
        }
    }
}

#[derive(Debug, Clone, Args)]
pub struct MoveArgs {
    pub task_ref: String,
    #[arg(long, value_name = "TASK_REF", required_unless_present = "to_root")]
    pub parent: Option<String>,
    #[arg(long = "to-root", conflicts_with = "parent")]
    pub to_root: bool,
}

#[derive(Debug, Clone, Args)]
pub struct DependArgs {
    pub task_ref: String,
    pub depends_on_ref: String,
    #[arg(long)]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Args)]
pub struct RelateArgs {
    pub source_ref: String,
    pub target_ref: String,
    pub relationship_type: String,
    #[arg(long)]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Args)]
pub struct UnrelateArgs {
    pub source_ref: String,
    pub target_ref: String,
    pub relationship_type: String,
}

#[derive(Debug, Clone, Args)]
pub struct ValidateArgs {
    pub task_ref: Option<String>,
}

impl Command {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Init { .. } => "init",
            Self::New(_) => "new",
            Self::List(_) => "list",
            Self::Tree(_) => "tree",
            Self::Show(_) => "show",
            Self::Context(_) => "context",
            Self::Log(_) => "log",
            Self::Instruction { .. } => "instruction",
            Self::Declaration { .. } => "declaration",
            Self::Status(_) => "status",
            Self::Complete(_) => "complete",
            Self::Reopen(_) => "reopen",
            Self::Move(_) => "move",
            Self::Depend(_) => "depend",
            Self::Undepend(_) => "undepend",
            Self::Relate(_) => "relate",
            Self::Unrelate(_) => "unrelate",
            Self::Children(_) => "children",
            Self::Ancestors(_) => "ancestors",
            Self::Repair { .. } => "repair",
            Self::Migrate { .. } => "migrate",
            Self::Rebuild { .. } => "rebuild",
            Self::Validate(_) => "validate",
        }
    }
}
