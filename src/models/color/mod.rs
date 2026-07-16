use std::num::ParseIntError;

#[derive(Clone)]
pub struct Color(u32);

impl Color {
    pub fn from_u32(value: u32) -> Self {
        Self(value)
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self(((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    pub fn from_hex(hex: &str) -> Result<Self, ParseIntError> {
        let hex = hex.trim_start_matches("#");
        Ok(Self(u32::from_str_radix(hex, 16)?))
    }
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        self.0
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Self(value)
    }
}