use vector::app::Application;

mod transform;

fn main() {
    let app = Application::prepare().unwrap_or_else(|code| {
        std::process::exit(code);
    });

    app.run();
}
