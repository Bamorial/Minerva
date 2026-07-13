fn main() {
    let interfaces = minerva_application::BootstrapService::interface_descriptions();
    let context = minerva_context::compile_workspace_context();
    println!(
        "minerva-mcp ready with {} interface contracts.\n{context}",
        interfaces.len()
    );
}
