#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorDetail {
    pub key: &'static str,
    pub value: ErrorValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorValue {
    Text(String),
    List(Vec<String>),
}

impl ErrorDetail {
    #[must_use]
    pub fn text(key: &'static str, value: impl Into<String>) -> Self {
        Self { key, value: ErrorValue::Text(value.into()) }
    }

    #[must_use]
    pub fn list(key: &'static str, value: Vec<String>) -> Self {
        Self { key, value: ErrorValue::List(value) }
    }
}
