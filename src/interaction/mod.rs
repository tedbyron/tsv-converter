//! User interaction event handlers.

use std::path::PathBuf;

use anyhow::{Error, Result};
use native_dialog::FileDialog;

pub fn select_file() -> Result<Option<PathBuf>> {
    FileDialog::new()
        .add_filter("Video files", &["mp4", "mov", "mpg", "mpeg", "avi", "gif"])
        .show_open_single_file()
        .map_err(Error::new)
}
