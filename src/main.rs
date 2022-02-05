#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    rust_2018_idioms
)]
#![windows_subsystem = "windows"]
#![doc = include_str!("../README.md")]

use iced::{window, Application, Settings};

mod app;
mod font;
mod style;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_TITLE: &str = "TSV Converter";

mod color {
    use iced::Color;

    lazy_static::lazy_static! {
        // pub static ref BG_DARKER:   Color = Color::from_rgb8(0x0B, 0x0B, 0x0F);
        // pub static ref BG_DARK:     Color = Color::from_rgb8(0x17, 0x16, 0x1D);
        pub static ref BG:          Color = Color::from_rgb8(0x22, 0x21, 0x2C);
        pub static ref BG_LIGHT:    Color = Color::from_rgb8(0x2E, 0x2B, 0x3B);
        // pub static ref BG_LIGHTER:  Color = Color::from_rgb8(0x39, 0x36, 0x49);

        // pub static ref GRAY:        Color = Color::from_rgb8(0x42, 0x44, 0x50);
        pub static ref GRAY_PURPLE: Color = Color::from_rgb8(0x45, 0x41, 0x58);
        // pub static ref PURPLE_GRAY: Color = Color::from_rgb8(0x79, 0x70, 0xA9);

        pub static ref FG:          Color = Color::from_rgb8(0xF8, 0xF8, 0xF2);

        pub static ref ORANGE:      Color = Color::from_rgb8(0xFF, 0xCA, 0x80);
    }
}

fn main() -> anyhow::Result<()> {
    let settings: Settings<()> = Settings {
        id: APP_NAME.to_owned().into(),
        window: window::Settings {
            // size: (1024, 768), // TODO: window size
            // min_size: None, // TODO: window size min
            resizable: false,
            // icon: None, // TODO: window icon
            ..window::Settings::default()
        },
        flags: (),
        text_multithreading: true,
        antialiasing: true,
        ..Settings::default()
    };

    app::TSVConverter::run(settings).map_err(anyhow::Error::new)
}
