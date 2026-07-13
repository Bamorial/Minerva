use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum IdentifierError {
    #[error("invalid `{kind}` identifier prefix: expected `{expected}`")]
    InvalidPrefix { kind: &'static str, expected: &'static str },
    #[error("invalid `{kind}` identifier body: {reason}")]
    InvalidBody { kind: &'static str, reason: &'static str },
}
