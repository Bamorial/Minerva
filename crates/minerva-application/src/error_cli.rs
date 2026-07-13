use minerva_domain::{ErrorDetail, ErrorValue, MinervaError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliErrorReport {
    pub code: &'static str,
    pub message: String,
    pub details: Vec<String>,
}

pub fn render_cli(error: &MinervaError) -> CliErrorReport {
    CliErrorReport {
        code: error.code().as_str(),
        message: error.to_string(),
        details: error.details().into_iter().map(format_detail).collect(),
    }
}

fn format_detail(detail: ErrorDetail) -> String {
    match detail.value {
        ErrorValue::Text(value) => format!("{}: {value}", detail.key),
        ErrorValue::List(value) => format!("{}: {}", detail.key, value.join(", ")),
    }
}
