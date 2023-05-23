mod event;
mod platform;
pub mod logger;
pub mod application;
pub mod time;

pub enum Resolution {
    HD = 720,
    FHD = 1024,
    QHD = 1440,
    UHD = 2160
}

pub enum AspectRatio {
    A4x3,
    A16x9,
    A21x9
}

pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8
}

impl Color {
    pub fn from(red: u8, green: u8, blue: u8, alpha: u8) -> Self { Self {red, green, blue, alpha } }
    pub fn packed(red: u8, green: u8, blue: u8, alpha: u8) -> u32 { unsafe { std::mem::transmute::<[u8; 4], u32>([red, green, blue, alpha]) } }
    pub fn as_us32(&self) -> u32 { Color::packed(self.red, self.green, self.blue, self.alpha) }
}