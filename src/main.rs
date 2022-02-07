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

use anyhow::{Error, Result};
use iced::{window, Application, Settings};

mod app;
mod interaction;
mod style;
mod view;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const TITLE: &str = "TSV Converter";

fn main() -> Result<()> {
    let settings: Settings<()> = Settings {
        id: Some(NAME.to_owned()),
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

    app::TSVConverter::run(settings).map_err(Error::new)
}
