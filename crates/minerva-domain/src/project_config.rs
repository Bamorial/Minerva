use crate::MinervaError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectConfig {
    pub schema_version: u32,
    pub editor: Option<String>,
}

impl ProjectConfig {
    pub fn new(config: Self) -> Result<Self, MinervaError> {
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), MinervaError> {
        if self.schema_version == 0 {
            return invalid("schema_version", "must be greater than zero");
        }
        if let Some(editor) = &self.editor
            && editor.trim().is_empty()
        {
            return invalid("editor", "must not be empty when present");
        }
        Ok(())
    }
}

fn invalid<T>(key: &str, reason: &str) -> Result<T, MinervaError> {
    Err(MinervaError::InvalidConfiguration { key: key.into(), reason: reason.into() })
}
