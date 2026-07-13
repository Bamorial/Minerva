use crate::{
    MinervaLayout,
    config_document::ProjectConfigDocument,
    project_document::ProjectDocument,
    yaml_codec::{read_yaml, write_yaml},
};
use minerva_domain::{MinervaError, Project, ProjectConfig};

pub fn read_project(layout: &MinervaLayout) -> Result<Project, MinervaError> {
    let path = layout.project_file();
    read_yaml::<ProjectDocument>(&path)?.try_into().map_err(|err| schema(&path, err))
}

pub fn write_project(
    layout: &MinervaLayout,
    project: &Project,
) -> Result<(), MinervaError> {
    project.validate().map_err(|err| schema(&layout.project_file(), err))?;
    write_yaml(&layout.project_file(), &ProjectDocument::from(project))
}

pub fn read_project_config(
    layout: &MinervaLayout,
) -> Result<ProjectConfig, MinervaError> {
    let path = layout.config_file();
    read_yaml::<ProjectConfigDocument>(&path)?
        .try_into()
        .map_err(|err| schema(&path, err))
}

pub fn write_project_config(
    layout: &MinervaLayout,
    config: &ProjectConfig,
) -> Result<(), MinervaError> {
    config.validate().map_err(|err| schema(&layout.config_file(), err))?;
    write_yaml(&layout.config_file(), &ProjectConfigDocument::from(config))
}

fn schema(path: &std::path::Path, err: MinervaError) -> MinervaError {
    let reason = match err {
        MinervaError::InvalidConfiguration { key, reason } => {
            format!("{key}: {reason}")
        }
        MinervaError::SchemaError { reason, .. } => reason,
        other => other.to_string(),
    };
    MinervaError::SchemaError { path: path.display().to_string(), reason }
}
