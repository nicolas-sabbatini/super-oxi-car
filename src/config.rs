#![allow(clippy::cast_precision_loss, dead_code)]
use bevy::prelude::Color;

pub const P8_BLACK: Color = Color::Rgba {
    red: 0x00 as f32 / 255.0,
    green: 0x00 as f32 / 255.0,
    blue: 0x00 as f32 / 255.0,
    alpha: 1.0,
};
pub const P8_DARK_BLUE: Color = Color::Rgba {
    red: 0x1D as f32 / 255.0,
    green: 0x2B as f32 / 255.0,
    blue: 0x53 as f32 / 255.0,
    alpha: 1.0,
};
pub const P8_WINE: Color = Color::Rgba {
    red: 0x7E as f32 / 255.0,
    green: 0x25 as f32 / 255.0,
    blue: 0x53 as f32 / 255.0,
    alpha: 1.0,
};
pub const P8_DARK_GREEN: Color = Color::Rgba {
    red: 0x00 as f32 / 255.0,
    green: 0x87 as f32 / 255.0,
    blue: 0x51 as f32 / 255.0,
    alpha: 1.0,
};
pub const P8_BROWN: Color = Color::Rgba {
    red: 0xAB as f32 / 255.0,
    green: 0x52 as f32 / 255.0,
    blue: 0x36 as f32 / 255.0,
    alpha: 1.0,
};
pub const P8_DARK_GREY: Color = Color::Rgba {
    red: 0x5F as f32 / 255.0,
    green: 0x57 as f32 / 255.0,
    blue: 0x4F as f32 / 255.0,
    alpha: 1.0,
};
pub const P8_LIGHT_GREY: Color = Color::Rgba {
    red: 0xC2 as f32 / 255.0,
    green: 0xC3 as f32 / 255.0,
    blue: 0xC7 as f32 / 255.0,
    alpha: 1.0,
};
pub const P8_WHITE: Color = Color::Rgba {
    red: 0xFF as f32 / 255.0,
    green: 0xF1 as f32 / 255.0,
    blue: 0xE8 as f32 / 255.0,
    alpha: 1.0,
};
pub const P8_RED: Color = Color::Rgba {
    red: 0xFF as f32 / 255.0,
    green: 0x00 as f32 / 255.0,
    blue: 0x4D as f32 / 255.0,
    alpha: 0.5,
};
pub const P8_ORANGE: Color = Color::Rgba {
    red: 0xFF as f32 / 255.0,
    green: 0xA3 as f32 / 255.0,
    blue: 0x00 as f32 / 255.0,
    alpha: 1.0,
};
pub const P8_YELLOW: Color = Color::Rgba {
    red: 0xFF as f32 / 255.0,
    green: 0xEC as f32 / 255.0,
    blue: 0x27 as f32 / 255.0,
    alpha: 1.0,
};
pub const P8_LIGHT_GREEN: Color = Color::Rgba {
    red: 0x00 as f32 / 255.0,
    green: 0xE4 as f32 / 255.0,
    blue: 0x36 as f32 / 255.0,
    alpha: 1.0,
};
pub const P8_LIGHT_BLUE: Color = Color::Rgba {
    red: 0x29 as f32 / 255.0,
    green: 0xAD as f32 / 255.0,
    blue: 0xFF as f32 / 255.0,
    alpha: 1.0,
};
pub const P8_GREY: Color = Color::Rgba {
    red: 0x83 as f32 / 255.0,
    green: 0x76 as f32 / 255.0,
    blue: 0x9C as f32 / 255.0,
    alpha: 1.0,
};
pub const P8_PINK: Color = Color::Rgba {
    red: 0xFF as f32 / 255.0,
    green: 0x77 as f32 / 255.0,
    blue: 0xA8 as f32 / 255.0,
    alpha: 1.0,
};
pub const P8_CREAM: Color = Color::Rgba {
    red: 0xFF as f32 / 255.0,
    green: 0xCC as f32 / 255.0,
    blue: 0xAA as f32 / 255.0,
    alpha: 1.0,
};

// Screen config
pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;
pub const WINDOW_TITLE: &str = "Super Oxi Car!";

pub const WINDOW_CAMERA_NAME: &str = "windows camera";
pub const WINDOW_CAMERA_CLEAR_COLOR: Color = P8_BLACK;

pub const GAME_CAMERA_NAME: &str = "game camera";
pub const GAME_CAMERA_TARGET_NAME: &str = "game camera target";
pub const GAME_CAMERA_CLEAR_COLOR: Color = P8_CREAM;
