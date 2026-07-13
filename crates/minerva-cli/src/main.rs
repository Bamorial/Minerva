fn main() {
    let interface_count =
        minerva_application::BootstrapService::interface_descriptions().len();
    let context = minerva_context::compile_workspace_context();
    println!("minerva-cli ready with {interface_count} interfaces.\n{context}");
}
