use crate::{
    MinervaLayout,
    task_markdown::{read_optional_markdown, write_markdown},
};
use minerva_domain::MinervaError;

pub fn read_project_instructions(
    layout: &MinervaLayout,
) -> Result<String, MinervaError> {
    read_optional_markdown(&layout.instructions_file())
}

pub fn write_project_instructions(
    layout: &MinervaLayout,
    contents: &str,
) -> Result<(), MinervaError> {
    write_markdown(&layout.instructions_file(), contents)
}
