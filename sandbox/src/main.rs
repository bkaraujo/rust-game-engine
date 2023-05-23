use engine::logger;
use engine::application::Application;

fn main() {
    Application::new()
        .initializer(&initialize)
        .updater(&update)
        .terminator(&terminate)
        .run();
}

fn initialize() {
    logger::debug("initialize()");
}

fn terminate() {
    logger::debug("terminate()");
}

fn update() {
    logger::debug("update()");
}
