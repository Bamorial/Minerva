use minerva_application::BootstrapService;

#[must_use]
pub fn compile_workspace_context() -> String {
    let crates = BootstrapService::workspace_blueprint().crates().join(", ");
    let interfaces = BootstrapService::interface_descriptions()
        .map(|item| item.crate_name)
        .join(", ");
    format!("Workspace crates: {crates}\nInterfaces: {interfaces}")
}

#[cfg(test)]
mod tests {
    use super::compile_workspace_context;

    #[test]
    fn context_compilation_is_deterministic() {
        let context = compile_workspace_context();
        assert!(context.contains("minerva-domain"));
        assert!(context.contains("minerva-mcp"));
    }
}
