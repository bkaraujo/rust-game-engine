use engine::logger;
use engine::application::Application;

fn main() {
    Application::new()
        .initializer(&initialize)
        .updater(&update)
        .terminator(&terminator)
        .run();
}

fn initialize() {
    logger::debug("Initializing");
}

fn terminator() {
    logger::debug("terminator()");
}

fn update() {
    logger::debug("update()");
}