fn main() {
    let crates =
        minerva_application::BootstrapService::workspace_blueprint().crates().len();
    let context = minerva_context::compile_workspace_context();
    println!("minerva-tui ready for {crates} workspace crates.\n{context}");
}
