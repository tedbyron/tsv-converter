//! Color definitions.

use iced::Color;

lazy_static::lazy_static! {
    pub static ref BG_DARKER:   Color = Color::from_rgb8(0x0B, 0x0B, 0x0F);
    pub static ref BG_DARK:     Color = Color::from_rgb8(0x17, 0x16, 0x1D);
    pub static ref BG:          Color = Color::from_rgb8(0x22, 0x21, 0x2C);
    pub static ref BG_LIGHT:    Color = Color::from_rgb8(0x2E, 0x2B, 0x3B);
    pub static ref BG_LIGHTER:  Color = Color::from_rgb8(0x39, 0x36, 0x49);

    pub static ref FG:          Color = Color::from_rgb8(0xF8, 0xF8, 0xF2);

    pub static ref GRAY:        Color = Color::from_rgb8(0x42, 0x44, 0x50);
    pub static ref GRAY_PURPLE: Color = Color::from_rgb8(0x45, 0x41, 0x58);
    pub static ref PURPLE_GRAY: Color = Color::from_rgb8(0x79, 0x70, 0xA9);

    // pub static ref GREEN:       Color = Color::from_rgb8(0x8A, 0xFF, 0x80);
    pub static ref ORANGE:      Color = Color::from_rgb8(0xFF, 0xCA, 0x80);
    // pub static ref PINK:        Color = Color::from_rgb8(0xFF, 0x80, 0xBF);
    pub static ref PURPLE:      Color = Color::from_rgb8(0x95, 0x80, 0xFF);
    pub static ref RED:         Color = Color::from_rgb8(0xFF, 0x95, 0x80);
}
