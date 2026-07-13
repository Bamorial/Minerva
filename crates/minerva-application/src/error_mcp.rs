use minerva_domain::{ErrorDetail, ErrorValue, MinervaError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct McpErrorResponse {
    pub json_rpc_code: i32,
    pub message: String,
    pub data: McpErrorData,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct McpErrorData {
    pub minerva_code: &'static str,
    pub details: Vec<(String, String)>,
}

pub fn render_mcp(error: &MinervaError) -> McpErrorResponse {
    McpErrorResponse {
        json_rpc_code: -32000,
        message: error.to_string(),
        data: McpErrorData {
            minerva_code: error.code().as_str(),
            details: error.details().into_iter().map(format_detail).collect(),
        },
    }
}

fn format_detail(detail: ErrorDetail) -> (String, String) {
    let value = match detail.value {
        ErrorValue::Text(value) => value,
        ErrorValue::List(value) => value.join(", "),
    };
    (detail.key.to_string(), value)
}
