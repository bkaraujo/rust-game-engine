use crate::{AspectRatio, Resolution};

pub mod console;

#[cfg(target_os = "windows")]
pub mod windows;

pub struct Monitor {
    pub width: i32,
    pub height: i32,
}

pub struct Platform {
    handle: isize,
    pub monitor: Monitor
}

pub struct Window {
    handle: isize,
    pub title: String,
    pub resolution: Resolution,
    pub ratio: AspectRatio,

    pub width: i32,
    pub height: i32,
}