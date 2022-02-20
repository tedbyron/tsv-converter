//! User interaction event handlers.

use std::fs::File;
use std::path::PathBuf;

use anyhow::{Error, Result};
use native_dialog::FileDialog;

// mod output {
//     pub(super) const WIDTH: i32 = 96;
//     pub(super) const HEIGHT: i32 = 64;
//     pub(super) const FRAMERATE: i32 = 30;
//     pub(super) const AUDIO_SAMPLE_BIT_DEPTH: i32 = 10;
//     pub(super) const AUDIO_SAMPLE_COUNT_PER_FRAME: i32 = 2 * 512;
//     pub(super) const AUDIO_SAMPLE_RATE: i32 = FRAMERATE * AUDIO_SAMPLE_COUNT_PER_FRAME;
// }

/// Open a file dialog, prompting for video files.
pub fn select_file() -> Result<Option<PathBuf>> {
    FileDialog::new()
        .add_filter("Video files", &["mp4", "mov", "mpg", "mpeg", "avi", "gif"])
        .show_open_single_file()
        .map_err(Error::new)
}

/// Load a preview frame of the video with the current settings applied.
pub fn preview_frame(file: File) -> Result<()> {
    Ok(())
}
