use minerva_application::{BootstrapService, render_cli, render_mcp, render_tui};
use minerva_domain::MinervaError;

#[test]
fn interfaces_are_reported_without_embedding_business_rules() {
    let crates = BootstrapService::workspace_blueprint().crates().len();
    let interfaces = BootstrapService::interface_descriptions();
    assert_eq!(crates, 7);
    assert_eq!(
        interfaces.map(|item| item.crate_name),
        ["minerva-cli", "minerva-tui", "minerva-mcp",]
    );
}

#[test]
fn interface_error_renderers_share_one_domain_error() {
    let error = MinervaError::InvalidConfiguration {
        key: "editor".into(),
        reason: "empty".into(),
    };
    let cli = render_cli(&error);
    let tui = render_tui(&error);
    let mcp = render_mcp(&error);
    assert_eq!(cli.code, "invalid_configuration");
    assert_eq!(tui.title, "Invalid configuration");
    assert_eq!(mcp.data.minerva_code, "invalid_configuration");
}
