use std::borrow::Borrow;

const LEVEL: [&str; 6] = ["TRACE",      "DEBUG",      "INFO ",       "WARN ",       "ERROR",      "FATAL"];
const COLOR: [&str; 6] = ["\x1b[0;35m", "\x1b[0;34m", "\x1b[0;32m", "\x1b[0;33m", "\x1b[0;31m", "\x1b[1;91m"];

pub fn trace(message: &str) { log(0, message); }
pub fn debug(message: &str) { log(1, message); }
pub fn info (message: &str) { log(2, message); }
pub fn warn (message: &str) { log(3, message); }
pub fn error(message: &str) { log(4, message); }
pub fn fatal(message: &str) { log(5, message); }

fn log(level: usize, message: &str) {
    let now = crate::platform::time::now();
    crate::platform::console::write(
        format!("{}{}-{}-{} {}:{}:{},{} {} - {}\x1b[0m",
                COLOR[level],
                now.year, now.month, now.day,
                now.hour, now.minute, now.second, now.millis,
                LEVEL[level],
                message
        ).borrow()
    );
}