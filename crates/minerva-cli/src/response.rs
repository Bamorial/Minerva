use serde_json::Value;

#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub text: String,
    pub json: Option<Value>,
}

impl CommandOutput {
    pub fn text(text: impl Into<String>) -> Self {
        Self { text: text.into(), json: None }
    }

    pub fn with_json(text: impl Into<String>, json: Value) -> Self {
        Self { text: text.into(), json: Some(json) }
    }
}
