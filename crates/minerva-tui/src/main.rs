use minerva_application::render_tui;

fn main() {
    if let Err(error) = minerva_tui::run() {
        let message = render_tui(&error);
        eprintln!("{}\n{}", message.title, message.body);
        std::process::exit(1);
    }
}
