use engine;

fn main() {
    engine::logger::trace("trace");
    engine::logger::debug("debug");
    engine::logger::info("info");
    engine::logger::warn("warn");
    engine::logger::error("error");
    engine::logger::fatal("fatal");
}
